use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{ConnectOptions, SqlitePool};
use std::path::PathBuf;
use std::str::FromStr;

pub mod conversation;
pub mod llm_provider_settings;
pub mod message;
pub mod shortcut_overrides;
pub mod stt_provider_settings;
pub mod system_prompt;
pub mod transcript;

pub async fn create_pool(app_dir: &PathBuf) -> Result<SqlitePool, sqlx::Error> {
    let db_path = app_dir.join("no-clue.db");
    let db_url = format!("sqlite://{}", db_path.display());

    tracing::info!(db_url, "Connecting to database");

    let options = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
        .busy_timeout(std::time::Duration::from_secs(5))
        .foreign_keys(true)
        .log_statements(tracing::log::LevelFilter::Info);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    tracing::info!("Running migrations...");

    sqlx::migrate!("src/infra/db/migrations").run(pool).await?;

    tracing::info!("Migrations completed successfully");
    Ok(())
}
