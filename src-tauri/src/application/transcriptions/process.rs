use crate::application::transcriptions::{CURRENT_CONVERSATION_ID, finish};
use crate::db::transcript as transcript_repo;
use crate::domain::events;
use crate::domain::stt::{SttProvider, SttTranscriptResult};
use crate::domain::transcripts::Transcript;
use crate::domain::transcripts::{AudioCaptureConfig, TranscriptResult};
use crate::infra::audio_capture::start_capture_pipeline;
use chrono::Utc;
use futures_util::StreamExt;
use sqlx::SqlitePool;
use tauri::Manager;
use tauri::{AppHandle, Emitter};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

pub async fn run_transcription(
    app: AppHandle,
    mut provider: Box<dyn SttProvider>,
    config: AudioCaptureConfig,
    ct: CancellationToken,
) {
    let audio = match start_capture_pipeline(&config, ct.child_token()) {
        Ok(stream) => stream,
        Err(e) => {
            tracing::error!(error = %e, "Failed to start audio capture");
            return finish(app).await;
        }
    };

    let mut results = match provider.transcribe(audio).await {
        Ok(stream) => stream,
        Err(e) => {
            tracing::error!(error = %e, "Failed to start transcription session");
            return finish(app).await;
        }
    };

    loop {
        tokio::select! {
            Some(result) = results.next() => {
                handle_result(&app, result).await;
            }
            _ = ct.cancelled() => break,
        }
    }

    finish(app).await;
}

async fn handle_result(app: &AppHandle, result: SttTranscriptResult) {
    tracing::trace!(?result, "STT transcription result");
    let pool = app.state::<SqlitePool>();

    let now = Utc::now();
    let id = Uuid::new_v4();
    let conversation_id = CURRENT_CONVERSATION_ID.lock().await.unwrap_or_default();

    let payload = TranscriptResult {
        id: id,
        conversation_id: conversation_id,
        text: result.text.clone(),
        is_final: result.is_final,
        confidence: result.confidence,
        source: result.source.clone(),
        created_at: now,
    };

    let _ = app.emit(events::TRANSCRIPTION_RESULT, &payload);

    if result.is_final {
        if let Err(e) = transcript_repo::save(&pool, &Transcript::from(payload)).await {
            tracing::error!(error = %e, "Failed to save trancription");
        }
    }
}
