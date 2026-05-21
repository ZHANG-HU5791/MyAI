use crate::error::AppResult;
use crate::storage::Database;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SessionRow {
    pub id: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn create_session(db: &Database, title: &str) -> AppResult<SessionRow> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let session = SessionRow {
        id: id.clone(),
        title: title.to_string(),
        created_at: now.clone(),
        updated_at: now,
    };

    sqlx::query("INSERT INTO sessions (id, title, created_at, updated_at) VALUES (?, ?, ?, ?)")
        .bind(&session.id)
        .bind(&session.title)
        .bind(&session.created_at)
        .bind(&session.updated_at)
        .execute(db.pool())
        .await?;

    Ok(session)
}

pub async fn list_sessions(db: &Database) -> AppResult<Vec<SessionRow>> {
    let rows = sqlx::query_as::<_, SessionRow>("SELECT id, title, created_at, updated_at FROM sessions ORDER BY updated_at DESC")
        .fetch_all(db.pool())
        .await?;
    Ok(rows)
}

pub async fn delete_session(db: &Database, session_id: &str) -> AppResult<()> {
    sqlx::query("DELETE FROM messages WHERE session_id = ?")
        .bind(session_id)
        .execute(db.pool())
        .await?;
    sqlx::query("DELETE FROM sessions WHERE id = ?")
        .bind(session_id)
        .execute(db.pool())
        .await?;
    Ok(())
}

pub async fn update_title(db: &Database, session_id: &str, title: &str) -> AppResult<()> {
    let now = Utc::now().to_rfc3339();
    sqlx::query("UPDATE sessions SET title = ?, updated_at = ? WHERE id = ?")
        .bind(title)
        .bind(now)
        .bind(session_id)
        .execute(db.pool())
        .await?;
    Ok(())
}
