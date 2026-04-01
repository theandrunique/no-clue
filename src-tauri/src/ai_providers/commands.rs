use crate::{
    ai_providers::{
        ai_tunnel::ai_tunnel_descriptor, fake::fake_provider_descriptor, ollama::ollama_descriptor,
        ProviderDescriptor, ProviderSettings,
    },
    db::ai_provider as provider_repo,
};

#[tauri::command]
pub fn get_providers() -> Vec<ProviderDescriptor> {
    vec![
        fake_provider_descriptor(),
        ollama_descriptor(),
        ai_tunnel_descriptor(),
    ]
}

#[tauri::command]
pub async fn save_provider_settings(
    provider: String,
    settings: ProviderSettings,
) -> Result<(), String> {
    tracing::trace!(provider, "save_provider_settings called");
    tokio::task::spawn_blocking(move || provider_repo::upsert_provider(&provider, &settings))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_provider_settings(provider: String) -> Result<Option<ProviderSettings>, String> {
    tracing::trace!(provider, "get_provider_settings called");
    tokio::task::spawn_blocking(move || provider_repo::get_provider_settings(&provider))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}
