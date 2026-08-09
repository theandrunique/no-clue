use crate::{db::get_connection, domain::conversations::Conversation};
use rusqlite::{params, Result};

pub fn create(conversation: &Conversation) -> Result<()> {
    let conn = get_connection()?;

    conn.execute(
        "INSERT INTO conversations (id, title, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
        params![
            conversation.id,
            conversation.title,
            conversation.created_at,
            conversation.updated_at
        ],
    )?;

    Ok(())
}

pub fn get_all() -> Result<Vec<Conversation>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT id, title, created_at, updated_at FROM conversations ORDER BY updated_at DESC",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(Conversation {
            id: row.get(0)?,
            title: row.get(1)?,
            created_at: row.get(2)?,
            updated_at: row.get(3)?,
        })
    })?;

    rows.collect()
}

pub fn get_by_id(id: &str) -> Result<Option<Conversation>> {
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

pub fn delete(id: &str) -> Result<()> {
    let conn = get_connection()?;
    conn.execute("DELETE FROM conversations WHERE id = ?1", params![id])?;
    Ok(())
}
