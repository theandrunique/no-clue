use sqlx::SqlitePool;
use tauri::{AppHandle, Manager};

use crate::{
    db::stt_provider_settings as stt_provider_repo,
    domain::{providers::ProviderDescriptor, stt::SttProviderSettings},
    errors::AppError,
    infra::stt_providers::{deepgram_descriptor, fake_stt_descriptor},
};

#[tauri::command]
pub fn get_stt_providers() -> Vec<ProviderDescriptor> {
    tracing::trace!("get_stt_providers called");
    vec![fake_stt_descriptor(), deepgram_descriptor()]
}

#[tauri::command]
pub async fn save_stt_provider_settings(
    app: AppHandle,
    provider: &str,
    settings: SttProviderSettings,
) -> Result<(), AppError> {
    tracing::trace!(provider, "save_stt_provider_settings called");
    let pool = app.state::<SqlitePool>();
    stt_provider_repo::upsert(&pool, provider, &settings).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_stt_provider_settings(
    app: AppHandle,
    provider: &str,
) -> Result<Option<SttProviderSettings>, AppError> {
    tracing::trace!(provider, "get_stt_provider_settings called");
    let pool = app.state::<SqlitePool>();
    Ok(stt_provider_repo::get(&pool, provider).await?)
}
