//! `fastembed` implementation of [`Embedder`].
//!
//! This is the only module in the crate that imports `fastembed`. Everything
//! outside this module talks to the [`crate::Embedder`] trait.

use crate::Embedder;
use anyhow::Result;
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

pub struct FastEmbedder {
    model: TextEmbedding,
}

impl FastEmbedder {
    pub fn new() -> Result<Self> {
        let model = TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true),
        )?;
        Ok(Self { model })
    }
}

impl Embedder for FastEmbedder {
    fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let embeddings = self.model.embed(vec![text.to_string()], None)?;
        embeddings
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("fastembed returned no embeddings"))
    }

    fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        Ok(self.model.embed(texts.to_vec(), None)?)
    }
}
