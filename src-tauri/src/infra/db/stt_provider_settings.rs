use anyhow::Context;
use sqlx::SqlitePool;

use crate::domain::stt::SttProviderSettings;

pub async fn upsert(
    pool: &SqlitePool,
    provider: &str,
    settings: &SttProviderSettings,
) -> Result<(), anyhow::Error> {
    let settings_json =
        serde_json::to_string(settings).context("Failed to serialize SttProviderSettings")?;

    sqlx::query(
        "INSERT INTO stt_providers_settings (id, settigns)
         VALUES (?, ?)
         ON CONFLICT(id) DO UPDATE SET settings=excluded.settings",
    )
    .bind(provider)
    .bind(settings_json)
    .execute(pool)
    .await
    .context("Failed to upsert STT provider settings")?;

    Ok(())
}

pub async fn get(
    pool: &SqlitePool,
    provider: &str,
) -> Result<Option<SttProviderSettings>, anyhow::Error> {
    let row: Option<(String,)> =
        sqlx::query_as("SELECT settings FROM stt_providers_settings WHERE id = ?")
            .bind(provider)
            .fetch_optional(pool)
            .await
            .context("Failed to fetch STT provider settigns")?;

    match row {
        Some((settings_json,)) => {
            let settings: SttProviderSettings = serde_json::from_str(&settings_json)
                .context("Failed to deserealize SttProviderSettings")?;
            Ok(Some(settings))
        }
        None => Ok(None),
    }
}
