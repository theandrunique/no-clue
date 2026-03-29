use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

pub mod ai_provider;
pub mod conversation;
pub mod message;
pub mod transcript;

static DB_PATH: Mutex<Option<PathBuf>> = Mutex::new(None);

pub fn get_connection() -> Result<Connection> {
    let db_path = DB_PATH.lock().unwrap();
    let path = db_path.as_ref().expect("Database path not set");
    Connection::open(path)
}

pub fn init_db(app_data_dir: &std::path::Path) -> Result<()> {
    let db_path = app_data_dir.join("no-clue.db");
    {
        let mut path = DB_PATH.lock().unwrap();
        *path = Some(db_path.clone());
    }

    let conn = Connection::open(&db_path)?;

    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    let migration = include_str!("migrations/001_initial.sql");
    conn.execute_batch(migration)?;

    tracing::info!(db_path = %db_path.display(), "Database initialized");
    Ok(())
}
