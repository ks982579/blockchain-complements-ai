//! Vector store abstraction.
//!
//! This crate defines the [`VectorStore`] trait that the rest of the project
//! depends on, plus a [`VectorStoreBuilder`] for constructing one. The only
//! import of the `surrealdb` SDK lives in [`surreal`] — swapping backends
//! later means adding a new module + builder variant here, with no changes
//! needed in the CLI crate.

mod surreal;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// A paper record (one per `research/<dir>/summary.md`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paper {
    /// Directory name, used as the unique key.
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub year: Option<i64>,
    pub venue: Option<String>,
    pub doi: Option<String>,
    pub url: Option<String>,
    pub keywords: Vec<String>,
    /// Arbitrary extra frontmatter fields (volume, issue, pages, type, ...),
    /// stored as-is for retrieval but not used in matching logic.
    pub extra: serde_json::Value,
}

/// A single `##`-section chunk of a paper's summary, with its embedding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    /// ID of the parent [`Paper`].
    pub paper_id: String,
    /// Section heading (e.g. "Overview", "Background", "Key Points").
    pub section: String,
    pub text: String,
    pub embedding: Vec<f32>,
}

/// A chunk returned from a similarity query, joined with its parent paper's metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub paper: Paper,
    pub section: String,
    pub text: String,
    /// Cosine distance to the query embedding (lower = more similar).
    pub distance: f32,
}

/// Kind of graph edge between two papers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationKind {
    Cites,
    SameTopic,
}

/// A related paper returned from graph traversal, with the edge that connects it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedPaper {
    pub paper: Paper,
    pub relation: RelationKind,
    /// Number of hops from the queried paper.
    pub depth: u32,
}

/// Backend-agnostic operations needed by the `surreal_core` CLI.
///
/// Implementations own their connection lifecycle: callers `connect()`, do
/// work, then `close()` (or simply drop the store) before process exit.
#[async_trait]
pub trait VectorStore: Send + Sync {
    /// Create/ensure schema (tables, indexes) exists. Idempotent.
    async fn init_schema(&self) -> Result<()>;

    /// Insert or update a paper record and its chunks. Replaces any existing
    /// chunks for `paper.id`.
    async fn upsert_paper(&self, paper: &Paper, chunks: &[Chunk]) -> Result<()>;

    /// Returns the set of paper IDs already present in the store.
    async fn existing_paper_ids(&self) -> Result<Vec<String>>;

    /// Replace all `cites`/`same_topic` edges for `paper_id` with `edges`.
    async fn set_relations(
        &self,
        paper_id: &str,
        edges: &[(String, RelationKind)],
    ) -> Result<()>;

    /// K-nearest-neighbor search over chunk embeddings.
    async fn query(&self, embedding: &[f32], top_k: usize) -> Result<Vec<QueryResult>>;

    /// Graph traversal over `cites`/`same_topic` edges starting at `paper_id`.
    async fn related(&self, paper_id: &str, depth: u32) -> Result<Vec<RelatedPaper>>;

    /// Cleanly release the underlying connection. Called before process exit.
    async fn close(&self) -> Result<()>;
}

/// Identifies which backend a [`VectorStoreBuilder`] should construct.
///
/// `Surreal` is the only variant today; adding a new backend means adding a
/// variant here, a matching module (like [`surreal`]), and a build arm in
/// [`VectorStoreBuilder::build`].
pub enum Backend {
    /// Embedded SurrealDB (SurrealKV) at the given file path.
    Surreal { path: String },
}

/// Builder/factory for constructing a [`VectorStore`].
///
/// ```no_run
/// # use vectordb::{Backend, VectorStoreBuilder};
/// # async fn run() -> anyhow::Result<()> {
/// let store = VectorStoreBuilder::new(Backend::Surreal { path: ".surreal/research.db".into() })
///     .build()
///     .await?;
/// # Ok(())
/// # }
/// ```
pub struct VectorStoreBuilder {
    backend: Backend,
}

impl VectorStoreBuilder {
    pub fn new(backend: Backend) -> Self {
        Self { backend }
    }

    /// Connects to the chosen backend and returns a ready-to-use [`VectorStore`].
    pub async fn build(self) -> Result<Box<dyn VectorStore>> {
        match self.backend {
            Backend::Surreal { path } => {
                let store = surreal::SurrealStore::connect(&path).await?;
                Ok(Box::new(store))
            }
        }
    }
}
