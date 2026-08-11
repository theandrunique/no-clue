use crate::db::stt_provider_settings as stt_provider_repo;
use crate::db::transcript as transcript_repo;
use crate::domain::stt::{SttProvider, SttTranscriptResult};
use crate::domain::transcriptions::Transcript;
use crate::domain::transcriptions::{AudioCaptureConfig, TranscriptionResult};
use crate::errors::AppError;
use crate::infra::audio_capture::start_capture_pipeline;
use crate::infra::stt_providers::create_stt_provider;
use futures_util::StreamExt;
use sqlx::SqlitePool;
use std::sync::LazyLock;
use std::sync::Mutex as StdMutex;
use tauri::Manager;
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
) -> Result<(), AppError> {
    let mut guard = SESSION.lock().await;
    if guard.is_some() {
        return Err(AppError::TranscriptionAlreadyRunning);
    }
    if !audio_config.capture_system_audio && !audio_config.capture_microphone {
        return Err(AppError::AtLeactOneAudioSourceMustBeEnabled);
    }
    let has_conversation = CURRENT_CONVERSATION_ID
        .lock()
        .map(|guard| guard.is_some())
        .unwrap_or(false);
    if !has_conversation {
        return Err(AppError::TranscriptionConversationIdNotSet);
    }

    let pool = app.state::<SqlitePool>();
    let settings = stt_provider_repo::get(&pool, &stt_provider)
        .await?
        .ok_or_else(|| AppError::SttProviderNotConfigured)?;

    let mut provider = create_stt_provider(&settings);

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
    let pool = app.state::<SqlitePool>();

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
        if let Err(e) = transcript_repo::create(&pool, &Transcript::from(payload)).await {
            tracing::error!(error = %e, "Failed to save trancription");
        }
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
