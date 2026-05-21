use crate::core_engine::memory::MasterSpec;
use crate::error::AppResult;
use crate::storage::Database;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MasterSpecRow {
    pub id: String,
    pub session_id: Option<String>,
    pub spec_json: String,
    pub version: i32,
    pub created_at: String,
}

pub async fn load_latest(db: &Database, session_id: Option<&str>) -> AppResult<Option<MasterSpec>> {
    let row = if let Some(sid) = session_id {
        sqlx::query_as::<_, MasterSpecRow>(
            "SELECT id, session_id, spec_json, version, created_at FROM master_specs WHERE session_id = ? ORDER BY version DESC LIMIT 1"
        )
        .bind(sid)
        .fetch_optional(db.pool())
        .await?
    } else {
        sqlx::query_as::<_, MasterSpecRow>(
            "SELECT id, session_id, spec_json, version, created_at FROM master_specs ORDER BY version DESC LIMIT 1"
        )
        .fetch_optional(db.pool())
        .await?
    };

    match row {
        Some(r) => {
            let spec: MasterSpec = serde_json::from_str(&r.spec_json)?;
            Ok(Some(spec))
        }
        None => Ok(None),
    }
}

pub async fn save(db: &Database, session_id: Option<&str>, spec: &MasterSpec) -> AppResult<()> {
    let spec_json = serde_json::to_string(spec)?;

    // Get next version number
    let max_version: i32 = if let Some(sid) = session_id {
        sqlx::query_scalar::<_, i32>(
            "SELECT COALESCE(MAX(version), 0) FROM master_specs WHERE session_id = ?"
        )
        .bind(sid)
        .fetch_one(db.pool())
        .await?
    } else {
        sqlx::query_scalar::<_, i32>(
            "SELECT COALESCE(MAX(version), 0) FROM master_specs"
        )
        .fetch_one(db.pool())
        .await?
    };

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO master_specs (id, session_id, spec_json, version, created_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(session_id)
    .bind(&spec_json)
    .bind(max_version + 1)
    .bind(&now)
    .execute(db.pool())
    .await?;

    Ok(())
}
