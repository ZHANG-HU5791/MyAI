use crate::error::AppResult;
use crate::storage::{cache_repo, Database};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum CacheResult {
    Hit(CachedResponse),
    Miss,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedResponse {
    pub response_text: String,
    pub model_used: String,
    pub token_count: i64,
    pub cache_id: String,
}

/// Semantic cache guard: checks SQLite for exact hash matches,
/// and LanceDB for vector similarity (Phase 4.3).
pub struct SemanticCacheGuard {
    db: Database,
    similarity_threshold: f32,
    max_entries: i64,
}

impl SemanticCacheGuard {
    pub fn new(db: Database, similarity_threshold: f32, max_entries: i64) -> Self {
        Self {
            db,
            similarity_threshold,
            max_entries,
        }
    }

    /// Check cache for a matching query (exact hash for now, semantic later).
    pub async fn check(&self, query: &str) -> AppResult<CacheResult> {
        let query_hash = hash_query(query);

        if let Some(entry) = cache_repo::find_by_query_hash(&self.db, &query_hash).await? {
            cache_repo::update_hit_count(&self.db, &entry.id).await?;
            return Ok(CacheResult::Hit(CachedResponse {
                response_text: entry.response_text,
                model_used: entry.model_used,
                token_count: entry.token_count,
                cache_id: entry.id,
            }));
        }

        Ok(CacheResult::Miss)
    }

    /// Store a response in the cache.
    pub async fn store(
        &self,
        query: &str,
        response: &str,
        model: &str,
        tokens: i64,
    ) -> AppResult<()> {
        let entry = cache_repo::make_entry(
            &hash_query(query),
            query,
            response,
            model,
            tokens,
        );
        cache_repo::insert_cache_entry(&self.db, &entry).await?;
        cache_repo::evict_oldest(&self.db, self.max_entries).await?;
        Ok(())
    }
}

fn hash_query(query: &str) -> String {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    query.trim().to_lowercase().hash(&mut hasher);
    format!("{:x}", hasher.finish())
}
