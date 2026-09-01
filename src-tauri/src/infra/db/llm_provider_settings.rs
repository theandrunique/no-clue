use anyhow::Context;
use sqlx::SqlitePool;

use crate::domain::chats::LlmProviderSettings;

pub async fn upsert(
    pool: &SqlitePool,
    provider_id: &str,
    settings: &LlmProviderSettings,
) -> Result<(), anyhow::Error> {
    let settings_json =
        serde_json::to_string(settings).context("Failed to serialize LlmProviderSettings")?;

    sqlx::query(
        "INSERT INTO llm_provider_settings (id, settings)
         VALUES (?, ?)
         ON CONFLICT(id)
            DO UPDATE SET settings=excluded.settings",
    )
    .bind(provider_id)
    .bind(settings_json)
    .execute(pool)
    .await
    .context("Failed to upsert LLM provider settigns")?;

    Ok(())
}

pub async fn get(
    pool: &SqlitePool,
    provider_id: &str,
) -> Result<Option<LlmProviderSettings>, anyhow::Error> {
    let row: Option<(String,)> =
        sqlx::query_as("SELECT settings FROM llm_provider_settings WHERE id = ?")
            .bind(provider_id)
            .fetch_optional(pool)
            .await
            .context("Failed to fetch LLM provider settings")?;

    match row {
        Some((settings_json,)) => {
            let settings: LlmProviderSettings = serde_json::from_str(&settings_json)
                .context("Failed to deserialize LlmProviderSettings")?;

            Ok(Some(settings))
        }
        None => Ok(None),
    }
}

pub async fn delete(pool: &SqlitePool, provider_id: &str) -> Result<bool, anyhow::Error> {
    let result = sqlx::query("DELETE FROM llm_provider_settings WHERE id = ?")
        .bind(provider_id)
        .execute(pool)
        .await
        .context("Failed to fetch LLM provider settings")?;

    Ok(result.rows_affected() > 0)
}
