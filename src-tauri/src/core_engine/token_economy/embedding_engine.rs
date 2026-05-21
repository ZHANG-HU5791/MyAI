use crate::error::AppResult;
use std::sync::OnceLock;

/// Local embedding engine using fastembed.
/// Loads the model on first use — zero network latency, zero API cost.
pub struct EmbeddingEngine {
    // fastembed model is loaded lazily
}

static ENGINE: OnceLock<EmbeddingEngine> = OnceLock::new();

impl EmbeddingEngine {
    pub fn get() -> &'static EmbeddingEngine {
        ENGINE.get_or_init(|| EmbeddingEngine {})
    }

    /// Generate an embedding vector for a single text.
    /// Returns a 384-dimensional vector (all-MiniLM-L6-v2).
    pub fn embed(&self, _text: &str) -> AppResult<Vec<f32>> {
        // fastembed integration will be finalized in Phase 4.2
        // For now, return a zero vector of the correct dimension
        Ok(vec![0.0; 384])
    }

    /// Generate embeddings for multiple texts in a single batch call.
    pub fn embed_batch(&self, texts: &[&str]) -> AppResult<Vec<Vec<f32>>> {
        Ok(texts.iter().map(|_| vec![0.0; 384]).collect())
    }
}
