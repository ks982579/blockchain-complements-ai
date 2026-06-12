//! Text embedding abstraction.
//!
//! Defines the [`Embedder`] trait and [`EmbedderBuilder`] for constructing
//! one. The only import of `fastembed` lives in [`fastembed_impl`] — swapping
//! embedding backends later means adding a new module + builder variant here,
//! with no changes needed in the CLI crate.

mod fastembed_impl;

use anyhow::Result;

/// Dimension of embeddings produced by the default model (AllMiniLM-L6-v2).
pub const EMBEDDING_DIM: usize = 384;

/// A text embedding backend.
pub trait Embedder: Send + Sync {
    /// Embed a single piece of text into a fixed-size vector.
    fn embed(&self, text: &str) -> Result<Vec<f32>>;

    /// Embed a batch of texts. Default implementation calls [`Embedder::embed`]
    /// per item; implementations can override for batching.
    fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        texts.iter().map(|t| self.embed(t)).collect()
    }
}

/// Identifies which backend an [`EmbedderBuilder`] should construct.
///
/// `FastEmbed` is the only variant today; adding a new backend means adding a
/// variant here, a matching module (like [`fastembed_impl`]), and a build arm
/// in [`EmbedderBuilder::build`].
pub enum Backend {
    /// Local ONNX inference via `fastembed`, AllMiniLM-L6-v2 (384-dim).
    FastEmbed,
}

/// Builder/factory for constructing an [`Embedder`].
///
/// ```no_run
/// # use embedder::{Backend, EmbedderBuilder};
/// # fn run() -> anyhow::Result<()> {
/// let embedder = EmbedderBuilder::new(Backend::FastEmbed).build()?;
/// let vector = embedder.embed("some text")?;
/// # Ok(())
/// # }
/// ```
pub struct EmbedderBuilder {
    backend: Backend,
}

impl EmbedderBuilder {
    pub fn new(backend: Backend) -> Self {
        Self { backend }
    }

    /// Loads the chosen backend (downloading model weights on first run if
    /// needed) and returns a ready-to-use [`Embedder`].
    pub fn build(self) -> Result<Box<dyn Embedder>> {
        match self.backend {
            Backend::FastEmbed => {
                let embedder = fastembed_impl::FastEmbedder::new()?;
                Ok(Box::new(embedder))
            }
        }
    }
}
