use crate::{
    application::ai_providers::{
        ai_tunnel::ai_tunnel_descriptor, create_ai_provider, fake::fake_provider_descriptor,
        ollama::ollama_descriptor, AiProviderSettings,
    },
    db::ai_provider as provider_repo,
    domain::{providers::ProviderDescriptor, ModelInfo},
};

#[tauri::command]
pub fn get_ai_providers() -> Vec<ProviderDescriptor> {
    tracing::trace!("get_ai_providers called");
    vec![
        fake_provider_descriptor(),
        ollama_descriptor(),
        ai_tunnel_descriptor(),
    ]
}

#[tauri::command]
pub async fn save_ai_provider_settings(
    provider: String,
    settings: AiProviderSettings,
) -> Result<(), String> {
    tracing::trace!(provider, "save_ai_provider_settings called");
    tokio::task::spawn_blocking(move || provider_repo::upsert_provider(&provider, &settings))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ai_provider_settings(
    provider: String,
) -> Result<Option<AiProviderSettings>, String> {
    tracing::trace!(provider, "get_ai_provider_settings called");
    tokio::task::spawn_blocking(move || provider_repo::get_provider_settings(&provider))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_model_info(provider: String) -> Result<ModelInfo, String> {
    tracing::trace!(provider, "get_model_info called");
    let provider_clone = provider.clone();
    let settings =
        tokio::task::spawn_blocking(move || provider_repo::get_provider_settings(&provider))
            .await
            .map_err(|e| e.to_string())?
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("Provider {} not configured", provider_clone))?;

    let ai_provider = create_ai_provider(&settings)?;
    ai_provider.get_model_info().await
}
