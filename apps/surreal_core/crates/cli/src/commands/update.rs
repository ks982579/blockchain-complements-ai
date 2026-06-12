use super::load_summaries;
use crate::relations;
use crate::summary::build_chunks;
use anyhow::Result;
use embedder::Embedder;
use std::collections::HashSet;
use std::path::Path;
use vectordb::VectorStore;

/// Ingest only `summary.md` files not already present in the store, then
/// recompute edges for the newly-inserted papers.
pub async fn run(store: &dyn VectorStore, embedder: &dyn Embedder, research_dir: &Path) -> Result<serde_json::Value> {
    store.init_schema().await?;

    let summaries = load_summaries(research_dir)?;
    let existing: HashSet<String> = store.existing_paper_ids().await?.into_iter().collect();

    let new_summaries: Vec<_> = summaries
        .iter()
        .filter(|s| !existing.contains(&s.paper.id))
        .collect();

    // Edge computation needs the full corpus (new papers may be referenced by
    // existing ones' Background sections too, and vice versa), so build the
    // paper list from everything on disk.
    let all_papers: Vec<_> = summaries.iter().map(|s| s.paper.clone()).collect();

    for parsed in &new_summaries {
        let texts: Vec<String> = parsed
            .sections
            .iter()
            .map(|(heading, text)| format!("{heading}\n\n{text}"))
            .collect();
        let embeddings = embedder.embed_batch(&texts)?;
        let chunks = build_chunks(parsed, embeddings)?;

        store.upsert_paper(&parsed.paper, &chunks).await?;

        let background = parsed
            .sections
            .iter()
            .find(|(heading, _)| heading.eq_ignore_ascii_case("background"))
            .map(|(_, text)| text.as_str());
        let edges = relations::compute_edges(&parsed.paper, background, &all_papers);
        store.set_relations(&parsed.paper.id, &edges).await?;
    }

    Ok(serde_json::json!({
        "papers_inserted": new_summaries.len(),
        "papers_skipped": summaries.len() - new_summaries.len(),
    }))
}
