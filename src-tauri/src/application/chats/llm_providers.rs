use serde::Serialize;
use sqlx::SqlitePool;
use tauri::State;

use crate::{
    domain::{
        chats::{LlmProviderSettings, LLM_PROVIDER_DESCRIPTORS},
        provider_schema::{LlmCapabilities, ModelRuntimeSettingsSchema, ProviderSettingsSchema},
    },
    errors::AppError,
    infra::db,
};

#[derive(Serialize)]
pub struct LlmProviderInfo {
    id: String,
    display_name: String,
    settings: Option<LlmProviderSettings>,
    settings_schema: ProviderSettingsSchema,
}

#[tauri::command]
pub async fn get_llm_providers(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<LlmProviderInfo>, AppError> {
    let mut result = Vec::new();

    for descriptor in LLM_PROVIDER_DESCRIPTORS.iter() {
        let settings = db::llm_provider_settings::get(&pool, &descriptor.id).await?;

        result.push(LlmProviderInfo {
            id: descriptor.id.clone(),
            display_name: descriptor.display_name.clone(),
            settings,
            settings_schema: descriptor.settings_schema.clone(),
        })
    }

    result.sort_by_key(|p| p.settings.is_none());

    Ok(result)
}

#[tauri::command]
pub async fn save_llm_provider_settings(
    pool: State<'_, SqlitePool>,
    provider_id: &str,
    settings: LlmProviderSettings,
) -> Result<(), AppError> {
    db::llm_provider_settings::upsert(&pool, provider_id, &settings).await?;
    Ok(())
}

#[tauri::command]
pub async fn remove_llm_provider_settings(
    pool: State<'_, SqlitePool>,
    provider_id: &str,
) -> Result<(), AppError> {
    db::llm_provider_settings::delete(&pool, provider_id).await?;
    Ok(())
}

#[derive(Serialize)]
pub struct LlmInfo {
    id: String,
    display_name: String,
    provider_id: String,
    provider_display_name: String,
    capabilities: LlmCapabilities,
    runtime_config: ModelRuntimeSettingsSchema,
}

#[tauri::command]
pub async fn get_available_llms(pool: State<'_, SqlitePool>) -> Result<Vec<LlmInfo>, AppError> {
    let mut result = Vec::new();

    for descriptor in LLM_PROVIDER_DESCRIPTORS.iter() {
        let settings = db::llm_provider_settings::get(&pool, &descriptor.id).await?;

        if settings.is_none() {
            continue;
        }

        for model in &descriptor.models {
            result.push(LlmInfo {
                id: model.id.clone(),
                display_name: model.display_name.clone(),
                provider_id: descriptor.id.clone(),
                provider_display_name: descriptor.display_name.clone(),
                capabilities: model.capabilities.clone(),
                runtime_config: model.runtime_settings.clone(),
            });
        }
    }

    result.sort_by(|a, b| a.provider_id.cmp(&b.provider_id));

    Ok(result)
}
