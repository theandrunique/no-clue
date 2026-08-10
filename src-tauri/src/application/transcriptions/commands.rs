use crate::db::stt_provider as stt_provider_repo;
use crate::db::transcript as transcript_repo;
use crate::domain::stt::{SttProvider, SttTranscriptResult};
use crate::domain::transcriptions::{AudioCaptureConfig, TranscriptionResult};
use crate::error::log_err;
use crate::infra::audio_capture::start_capture_pipeline;
use crate::infra::stt_providers::create_stt_provider;
use futures_util::StreamExt;
use std::sync::LazyLock;
use std::sync::Mutex as StdMutex;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

struct TranscriptionSession {
    cancellation_token: CancellationToken,
    task: JoinHandle<()>,
}

static SESSION: LazyLock<Mutex<Option<TranscriptionSession>>> = LazyLock::new(|| Mutex::new(None));
static CURRENT_CONVERSATION_ID: LazyLock<StdMutex<Option<String>>> =
    LazyLock::new(|| StdMutex::new(None));

#[tauri::command]
pub async fn update_transcription_session(conversation_id: String) -> Result<(), String> {
    tracing::trace!(conversation_id, "update_transcription_session called");
    let mut current = CURRENT_CONVERSATION_ID.lock().map_err(|e| e.to_string())?;
    *current = Some(conversation_id);
    Ok(())
}

#[tauri::command]
pub async fn start_transcription(
    app: AppHandle,
    stt_provider: String,
    audio_config: AudioCaptureConfig,
) -> Result<(), String> {
    let mut guard = SESSION.lock().await;
    if guard.is_some() {
        return Err("Transcription already running".to_string());
    }
    if !audio_config.capture_system_audio && !audio_config.capture_microphone {
        return Err("At least one audio source must be enabled".to_string());
    }
    if CURRENT_CONVERSATION_ID
        .lock()
        .map_err(|e| e.to_string())?
        .is_none()
    {
        return Err("No conversation ID set".to_string());
    }

    let settings =
        tokio::task::spawn_blocking(move || stt_provider_repo::get_stt_settings(&stt_provider))
            .await
            .map_err(|e| log_err(e, "get_stt_settings"))?
            .map_err(|e| log_err(e, "get_stt_settings"))?
            .ok_or_else(|| log_err("STT provider not configured", "get_stt_settings"))?;

    let mut provider =
        create_stt_provider(&settings).map_err(|e| log_err(e, "create_stt_provider"))?;

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

async fn run_transcription(
    app: AppHandle,
    mut provider: Box<dyn SttProvider>,
    config: AudioCaptureConfig,
    token: CancellationToken,
) {
    let audio = match start_capture_pipeline(&config, token.child_token()) {
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
            _ = token.cancelled() => break,
        }
    }

    finish(app).await;
}

async fn handle_result(app: &AppHandle, result: SttTranscriptResult) {
    let timestamp = chrono::Utc::now().timestamp_millis();
    let id = Uuid::new_v4().to_string();
    let conversation_id = CURRENT_CONVERSATION_ID
        .lock()
        .map(|guard| guard.clone())
        .unwrap_or(None)
        .unwrap_or_default();

    let payload = TranscriptionResult {
        id: id.clone(),
        conversation_id: conversation_id.clone(),
        text: result.text.clone(),
        is_final: result.is_final,
        confidence: result.confidence,
        source: result.source.clone(),
        timestamp,
    };

    let _ = app.emit("transcription-result", &payload);

    if result.is_final {
        let text = result.text;
        let confidence = result.confidence;
        let source = result.source;
        tokio::spawn(async move {
            let _ = tokio::task::spawn_blocking(move || {
                transcript_repo::create(id, conversation_id, source, text, confidence, timestamp)
            })
            .await;
        });
    }
}

async fn finish(app: AppHandle) {
    *SESSION.lock().await = None;
    let _ = app.emit("transcription-stopped", ());
}

#[tauri::command]
pub async fn stop_transcription(app: AppHandle) -> Result<(), String> {
    tracing::info!("stop_transcription called");
    let _ = app.emit("transcription-stopping", ());
    if let Some(session) = SESSION.lock().await.as_ref() {
        session.cancellation_token.cancel();
    }
    Ok(())
}
