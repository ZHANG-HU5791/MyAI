use crate::error::AppResult;
use crate::storage::Database;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const KEYRING_SERVICE: &str = "omni-creator-hub";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeys {
    pub gemini: Option<String>,
    pub openai: Option<String>,
    pub deepseek: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub default_model: String,
    pub theme: String,
    pub mcp_config_path: String,
}

fn read_keyring(provider: &str) -> Option<String> {
    match keyring::Entry::new(KEYRING_SERVICE, provider) {
        Ok(entry) => match entry.get_password() {
            Ok(pw) => Some(pw),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

fn write_keyring(provider: &str, key: &str) -> AppResult<()> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, provider)
        .map_err(|e| crate::error::AppError::Keyring(e.to_string()))?;
    entry
        .set_password(key)
        .map_err(|e| crate::error::AppError::Keyring(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub async fn get_api_keys() -> Result<ApiKeys, String> {
    Ok(ApiKeys {
        gemini: read_keyring("gemini"),
        openai: read_keyring("openai"),
        deepseek: read_keyring("deepseek"),
    })
}

#[tauri::command]
pub async fn set_api_key(provider: String, key: String) -> Result<(), String> {
    write_keyring(&provider, &key).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_api_key(provider: String) -> Result<(), String> {
    match keyring::Entry::new(KEYRING_SERVICE, &provider) {
        Ok(entry) => entry
            .delete_credential()
            .map_err(|e| crate::error::AppError::Keyring(e.to_string()).to_string()),
        Err(e) => Err(crate::error::AppError::Keyring(e.to_string()).to_string()),
    }
}

#[tauri::command]
pub async fn get_app_settings(db: tauri::State<'_, Database>) -> Result<AppSettings, String> {
    // Read non-sensitive settings from SQLite
    let default_model = sqlx::query_scalar::<_, String>(
        "SELECT value FROM settings WHERE key = 'default_model'",
    )
    .fetch_optional(db.pool())
    .await
    .map_err(|e| e.to_string())?
    .unwrap_or_else(|| "gemini-2.5-flash".to_string());

    let theme = sqlx::query_scalar::<_, String>(
        "SELECT value FROM settings WHERE key = 'theme'",
    )
    .fetch_optional(db.pool())
    .await
    .map_err(|e| e.to_string())?
    .unwrap_or_else(|| "dark".to_string());

    let mcp_config_path = sqlx::query_scalar::<_, String>(
        "SELECT value FROM settings WHERE key = 'mcp_config_path'",
    )
    .fetch_optional(db.pool())
    .await
    .map_err(|e| e.to_string())?
    .unwrap_or_default();

    Ok(AppSettings {
        default_model,
        theme,
        mcp_config_path,
    })
}

#[tauri::command]
pub async fn update_app_settings(
    db: tauri::State<'_, Database>,
    settings: AppSettings,
) -> Result<(), String> {
    for (key, value) in [
        ("default_model", settings.default_model),
        ("theme", settings.theme),
        ("mcp_config_path", settings.mcp_config_path),
    ] {
        sqlx::query(
            "INSERT INTO settings (key, value) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        )
        .bind(key)
        .bind(value)
        .execute(db.pool())
        .await
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
