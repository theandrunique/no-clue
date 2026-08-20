use crate::application::transcriptions::CURRENT_CONVERSATION_ID;
use crate::application::transcriptions::SESSION;
use crate::application::transcriptions::TranscriptionSession;
use crate::application::transcriptions::process::run_transcription;
use crate::db::stt_provider_settings as stt_provider_repo;
use crate::domain::transcripts::{AudioCaptureConfig};
use crate::errors::AppError;
use crate::infra::stt_providers::create_stt_provider;
use sqlx::SqlitePool;
use tauri::Manager;
use tauri::{AppHandle};
use tokio_util::sync::CancellationToken;

#[tauri::command]
pub async fn start_transcription(
    app: AppHandle,
    stt_provider: String,
    audio_config: AudioCaptureConfig,
) -> Result<(), AppError> {
    tracing::trace!(stt_provider, ?audio_config, "start_transcription called");

    let mut guard = SESSION.lock().await;
    if guard.is_some() {
        return Err(AppError::SttProviderAlreadyRunning);
    }
    if !audio_config.capture_system_audio && !audio_config.capture_microphone {
        return Err(AppError::AtLeactOneAudioSourceMustBeEnabled);
    }
    let has_conversation = CURRENT_CONVERSATION_ID.lock().await;

    if !has_conversation.is_some() {
        return Err(AppError::TranscriptionConversationIdNotSet);
    }

    let pool = app.state::<SqlitePool>();
    let settings = stt_provider_repo::get(&pool, &stt_provider)
        .await?
        .ok_or_else(|| AppError::SttProviderNotConfigured)?;

    let provider = create_stt_provider(&settings);

    let cancellation_token = CancellationToken::new();
    let task = tokio::spawn({
        let app = app.clone();
        let token = cancellation_token.clone();
        async move {
            run_transcription(app, provider, audio_config, token).await;
        }
    });

    *guard = Some(TranscriptionSession {
        cancellation_token,
        task,
    });
    Ok(())
}
