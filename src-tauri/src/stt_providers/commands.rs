use crate::{
    db::stt_provider as stt_provider_repo,
    models::ProviderDescriptor,
    stt_providers::{
        deepgram::deepgram_descriptor, fake::fake_stt_descriptor, SttProviderSettings,
    },
};

#[tauri::command]
pub fn get_stt_providers() -> Vec<ProviderDescriptor> {
    tracing::trace!("get_stt_providers called");
    vec![fake_stt_descriptor(), deepgram_descriptor()]
}

#[tauri::command]
pub async fn save_stt_provider_settings(
    provider: String,
    settings: SttProviderSettings,
) -> Result<(), String> {
    tracing::trace!(provider, "save_stt_provider_settings called");
    tokio::task::spawn_blocking(move || {
        stt_provider_repo::upsert_stt_settings(&provider, &settings)
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_stt_provider_settings(
    provider: String,
) -> Result<Option<SttProviderSettings>, String> {
    tracing::trace!(provider, "get_stt_provider_settings called");

    tokio::task::spawn_blocking(move || stt_provider_repo::get_stt_settings(&provider))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}
