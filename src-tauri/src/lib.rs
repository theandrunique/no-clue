use crate::{
    ai_providers::{get_provider_settings, get_providers, save_provider_settings},
    audio_capture::{get_input_devices, get_output_devices, test_stream_audio},
    chat::{send_message, stop_stream},
    conversations::{
        create_conversation, delete_conversation, get_conversation, get_conversations,
        get_messages, get_transcripts,
    },
    transcriptions::{
        get_stt_providers, get_stt_settings, save_stt_settings, start_transcription,
        stop_transcription, update_transcription_session,
    },
    utils::{move_overlay, open_dashboard, set_overlay_visible},
};
use std::sync::OnceLock;
use tauri::Manager;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

static LOG_GUARD: OnceLock<WorkerGuard> = OnceLock::new();

mod ai_providers;
mod audio_capture;
mod chat;
mod conversations;
mod db;
mod error;
mod models;
mod screenshot;
mod stt_providers;
mod transcriptions;
mod utils;

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

            let logs_dir = app_data_dir.join("logs");
            std::fs::create_dir_all(&logs_dir).ok();

            let file_appender = RollingFileAppender::new(Rotation::DAILY, &logs_dir, "no-clue");
            let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

            LOG_GUARD.set(guard).ok();

            let file_layer = fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_target(true);

            let stdout_layer = fmt::layer()
                .with_writer(std::io::stdout)
                .with_ansi(true)
                .with_target(true);

            let env_filter = EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("trace,wasapi=warn,wgpu=warn,nokia=warn"));

            tracing_subscriber::registry()
                .with(env_filter)
                .with(file_layer)
                .with(stdout_layer)
                .init();

            tracing::info!(logs_dir = %logs_dir.display(), "Logging initialized");

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
            send_message,
            stop_stream,
            start_transcription,
            stop_transcription,
            update_transcription_session,
            save_stt_settings,
            get_stt_settings,
            get_stt_providers,
            get_providers,
            save_provider_settings,
            get_provider_settings,
            get_input_devices,
            get_output_devices,
            test_stream_audio,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
