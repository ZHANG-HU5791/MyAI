use serde::Serialize;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Storage error: {0}")]
    Storage(String),

    #[error("IPC error: {0}")]
    Ipc(String),

    #[error("MCP error: {0}")]
    Mcp(String),

    #[error("LLM error: {0}")]
    Llm(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Path resolution error: {0}")]
    Path(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Database error: {0}")]
    Database(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Keyring error: {0}")]
    Keyring(String),

    #[error("Event bus error: {0}")]
    EventBus(String),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<keyring::Error> for AppError {
    fn from(err: keyring::Error) -> Self {
        AppError::Keyring(err.to_string())
    }
}

impl From<tokio::sync::broadcast::error::SendError<crate::event_bus::bus::AppEvent>>
    for AppError
{
    fn from(err: tokio::sync::broadcast::error::SendError<crate::event_bus::bus::AppEvent>) -> Self {
        AppError::EventBus(err.to_string())
    }
}
