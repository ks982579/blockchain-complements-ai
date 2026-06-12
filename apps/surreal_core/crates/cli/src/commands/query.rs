use anyhow::Result;
use embedder::Embedder;
use vectordb::VectorStore;

/// Embed `question` and return the top-k nearest chunks with paper metadata.
pub async fn run(
    store: &dyn VectorStore,
    embedder: &dyn Embedder,
    question: &str,
    top_k: usize,
) -> Result<serde_json::Value> {
    let embedding = embedder.embed(question)?;
    let results = store.query(&embedding, top_k).await?;
    Ok(serde_json::to_value(results)?)
}
