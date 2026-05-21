use crate::error::AppResult;
use crate::storage::Database;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MessageRow {
    pub id: String,
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub model_used: Option<String>,
    pub token_count: Option<i64>,
    pub created_at: String,
}

pub async fn insert_message(db: &Database, msg: &MessageRow) -> AppResult<()> {
    sqlx::query(
        "INSERT INTO messages (id, session_id, role, content, model_used, token_count, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&msg.id)
    .bind(&msg.session_id)
    .bind(&msg.role)
    .bind(&msg.content)
    .bind(&msg.model_used)
    .bind(msg.token_count)
    .bind(&msg.created_at)
    .execute(db.pool())
    .await?;

    // Touch the session's updated_at
    let now = Utc::now().to_rfc3339();
    sqlx::query("UPDATE sessions SET updated_at = ? WHERE id = ?")
        .bind(now)
        .bind(&msg.session_id)
        .execute(db.pool())
        .await?;

    Ok(())
}

pub async fn get_messages(db: &Database, session_id: &str) -> AppResult<Vec<MessageRow>> {
    let rows = sqlx::query_as::<_, MessageRow>(
        "SELECT id, session_id, role, content, model_used, token_count, created_at FROM messages WHERE session_id = ? ORDER BY created_at ASC"
    )
    .bind(session_id)
    .fetch_all(db.pool())
    .await?;
    Ok(rows)
}

pub async fn get_last_n_messages(db: &Database, session_id: &str, n: i32) -> AppResult<Vec<MessageRow>> {
    let rows = sqlx::query_as::<_, MessageRow>(
        "SELECT id, session_id, role, content, model_used, token_count, created_at FROM messages WHERE session_id = ? ORDER BY created_at DESC LIMIT ?"
    )
    .bind(session_id)
    .bind(n)
    .fetch_all(db.pool())
    .await?;

    // Reverse so oldest is first
    let mut rows = rows;
    rows.reverse();
    Ok(rows)
}

pub fn new_message(session_id: &str, role: &str, content: &str, model_used: Option<&str>) -> MessageRow {
    MessageRow {
        id: Uuid::new_v4().to_string(),
        session_id: session_id.to_string(),
        role: role.to_string(),
        content: content.to_string(),
        model_used: model_used.map(|s| s.to_string()),
        token_count: None,
        created_at: Utc::now().to_rfc3339(),
    }
}
