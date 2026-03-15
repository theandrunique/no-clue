use crate::db::get_connection;
use crate::models::{Message, MessageRole};
use rusqlite::{params, Result};

pub fn create(conversation_id: String, role: MessageRole, content: String) -> Result<String> {
    let conn = get_connection()?;
    let id = crate::db::create_uuid();
    let timestamp = crate::db::now_timestamp();

    conn.execute(
        "INSERT INTO messages (id, conversation_id, role, content, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, conversation_id, role.to_string(), content, timestamp],
    )?;

    Ok(id)
}

pub fn get_by_conversation(conversation_id: &str) -> Result<Vec<Message>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT id, conversation_id, role, content, timestamp FROM messages WHERE conversation_id = ?1 ORDER BY timestamp ASC",
    )?;

    let rows = stmt.query_map(params![conversation_id], |row| {
        let role_str: String = row.get(2)?;
        Ok(Message {
            id: row.get(0)?,
            conversation_id: row.get(1)?,
            role: role_str.parse().unwrap_or(MessageRole::User),
            content: row.get(3)?,
            timestamp: row.get(4)?,
        })
    })?;

    rows.collect()
}
