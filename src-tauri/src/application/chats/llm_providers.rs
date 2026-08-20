use sqlx::SqlitePool;
use tauri::{AppHandle, Manager};

use crate::{
    db::llm_provider_settings as provider_repo,
    domain::{
        llm::{LlmProviderSettings, ModelInfo},
        provider_schema::ProviderDescriptor,
    },
    errors::AppError,
    infra::llm_providers::{
        ai_tunnel_descriptor, create_llm_provider, fake_provider_descriptor, ollama_descriptor,
    },
};

#[tauri::command]
pub fn get_llm_providers() -> Vec<ProviderDescriptor> {
    tracing::trace!("get_llm_providers called");
    vec![
        fake_provider_descriptor(),
        ollama_descriptor(),
        ai_tunnel_descriptor(),
    ]
}

#[tauri::command]
pub async fn save_llm_provider_settings(
    app: AppHandle,
    provider: &str,
    settings: LlmProviderSettings,
) -> Result<(), AppError> {
    tracing::trace!(provider, "save_ai_provider_settings called");
    let pool = app.state::<SqlitePool>();
    provider_repo::upsert(&pool, provider, &settings).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_llm_provider_settings(
    app: AppHandle,
    provider: &str,
) -> Result<Option<LlmProviderSettings>, AppError> {
    tracing::trace!(provider, "get_ai_provider_settings called");
    let pool = app.state::<SqlitePool>();
    Ok(provider_repo::get(&pool, &provider).await?)
}

#[tauri::command]
pub async fn get_model_info(app: AppHandle, provider: &str) -> Result<ModelInfo, AppError> {
    tracing::trace!(provider, "get_model_info called");

    let pool = app.state::<SqlitePool>();
    let settings = provider_repo::get(&pool, &provider)
        .await?
        .ok_or_else(|| AppError::LlmProviderNotConfigured)?;

    let ai_provider = create_llm_provider(&settings)?;
    Ok(ai_provider.get_model_info().await?)
}
