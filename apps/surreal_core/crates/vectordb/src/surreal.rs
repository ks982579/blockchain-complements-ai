//! SurrealDB (embedded SurrealKV) implementation of [`VectorStore`].
//!
//! This is the only module in the crate that imports `surrealdb`. Everything
//! outside this module talks to the [`crate::VectorStore`] trait.

use crate::{Chunk, Paper, QueryResult, RelatedPaper, RelationKind, VectorStore};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use surrealdb::engine::local::{Db, SurrealKv};
use surrealdb::{RecordId, Surreal};

const NAMESPACE: &str = "research";
const DATABASE: &str = "papers";
const EMBEDDING_DIM: usize = 384;

/// Extract the raw string key of a `paper` record ID (e.g. `"paper:foo"` ->
/// `"foo"`), without the surql escaping that `RecordIdKey::to_string()` adds
/// for strings containing special characters like `-`.
fn record_id_to_string(id: &RecordId) -> String {
    String::try_from(id.key().clone()).expect("paper record IDs are always strings")
}

pub struct SurrealStore {
    db: Surreal<Db>,
}

impl SurrealStore {
    pub async fn connect(path: &str) -> Result<Self> {
        let db = Surreal::new::<SurrealKv>(path).await?;
        db.use_ns(NAMESPACE).use_db(DATABASE).await?;
        Ok(Self { db })
    }
}

/// Row shape returned by `query()` KNN search before joining paper metadata.
#[derive(Debug, Deserialize)]
struct ChunkRow {
    paper: RecordId,
    section: String,
    text: String,
    distance: f32,
}

/// Row shape for a paper record fetched from SurrealDB.
#[derive(Debug, Deserialize)]
struct PaperRow {
    id: RecordId,
    title: String,
    authors: Vec<String>,
    year: Option<i64>,
    venue: Option<String>,
    doi: Option<String>,
    url: Option<String>,
    keywords: Vec<String>,
    extra: serde_json::Value,
}

impl From<PaperRow> for Paper {
    fn from(row: PaperRow) -> Self {
        Paper {
            id: record_id_to_string(&row.id),
            title: row.title,
            authors: row.authors,
            year: row.year,
            venue: row.venue,
            doi: row.doi,
            url: row.url,
            keywords: row.keywords,
            extra: row.extra,
        }
    }
}

#[async_trait]
impl VectorStore for SurrealStore {
    async fn init_schema(&self) -> Result<()> {
        let setup = format!(
            "
            DEFINE TABLE IF NOT EXISTS paper SCHEMALESS;
            DEFINE TABLE IF NOT EXISTS chunk SCHEMALESS;
            DEFINE FIELD IF NOT EXISTS embedding ON chunk TYPE array<float>;
            DEFINE INDEX IF NOT EXISTS chunk_embedding_idx ON chunk
                FIELDS embedding HNSW DIMENSION {EMBEDDING_DIM} DIST COSINE;
            DEFINE TABLE IF NOT EXISTS cites TYPE RELATION IN paper OUT paper SCHEMALESS;
            DEFINE TABLE IF NOT EXISTS same_topic TYPE RELATION IN paper OUT paper SCHEMALESS;
            "
        );
        self.db.query(setup).await?.check()?;
        Ok(())
    }

    async fn upsert_paper(&self, paper: &Paper, chunks: &[Chunk]) -> Result<()> {
        let paper_rid = RecordId::from(("paper", paper.id.as_str()));

        // Replace paper record.
        self.db
            .query("UPSERT $rid CONTENT $data")
            .bind(("rid", paper_rid.clone()))
            .bind((
                "data",
                serde_json::json!({
                    "title": paper.title,
                    "authors": paper.authors,
                    "year": paper.year,
                    "venue": paper.venue,
                    "doi": paper.doi,
                    "url": paper.url,
                    "keywords": paper.keywords,
                    "extra": paper.extra,
                }),
            ))
            .await?
            .check()?;

        // Drop existing chunks for this paper, then insert the new set.
        self.db
            .query("DELETE chunk WHERE paper = $rid")
            .bind(("rid", paper_rid.clone()))
            .await?
            .check()?;

        for chunk in chunks {
            self.db
                .query("CREATE chunk SET paper = $rid, section = $section, text = $text, embedding = $embedding")
                .bind(("rid", paper_rid.clone()))
                .bind(("section", chunk.section.clone()))
                .bind(("text", chunk.text.clone()))
                .bind(("embedding", chunk.embedding.clone()))
                .await?
                .check()?;
        }

        Ok(())
    }

    async fn existing_paper_ids(&self) -> Result<Vec<String>> {
        #[derive(Deserialize)]
        struct IdRow {
            id: RecordId,
        }
        let ids: Vec<IdRow> = self.db.query("SELECT id FROM paper").await?.take(0)?;
        Ok(ids.into_iter().map(|r| record_id_to_string(&r.id)).collect())
    }

    async fn set_relations(
        &self,
        paper_id: &str,
        edges: &[(String, RelationKind)],
    ) -> Result<()> {
        let from = RecordId::from(("paper", paper_id));

        self.db
            .query("DELETE cites WHERE in = $from; DELETE same_topic WHERE in = $from;")
            .bind(("from", from.clone()))
            .await?
            .check()?;

        for (target_id, kind) in edges {
            let table = match kind {
                RelationKind::Cites => "cites",
                RelationKind::SameTopic => "same_topic",
            };
            let to = RecordId::from(("paper", target_id.as_str()));
            self.db
                .query(format!("RELATE $from->{table}->$to"))
                .bind(("from", from.clone()))
                .bind(("to", to))
                .await?
                .check()?;
        }

        Ok(())
    }

    async fn query(&self, embedding: &[f32], top_k: usize) -> Result<Vec<QueryResult>> {
        // The KNN operator `<|K,COSINE|>` requires K as a literal, not a bound
        // parameter; `top_k` is a usize from our own CLI args, not user SQL.
        let rows: Vec<ChunkRow> = self
            .db
            .query(format!(
                "SELECT paper, section, text,
                        vector::distance::knn() AS distance
                 FROM chunk
                 WHERE embedding <|{top_k},COSINE|> $embedding
                 ORDER BY distance"
            ))
            .bind(("embedding", embedding.to_vec()))
            .await?
            .take(0)?;

        let mut results = Vec::with_capacity(rows.len());
        for row in rows {
            let paper_row: Option<PaperRow> = self.db.select(row.paper.clone()).await?;
            let Some(paper_row) = paper_row else {
                continue;
            };
            results.push(QueryResult {
                paper: paper_row.into(),
                section: row.section,
                text: row.text,
                distance: row.distance,
            });
        }

        Ok(results)
    }

    async fn related(&self, paper_id: &str, depth: u32) -> Result<Vec<RelatedPaper>> {
        let start = RecordId::from(("paper", paper_id));
        let mut frontier = vec![start.clone()];
        let mut seen = std::collections::HashSet::new();
        seen.insert(start.clone());

        #[derive(Deserialize)]
        struct EdgeRow {
            paper: PaperRow,
            relation: String,
        }

        let mut results = Vec::new();

        for hop in 1..=depth {
            let mut next_frontier = Vec::new();

            for node in &frontier {
                let mut response = self
                    .db
                    .query("SELECT out.* AS paper, 'cites' AS relation FROM cites WHERE in = $node")
                    .query("SELECT out.* AS paper, 'same_topic' AS relation FROM same_topic WHERE in = $node")
                    .bind(("node", node.clone()))
                    .await?;
                let cites: Vec<EdgeRow> = response.take(0)?;
                let same_topic: Vec<EdgeRow> = response.take(1)?;
                let rows = cites.into_iter().chain(same_topic);

                for row in rows {
                    let rid = RecordId::from(("paper", record_id_to_string(&row.paper.id).as_str()));
                    let relation = match row.relation.as_str() {
                        "cites" => RelationKind::Cites,
                        _ => RelationKind::SameTopic,
                    };
                    // A node may be reached via multiple edge kinds; report each
                    // kind once, but only expand traversal from it the first time.
                    let first_visit = seen.insert(rid.clone());
                    results.push(RelatedPaper {
                        paper: row.paper.into(),
                        relation,
                        depth: hop,
                    });
                    if first_visit {
                        next_frontier.push(rid);
                    }
                }
            }

            if next_frontier.is_empty() {
                break;
            }
            frontier = next_frontier;
        }

        Ok(results)
    }

    async fn close(&self) -> Result<()> {
        // Surreal<Db> has no explicit close; dropping the handle releases the
        // embedded SurrealKv file lock.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Chunk, RelationKind};

    /// Build a deterministic 384-dim embedding from a seed, varying enough
    /// per seed that KNN ordering is meaningful in tests.
    fn fake_embedding(seed: u32) -> Vec<f32> {
        (0..EMBEDDING_DIM)
            .map(|i| ((seed as f32) + i as f32).sin())
            .collect()
    }

    async fn temp_store() -> (SurrealStore, tempfile::TempDir) {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join("test.db");
        let store = SurrealStore::connect(path.to_str().unwrap())
            .await
            .expect("connect to temp store");
        store.init_schema().await.expect("init schema");
        (store, dir)
    }

    fn sample_paper(id: &str, keywords: &[&str]) -> Paper {
        Paper {
            id: id.to_string(),
            title: format!("Title for {id}"),
            authors: vec!["Author A".to_string()],
            year: Some(2024),
            venue: Some("Venue".to_string()),
            doi: Some(format!("10.0000/{id}")),
            url: Some(format!("https://example.com/{id}")),
            keywords: keywords.iter().map(|k| k.to_string()).collect(),
            extra: serde_json::json!({}),
        }
    }

    #[tokio::test]
    async fn upsert_and_query_roundtrip() {
        let (store, _dir) = temp_store().await;

        let paper = sample_paper("paper-a", &["blockchain", "ai"]);
        let chunks = vec![
            Chunk {
                paper_id: paper.id.clone(),
                section: "Overview".to_string(),
                text: "An overview of paper A.".to_string(),
                embedding: fake_embedding(1),
            },
            Chunk {
                paper_id: paper.id.clone(),
                section: "Background".to_string(),
                text: "Background of paper A.".to_string(),
                embedding: fake_embedding(2),
            },
        ];

        store.upsert_paper(&paper, &chunks).await.unwrap();

        let results = store.query(&fake_embedding(1), 5).await.unwrap();
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.paper.id == "paper-a"));
        // The chunk with the matching embedding should be the closest match.
        assert_eq!(results[0].section, "Overview");
        assert_eq!(results[0].paper.title, "Title for paper-a");
    }

    #[tokio::test]
    async fn upsert_replaces_existing_chunks() {
        let (store, _dir) = temp_store().await;

        let paper = sample_paper("paper-a", &["blockchain"]);
        let first_chunks = vec![Chunk {
            paper_id: paper.id.clone(),
            section: "Overview".to_string(),
            text: "First version.".to_string(),
            embedding: fake_embedding(1),
        }];
        store.upsert_paper(&paper, &first_chunks).await.unwrap();

        let second_chunks = vec![Chunk {
            paper_id: paper.id.clone(),
            section: "Overview".to_string(),
            text: "Second version.".to_string(),
            embedding: fake_embedding(1),
        }];
        store.upsert_paper(&paper, &second_chunks).await.unwrap();

        let results = store.query(&fake_embedding(1), 10).await.unwrap();
        let matching: Vec<_> = results
            .iter()
            .filter(|r| r.paper.id == "paper-a")
            .collect();
        assert_eq!(matching.len(), 1);
        assert_eq!(matching[0].text, "Second version.");
    }

    #[tokio::test]
    async fn existing_paper_ids_round_trips_ids_without_escaping() {
        let (store, _dir) = temp_store().await;

        let paper = sample_paper("a-tricky_id__with-chars", &[]);
        store.upsert_paper(&paper, &[]).await.unwrap();

        let ids = store.existing_paper_ids().await.unwrap();
        assert_eq!(ids, vec!["a-tricky_id__with-chars".to_string()]);
    }

    #[tokio::test]
    async fn relations_round_trip() {
        let (store, _dir) = temp_store().await;

        let a = sample_paper("paper-a", &["blockchain"]);
        let b = sample_paper("paper-b", &["blockchain"]);
        let c = sample_paper("paper-c", &["ai"]);

        for p in [&a, &b, &c] {
            store.upsert_paper(p, &[]).await.unwrap();
        }

        store
            .set_relations(
                &a.id,
                &[
                    ("paper-b".to_string(), RelationKind::SameTopic),
                    ("paper-c".to_string(), RelationKind::Cites),
                ],
            )
            .await
            .unwrap();

        let related = store.related(&a.id, 1).await.unwrap();
        assert_eq!(related.len(), 2);

        let b_rel = related.iter().find(|r| r.paper.id == "paper-b").unwrap();
        assert_eq!(b_rel.relation, RelationKind::SameTopic);
        assert_eq!(b_rel.depth, 1);

        let c_rel = related.iter().find(|r| r.paper.id == "paper-c").unwrap();
        assert_eq!(c_rel.relation, RelationKind::Cites);
    }

    #[tokio::test]
    async fn set_relations_replaces_previous_edges() {
        let (store, _dir) = temp_store().await;

        let a = sample_paper("paper-a", &[]);
        let b = sample_paper("paper-b", &[]);
        let c = sample_paper("paper-c", &[]);
        for p in [&a, &b, &c] {
            store.upsert_paper(p, &[]).await.unwrap();
        }

        store
            .set_relations(&a.id, &[("paper-b".to_string(), RelationKind::SameTopic)])
            .await
            .unwrap();
        store
            .set_relations(&a.id, &[("paper-c".to_string(), RelationKind::Cites)])
            .await
            .unwrap();

        let related = store.related(&a.id, 1).await.unwrap();
        assert_eq!(related.len(), 1);
        assert_eq!(related[0].paper.id, "paper-c");
        assert_eq!(related[0].relation, RelationKind::Cites);
    }
}
