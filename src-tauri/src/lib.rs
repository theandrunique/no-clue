use crate::{
    ai_providers::{get_providers, ProviderSettings}, conversations::{
        create_conversation, delete_conversation, get_conversation, get_conversations, get_messages, get_transcripts, send_message, stop_stream,
    }, db::ai_provider as provider_repo, transcriptions::{start_transcription, stop_transcription, update_transcription_session}, utils::{move_overlay, open_dashboard, set_overlay_visible}
};
use serde::{Deserialize, Serialize};
use tauri::Manager;

mod conversations;
mod db;
mod models;
mod screenshot;
mod transcriptions;
mod utils;
mod ai_providers;

#[derive(Debug, Serialize, Deserialize)]
pub struct SttSettings {
    pub api_key: String,
    pub model: String,
    pub language: String,
}

// STT Settings
#[tauri::command]
async fn save_stt_settings(
    _api_key: String,
    model: String,
    language: String,
) -> Result<(), String> {
    println!(
        "[COMMAND] save_stt_settings called: model={}, language={}",
        model, language
    );
    Ok(())
}

#[tauri::command]
async fn get_stt_settings() -> Result<SttSettings, String> {
    println!("[COMMAND] get_stt_settings called");
    Ok(SttSettings {
        api_key: "".to_string(),
        model: "nova-3".to_string(),
        language: "ru".to_string(),
    })
}

#[tauri::command]
async fn save_provider_settings(provider: String, settings: ProviderSettings) -> Result<(), String> {
    println!("[COMMAND] save_provider_settings called: provider={}", provider);
    tokio::task::spawn_blocking(move || {
        provider_repo::upsert_provider(&provider, &settings)
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_provider_settings(provider: String) -> Result<Option<ProviderSettings>, String> {
    println!("[COMMAND] get_provider_settings called: provider={}", provider);
    tokio::task::spawn_blocking(move || {
        provider_repo::get_provider_settings(&provider)
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_data_dir).ok();
            db::init_db(&app_data_dir).expect("Failed to initialize database");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_dashboard,
            move_overlay,
            set_overlay_visible,
            create_conversation,
            delete_conversation,
            get_conversations,
            get_conversation,
            get_messages,
            get_transcripts,
            update_transcription_session,
            send_message,
            stop_stream,
            start_transcription,
            stop_transcription,
            save_stt_settings,
            get_stt_settings,
            get_providers,
            save_provider_settings,
            get_provider_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
