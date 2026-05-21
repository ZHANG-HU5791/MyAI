use crate::error::{AppError, AppResult};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::path::PathBuf;

/// Central database handle wrapping an async SQLite connection pool.
/// All file paths are resolved via Tauri's `app_data_dir` — no hardcoded paths.
#[derive(Clone)]
pub struct Database {
    pool: Pool<Sqlite>,
    db_path: PathBuf,
}

impl Database {
    /// Open (or create) the SQLite database at `{app_data_dir}/data/omni_creator.db`.
    /// Runs embedded migrations on first open.
    pub async fn new(app_data_dir: &PathBuf) -> AppResult<Self> {
        let data_dir = app_data_dir.join("data");
        std::fs::create_dir_all(&data_dir)
            .map_err(|e| AppError::Io(e))?;

        let db_path = data_dir.join("omni_creator.db");
        let db_path_str = db_path.to_string_lossy().to_string();

        let options = SqliteConnectOptions::new()
            .filename(&db_path_str)
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        let db = Self { pool, db_path };
        db.run_migrations().await?;
        Ok(db)
    }

    /// Execute the embedded SQL migration file.
    async fn run_migrations(&self) -> AppResult<()> {
        let migration_sql = include_str!("../../migrations/001_initial.sql");
        sqlx::raw_sql(migration_sql).execute(&self.pool).await?;
        Ok(())
    }

    /// Expose the pool for repository functions.
    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }

    /// The resolved path to the database file.
    pub fn db_path(&self) -> &PathBuf {
        &self.db_path
    }
}
