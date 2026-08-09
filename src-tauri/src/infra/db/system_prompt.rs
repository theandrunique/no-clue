use crate::{db::get_connection, domain::system_prompts::SystemPrompt};
use rusqlite::{params, Result};

pub fn create(prompt: &SystemPrompt) -> Result<()> {
    let conn = get_connection()?;

    conn.execute(
        "INSERT INTO system_prompts (id, name, prompt, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            prompt.id,
            prompt.name,
            prompt.prompt,
            prompt.created_at,
            prompt.updated_at
        ],
    )?;

    Ok(())
}

pub fn get_all() -> Result<Vec<SystemPrompt>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT id, name, prompt, created_at, updated_at FROM system_prompts ORDER BY updated_at DESC",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(SystemPrompt {
            id: row.get(0)?,
            name: row.get(1)?,
            prompt: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    })?;

    rows.collect()
}

pub fn get_by_id(id: &str) -> Result<Option<SystemPrompt>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT id, name, prompt, created_at, updated_at FROM system_prompts WHERE id = ?1",
    )?;

    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(SystemPrompt {
            id: row.get(0)?,
            name: row.get(1)?,
            prompt: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn update(id: &str, name: &str, prompt: &str) -> Result<()> {
    let conn = get_connection()?;
    let now = chrono::Utc::now().timestamp();

    conn.execute(
        "UPDATE system_prompts SET name = ?1, prompt = ?2, updated_at = ?3 WHERE id = ?4",
        params![name, prompt, now, id],
    )?;

    Ok(())
}

pub fn delete(id: &str) -> Result<()> {
    let conn = get_connection()?;
    conn.execute("DELETE FROM system_prompts WHERE id = ?1", params![id])?;
    Ok(())
}
