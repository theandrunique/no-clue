use crate::{
    db::get_connection,
    domain::transcriptions::{AudioSource, Transcript},
};
use rusqlite::{params, Result};

pub fn create(
    id: String,
    conversation_id: String,
    source: AudioSource,
    text: String,
    confidence: f64,
    timestamp: i64,
) -> Result<Transcript> {
    let conn = get_connection()?;

    conn.execute(
        "INSERT INTO transcripts (id, conversation_id, source, text, confidence, timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![id, conversation_id, source.to_string(), text, confidence, timestamp],
    )?;

    Ok(Transcript {
        id,
        conversation_id,
        source,
        text,
        confidence,
        timestamp,
    })
}

pub fn get_by_conversation(conversation_id: &str) -> Result<Vec<Transcript>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT id, conversation_id, source, text, confidence, timestamp FROM transcripts WHERE conversation_id = ?1 ORDER BY timestamp ASC",
    )?;

    let rows = stmt.query_map(params![conversation_id], |row| {
        let source_str: String = row.get(2)?;
        let source = match source_str.to_lowercase().as_str() {
            "microphone" => AudioSource::Microphone,
            _ => AudioSource::System,
        };
        Ok(Transcript {
            id: row.get(0)?,
            conversation_id: row.get(1)?,
            source: source,
            text: row.get(3)?,
            confidence: row.get(4)?,
            timestamp: row.get(5)?,
        })
    })?;

    rows.collect()
}
