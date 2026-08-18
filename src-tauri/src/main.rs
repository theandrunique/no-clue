// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::{
    application::{
        audio::{get_input_devices, get_output_devices, test_microphone_audio, test_system_audio},
        chats::{send_message, stop_stream},
        conversations::{
            create_conversation, delete_conversation, get_conversation, get_conversations,
            get_messages, get_transcripts,
        },
        llm_providers::{
            get_ai_provider_settings, get_ai_providers, get_model_info, save_ai_provider_settings,
        },
        overlay::{start_overlay_session, stop_overlay_session},
        shortcuts::{
            delete_shortcut_override, get_shortcuts, register_all_shortcuts, save_shortcut,
        },
        stt_providers::{get_stt_provider_settings, get_stt_providers, save_stt_provider_settings},
        system_prompts::{
            create_system_prompt, delete_system_prompt, get_system_prompt, get_system_prompts,
            update_system_prompt,
        },
        transcriptions::{start_transcription, stop_transcription, update_transcription_session},
    },
    infra::db,
};
use tauri::Manager;

mod application;
mod domain;
mod errors;
mod infra;
mod logging;
mod presentation;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_data_dir).ok();

            logging::init_logging(&app_data_dir);

            let pool = tauri::async_runtime::block_on(async {
                let pool = db::create_pool(&app_data_dir)
                    .await
                    .expect("Failed to create database pool");
                db::run_migrations(&pool)
                    .await
                    .expect("Failed to run migrations");
                pool
            });

            app.manage(pool);

            if let Err(e) = tauri::async_runtime::block_on(register_all_shortcuts(app.handle())) {
                tracing::error!("Failed to register shortcuts: {}", e);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_conversation,
            delete_conversation,
            get_conversations,
            get_conversation,
            get_messages,
            get_transcripts,
            send_message,
            stop_stream,
            start_transcription,
            stop_transcription,
            update_transcription_session,
            save_stt_provider_settings,
            get_stt_provider_settings,
            get_stt_providers,
            get_ai_providers,
            save_ai_provider_settings,
            get_ai_provider_settings,
            get_input_devices,
            get_output_devices,
            test_system_audio,
            test_microphone_audio,
            get_system_prompts,
            get_system_prompt,
            create_system_prompt,
            update_system_prompt,
            delete_system_prompt,
            get_shortcuts,
            save_shortcut,
            delete_shortcut_override,
            get_model_info,
            start_overlay_session,
            stop_overlay_session,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
