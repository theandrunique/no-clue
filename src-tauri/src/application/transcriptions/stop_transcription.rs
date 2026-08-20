use tauri::{AppHandle, Emitter};

use crate::{application::transcriptions::SESSION, domain::events};

#[tauri::command]
pub async fn stop_transcription(app: AppHandle) -> Result<(), String> {
    tracing::info!("stop_transcription called");
    if let Some(session) = SESSION.lock().await.as_ref() {
        let _ = app.emit(events::TRANSCRIPTION_STOPPING, ());
        session.cancellation_token.cancel();
    } else {
        tracing::warn!("Transcription was not running but stop was requested");
    }
    Ok(())
}
