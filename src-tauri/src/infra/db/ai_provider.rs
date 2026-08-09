use crate::{application::ai_providers::AiProviderSettings, db::get_connection};
use rusqlite::{params, Result};

pub fn upsert_provider(provider: &str, settings: &AiProviderSettings) -> Result<()> {
    let conn = get_connection()?;

    let settings_json =
        serde_json::to_string(settings).map_err(|_| rusqlite::Error::InvalidQuery)?;

    conn.execute(
        "INSERT INTO ai_providers (id, config)
         VALUES (?1, ?2)
         ON CONFLICT(id) DO UPDATE SET config=excluded.config",
        params![provider, settings_json],
    )?;

    Ok(())
}

pub fn get_provider_settings(provider: &str) -> Result<Option<AiProviderSettings>> {
    let conn = get_connection()?;

    let mut stmt = conn.prepare("SELECT config FROM ai_providers WHERE id = ?1")?;

    let mut rows = stmt.query(params![provider])?;

    if let Some(row) = rows.next()? {
        let config_str: String = row.get(0)?;

        let settings: AiProviderSettings = serde_json::from_str(&config_str).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
        })?;

        Ok(Some(settings))
    } else {
        Ok(None)
    }
}
