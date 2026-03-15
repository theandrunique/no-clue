use crate::db::get_connection;
use crate::models::{Speaker, Transcript};
use rusqlite::{params, Result};

pub fn create(
    conversation_id: String,
    speaker: Speaker,
    text: String,
    confidence: Option<f64>,
) -> Result<String> {
    let conn = get_connection()?;
    let id = crate::db::create_uuid();
    let timestamp = crate::db::now_timestamp();

    conn.execute(
        "INSERT INTO transcripts (id, conversation_id, speaker, text, confidence, timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![id, conversation_id, speaker.to_string(), text, confidence, timestamp],
    )?;

    Ok(id)
}

pub fn get_by_conversation(conversation_id: &str) -> Result<Vec<Transcript>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT id, conversation_id, speaker, text, confidence, timestamp FROM transcripts WHERE conversation_id = ?1 ORDER BY timestamp ASC",
    )?;

    let rows = stmt.query_map(params![conversation_id], |row| {
        let speaker_str: String = row.get(2)?;
        Ok(Transcript {
            id: row.get(0)?,
            conversation_id: row.get(1)?,
            speaker: speaker_str.parse().unwrap_or(Speaker::User),
            text: row.get(3)?,
            confidence: row.get(4)?,
            timestamp: row.get(5)?,
        })
    })?;

    rows.collect()
}
