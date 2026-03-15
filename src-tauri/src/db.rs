use rusqlite::{params, Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

static DB_PATH: Mutex<Option<PathBuf>> = Mutex::new(None);

pub fn set_db_path(path: PathBuf) {
    let mut db_path = DB_PATH.lock().unwrap();
    *db_path = Some(path);
}

fn get_connection() -> Result<Connection> {
    let db_path = DB_PATH.lock().unwrap();
    let path = db_path.as_ref().expect("Database path not set");
    Connection::open(path)
}

pub fn init_db(app_data_dir: &std::path::Path) -> Result<()> {
    let db_path = app_data_dir.join("no-clue.db");
    set_db_path(db_path.clone());

    let conn = Connection::open(&db_path)?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS conversations (
            id TEXT PRIMARY KEY NOT NULL,
            title TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS messages (
            id TEXT PRIMARY KEY NOT NULL,
            conversation_id TEXT NOT NULL,
            role TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'system')),
            content TEXT NOT NULL,
            timestamp INTEGER NOT NULL,
            FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS transcripts (
            id TEXT PRIMARY KEY NOT NULL,
            conversation_id TEXT NOT NULL,
            speaker TEXT NOT NULL CHECK (speaker IN ('user', 'system')),
            text TEXT NOT NULL,
            confidence REAL,
            timestamp INTEGER NOT NULL,
            FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY NOT NULL,
            value TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_messages_conversation ON messages(conversation_id);
        CREATE INDEX IF NOT EXISTS idx_transcripts_conversation ON transcripts(conversation_id);
        ",
    )?;

    println!("[DB] Database initialized at {:?}", db_path);
    Ok(())
}

#[tauri::command]
pub async fn save_transcript(
    conversation_id: String,
    speaker: String,
    text: String,
    confidence: Option<f64>,
) -> Result<(), String> {
    add_transcript(conversation_id, speaker, text, confidence).map_err(|e| e.to_string())
}

fn add_transcript(
    conversation_id: String,
    speaker: String,
    text: String,
    confidence: Option<f64>,
) -> Result<()> {
    let conn = get_connection()?;
    let id = uuid::Uuid::new_v4().to_string();
    let timestamp = chrono::Utc::now().timestamp();

    conn.execute(
        "INSERT INTO transcripts (id, conversation_id, speaker, text, confidence, timestamp) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![id, conversation_id, speaker, text, confidence, timestamp],
    )?;

    Ok(())
}

#[tauri::command]
pub async fn save_message(
    conversation_id: String,
    role: String,
    content: String,
) -> Result<(), String> {
    add_message(conversation_id, role, content).map_err(|e| e.to_string())
}

fn add_message(conversation_id: String, role: String, content: String) -> Result<()> {
    let conn = get_connection()?;
    let id = uuid::Uuid::new_v4().to_string();
    let timestamp = chrono::Utc::now().timestamp();

    conn.execute(
        "INSERT INTO messages (id, conversation_id, role, content, timestamp) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, conversation_id, role, content, timestamp],
    )?;

    Ok(())
}

#[tauri::command]
pub async fn create_conversation_db(title: String) -> Result<String, String> {
    create_conversation(title).map_err(|e| e.to_string())
}

fn create_conversation(title: String) -> Result<String> {
    let conn = get_connection()?;
    let id = uuid::Uuid::new_v4().to_string();
    let timestamp = chrono::Utc::now().timestamp();

    conn.execute(
        "INSERT INTO conversations (id, title, created_at, updated_at) 
         VALUES (?1, ?2, ?3, ?4)",
        params![id, title, timestamp, timestamp],
    )?;

    Ok(id)
}

#[derive(Debug, serde::Serialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[tauri::command]
pub async fn get_conversations_db() -> Result<Vec<Conversation>, String> {
    get_conversations().map_err(|e| e.to_string())
}

fn get_conversations() -> Result<Vec<Conversation>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT id, title, created_at, updated_at FROM conversations ORDER BY updated_at DESC",
    )?;

    let conversations = stmt
        .query_map([], |row| {
            Ok(Conversation {
                id: row.get(0)?,
                title: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>>>()?;

    Ok(conversations)
}

#[tauri::command]
pub async fn get_conversation_db(id: String) -> Result<Option<Conversation>, String> {
    get_conversation(id).map_err(|e| e.to_string())
}

fn get_conversation(id: String) -> Result<Option<Conversation>> {
    let conn = get_connection()?;
    let mut stmt =
        conn.prepare("SELECT id, title, created_at, updated_at FROM conversations WHERE id = ?1")?;

    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(Conversation {
            id: row.get(0)?,
            title: row.get(1)?,
            created_at: row.get(2)?,
            updated_at: row.get(3)?,
        }))
    } else {
        Ok(None)
    }
}
