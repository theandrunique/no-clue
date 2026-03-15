use crate::{
    conversations::{
        create_conversation, get_conversation, get_conversations, send_message, stop_stream,
    },
    transcriptions::{start_transcription, stop_transcription, update_transcription_session},
    utils::{move_overlay, open_dashboard, set_overlay_visible},
};
use serde::{Deserialize, Serialize};
use tauri::Manager;

mod conversations;
mod db;
mod transcriptions;
mod utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderSettings {
    pub provider: String,
    pub api_key: String,
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SttSettings {
    pub api_key: String,
    pub model: String,
    pub language: String,
}

// Provider Settings
#[tauri::command]
async fn save_provider_settings(
    provider: String,
    _api_key: String,
    model: String,
) -> Result<(), String> {
    println!(
        "[COMMAND] save_provider_settings called: provider={}, model={}",
        provider, model
    );
    Ok(())
}

#[tauri::command]
async fn get_provider_settings(provider: String) -> Result<ProviderSettings, String> {
    println!(
        "[COMMAND] get_provider_settings called: provider={}",
        provider
    );
    Err("Not implemented".to_string())
}

#[tauri::command]
async fn get_all_providers() -> Result<Vec<String>, String> {
    println!("[COMMAND] get_all_providers called");
    Ok(vec![
        "openrouter".to_string(),
        "openai".to_string(),
        "anthropic".to_string(),
    ])
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations(
                    "sqlite:no-clue.db",
                    vec![tauri_plugin_sql::Migration {
                        version: 1,
                        description: "create_initial_schema",
                        sql: include_str!("../migrations/001_initial.sql"),
                        kind: tauri_plugin_sql::MigrationKind::Up,
                    }],
                )
                .build(),
        )
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
            get_conversations,
            get_conversation,
            update_transcription_session,
            send_message,
            stop_stream,
            start_transcription,
            stop_transcription,
            save_provider_settings,
            get_provider_settings,
            get_all_providers,
            save_stt_settings,
            get_stt_settings,
            db::save_transcript,
            db::save_message,
            db::create_conversation_db,
            db::get_conversations_db,
            db::get_conversation_db,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
