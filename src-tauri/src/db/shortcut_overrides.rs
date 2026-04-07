use rusqlite::Result;

use crate::models::shortcut::ShortcutOverride;

pub fn get_all_overrides() -> Result<Vec<ShortcutOverride>> {
    let conn = crate::db::get_connection()?;
    let mut stmt = conn.prepare("SELECT id, key_override, enabled FROM shortcut_overrides")?;
    let overrides = stmt
        .query_map([], |row| {
            Ok(ShortcutOverride {
                id: row.get(0)?,
                key_override: row.get(1)?,
                enabled: row.get::<_, i32>(2)? == 1,
            })
        })?
        .collect::<Result<Vec<_>>>()?;
    Ok(overrides)
}

pub fn get_override(shortcut_id: &str) -> Result<Option<ShortcutOverride>> {
    let conn = crate::db::get_connection()?;
    let mut stmt =
        conn.prepare("SELECT id, key_override, enabled FROM shortcut_overrides WHERE id = ?")?;
    let mut rows = stmt.query([shortcut_id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(ShortcutOverride {
            id: row.get(0)?,
            key_override: row.get(1)?,
            enabled: row.get::<_, i32>(2)? == 1,
        }))
    } else {
        Ok(None)
    }
}

pub fn save_override(shortcut_id: &str, key_override: Option<String>, enabled: bool) -> Result<()> {
    let conn = crate::db::get_connection()?;
    conn.execute(
        "INSERT INTO shortcut_overrides (id, key_override, enabled) VALUES (?, ?, ?)
         ON CONFLICT(id) DO UPDATE SET key_override = excluded.key_override, enabled = excluded.enabled",
        rusqlite::params![shortcut_id, key_override, enabled as i32],
    )?;
    Ok(())
}

pub fn delete_override(shortcut_id: &str) -> Result<()> {
    let conn = crate::db::get_connection()?;
    conn.execute("DELETE FROM shortcut_overrides WHERE id = ?", [shortcut_id])?;
    Ok(())
}
