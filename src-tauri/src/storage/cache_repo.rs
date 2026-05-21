use crate::error::AppResult;
use crate::storage::Database;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CacheEntry {
    pub id: String,
    pub query_hash: String,
    pub query_text: String,
    pub response_text: String,
    pub model_used: String,
    pub token_count: i64,
    pub hit_count: i64,
    pub created_at: String,
    pub last_hit_at: String,
}

pub async fn find_by_query_hash(db: &Database, query_hash: &str) -> AppResult<Option<CacheEntry>> {
    let row = sqlx::query_as::<_, CacheEntry>(
        "SELECT id, query_hash, query_text, response_text, model_used, token_count, hit_count, created_at, last_hit_at FROM cache_entries WHERE query_hash = ?"
    )
    .bind(query_hash)
    .fetch_optional(db.pool())
    .await?;
    Ok(row)
}

pub async fn insert_cache_entry(db: &Database, entry: &CacheEntry) -> AppResult<()> {
    sqlx::query(
        "INSERT INTO cache_entries (id, query_hash, query_text, response_text, model_used, token_count, hit_count, created_at, last_hit_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&entry.id)
    .bind(&entry.query_hash)
    .bind(&entry.query_text)
    .bind(&entry.response_text)
    .bind(&entry.model_used)
    .bind(entry.token_count)
    .bind(entry.hit_count)
    .bind(&entry.created_at)
    .bind(&entry.last_hit_at)
    .execute(db.pool())
    .await?;
    Ok(())
}

pub async fn update_hit_count(db: &Database, entry_id: &str) -> AppResult<()> {
    let now = Utc::now().to_rfc3339();
    sqlx::query("UPDATE cache_entries SET hit_count = hit_count + 1, last_hit_at = ? WHERE id = ?")
        .bind(now)
        .bind(entry_id)
        .execute(db.pool())
        .await?;
    Ok(())
}

pub async fn evict_oldest(db: &Database, max_entries: i64) -> AppResult<()> {
    sqlx::query(
        "DELETE FROM cache_entries WHERE id NOT IN (SELECT id FROM cache_entries ORDER BY last_hit_at DESC LIMIT ?)"
    )
    .bind(max_entries)
    .execute(db.pool())
    .await?;
    Ok(())
}

pub fn make_entry(query_hash: &str, query_text: &str, response_text: &str, model_used: &str, token_count: i64) -> CacheEntry {
    let now = Utc::now().to_rfc3339();
    CacheEntry {
        id: Uuid::new_v4().to_string(),
        query_hash: query_hash.to_string(),
        query_text: query_text.to_string(),
        response_text: response_text.to_string(),
        model_used: model_used.to_string(),
        token_count,
        hit_count: 0,
        created_at: now.clone(),
        last_hit_at: now,
    }
}
