use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Result from a vector similarity search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub text: String,
    pub score: f32,
    pub metadata: serde_json::Value,
}

/// LanceDB wrapper for local vector storage.
/// Schema: id (Utf8), text (Utf8), vector (FixedSizeList<Float32, 384>), metadata (Utf8).
#[derive(Clone)]
pub struct LanceStore {
    db_path: PathBuf,
    // LanceDB handles are not Send-safe at the type level in all versions,
    // so we store the path and open connections on demand.
}

impl LanceStore {
    pub async fn new(app_data_dir: &PathBuf) -> AppResult<Self> {
        let lance_dir = app_data_dir.join("lance_db");
        std::fs::create_dir_all(&lance_dir)
            .map_err(|e| AppError::Io(e))?;
        Ok(Self { db_path: lance_dir })
    }

    pub fn db_path(&self) -> &PathBuf {
        &self.db_path
    }

    pub async fn initialize(&self) -> AppResult<()> {
        // LanceDB table creation will be implemented in Phase 4.3
        // when LanceDB crate integration is finalized.
        log::info!("LanceDB initialized at {:?}", self.db_path);
        Ok(())
    }

    pub async fn insert_embedding(
        &self,
        _text: &str,
        _embedding: &[f32],
        _metadata: &serde_json::Value,
    ) -> AppResult<()> {
        // Full implementation in Phase 4.3
        Ok(())
    }

    pub async fn search_embedding(
        &self,
        _query_embedding: &[f32],
        _top_k: usize,
    ) -> AppResult<Vec<SearchResult>> {
        // Full implementation in Phase 4.3
        Ok(Vec::new())
    }
}
