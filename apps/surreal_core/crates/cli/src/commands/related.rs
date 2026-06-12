use anyhow::Result;
use vectordb::VectorStore;

/// Graph traversal over `cites`/`same_topic` edges starting at `paper_id`.
pub async fn run(store: &dyn VectorStore, paper_id: &str, depth: u32) -> Result<serde_json::Value> {
    let results = store.related(paper_id, depth).await?;
    Ok(serde_json::to_value(results)?)
}
