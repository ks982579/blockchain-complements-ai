use super::load_summaries;
use crate::relations;
use crate::summary::build_chunks;
use anyhow::Result;
use embedder::Embedder;
use std::path::Path;
use vectordb::VectorStore;

/// Recreate schema and (re)ingest every `summary.md` under `research_dir`.
pub async fn run(store: &dyn VectorStore, embedder: &dyn Embedder, research_dir: &Path) -> Result<serde_json::Value> {
    store.init_schema().await?;

    let summaries = load_summaries(research_dir)?;
    let papers: Vec<_> = summaries.iter().map(|s| s.paper.clone()).collect();

    let mut inserted = 0usize;

    for parsed in &summaries {
        let texts: Vec<String> = parsed
            .sections
            .iter()
            .map(|(heading, text)| format!("{heading}\n\n{text}"))
            .collect();
        let embeddings = embedder.embed_batch(&texts)?;
        let chunks = build_chunks(parsed, embeddings)?;

        store.upsert_paper(&parsed.paper, &chunks).await?;
        inserted += 1;
    }

    let mut edges_set = 0usize;
    for parsed in &summaries {
        let background = parsed
            .sections
            .iter()
            .find(|(heading, _)| heading.eq_ignore_ascii_case("background"))
            .map(|(_, text)| text.as_str());

        let edges = relations::compute_edges(&parsed.paper, background, &papers);
        store.set_relations(&parsed.paper.id, &edges).await?;
        edges_set += 1;
    }

    Ok(serde_json::json!({
        "papers_ingested": inserted,
        "papers_with_edges": edges_set,
    }))
}
