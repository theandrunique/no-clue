use tauri::State;
use uuid::Uuid;

use crate::{
    application::transcriptions::{TranscriptionHandle, TranscriptionStatus},
    domain::transcript::AudioCaptureConfig,
    errors::AppError,
};

#[tauri::command]
pub async fn start_transcription(
    handle: State<'_, TranscriptionHandle>,
    stt_provider: String,
    audio_config: AudioCaptureConfig,
) -> Result<(), AppError> {
    tracing::trace!(stt_provider, ?audio_config, "start_transcription called");
    handle.start_transcription(stt_provider, audio_config).await
}

#[tauri::command]
pub async fn stop_transcription(handle: State<'_, TranscriptionHandle>) -> Result<(), AppError> {
    tracing::info!("stop_transcription called");
    handle.stop_transcription().await
}

#[tauri::command]
pub async fn update_transcription_session(
    conversation_id: Uuid,
    handle: State<'_, TranscriptionHandle>,
) -> Result<(), AppError> {
    tracing::trace!(%conversation_id, "update_transcription_session called");
    handle.switch_conversation(conversation_id).await
}

#[tauri::command]
pub async fn get_current_state(
    handle: State<'_, TranscriptionHandle>,
) -> Result<TranscriptionStatus, AppError> {
    tracing::trace!("get_current_state called");
    handle.get_current_status().await
}
