use crate::{
    ai_providers::{get_ai_provider_settings, get_ai_providers, get_model_info, save_ai_provider_settings},
    audio_capture::{
        get_input_devices, get_output_devices, test_microphone_audio, test_system_audio,
    },
    chat::{send_message, stop_stream},
    conversations::{
        create_conversation, delete_conversation, get_conversation, get_conversations,
        get_messages, get_transcripts,
    },
    shortcuts::{delete_shortcut_override, get_shortcuts, register_all_shortcuts, save_shortcut},
    stt_providers::{get_stt_provider_settings, get_stt_providers, save_stt_provider_settings},
    system_prompts::{
        create_system_prompt, delete_system_prompt, get_system_prompt, get_system_prompts,
        update_system_prompt,
    },
    transcriptions::{start_transcription, stop_transcription, update_transcription_session},
    utils::{open_dashboard, set_overlay_visible},
};
use tauri::Manager;

mod ai_providers;
mod audio_capture;
mod audio_processing;
mod chat;
mod conversations;
mod db;
mod error;
mod models;
mod screenshot;
mod shortcuts;
mod stt_providers;
mod system_prompts;
mod transcriptions;
mod utils;
mod logging;

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

            logging::init_logging(&app_data_dir);

            db::init_db(&app_data_dir).expect("Failed to initialize database");

            if let Err(e) = register_all_shortcuts(&app.handle()) {
                tracing::error!("Failed to register shortcuts: {}", e);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_dashboard,
            set_overlay_visible,
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
