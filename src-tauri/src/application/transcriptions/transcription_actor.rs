use std::sync::Arc;

use chrono::Utc;
use sqlx::SqlitePool;
use tauri::{AppHandle, Manager};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::{
    application::transcriptions::{
        worker::{self, WorkerEvent},
        TranscriptionActorCommand, TranscriptionOutput, TranscriptionStatus,
    },
    domain::transcript::{AudioCaptureConfig, Transcript, TranscriptResult},
    errors::AppError,
    infra::{
        db::{self},
        stt_providers::create_stt_provider,
    },
};

pub struct TranscriptionActor {
    app: AppHandle,
    status: TranscriptionStatus,
    current_conversation_id: Option<Uuid>,
    cancellation_token: Option<CancellationToken>,
    output: Arc<dyn TranscriptionOutput>,
    rx: mpsc::Receiver<TranscriptionActorCommand>,
    tx: mpsc::Sender<TranscriptionActorCommand>,
}

impl TranscriptionActor {
    pub fn new(
        app: AppHandle,
        output: Arc<dyn TranscriptionOutput>,
    ) -> (Self, mpsc::Sender<TranscriptionActorCommand>) {
        let (tx, rx) = mpsc::channel::<TranscriptionActorCommand>(100);

        (
            Self {
                app,
                status: TranscriptionStatus::Idle,
                current_conversation_id: None,
                cancellation_token: None,
                tx: tx.clone(),
                rx,
                output,
            },
            tx,
        )
    }

    pub async fn run(mut self) {
        while let Some(command) = self.rx.recv().await {
            match command {
                TranscriptionActorCommand::Start {
                    stt_provider,
                    audio_config,
                    reply,
                } => {
                    tracing::trace!(?audio_config, stt_provider, "Command 'Start' received");
                    let result = self.handle_start(stt_provider, audio_config).await;
                    let _ = reply.send(result);
                }
                TranscriptionActorCommand::Stop { reply } => {
                    tracing::trace!("Command 'Stop' received");
                    self.handle_stop().await;
                    let _ = reply.send(Ok(()));
                }
                TranscriptionActorCommand::GetState { reply } => {
                    tracing::trace!("Command 'GetState' received");
                    let _ = reply.send(self.status.clone());
                }
                TranscriptionActorCommand::SwitchConversation {
                    conversation_id,
                    reply,
                } => {
                    tracing::trace!(%conversation_id, "Command 'SwitchConversation' received");
                    self.current_conversation_id = Some(conversation_id);
                    let _ = reply.send(Ok(()));
                }
                TranscriptionActorCommand::WorkerUpdate(event) => {
                    self.handle_worker_event(event).await;
                }
            }
        }
    }

    pub async fn handle_worker_event(&mut self, event: WorkerEvent) {
        match event {
            WorkerEvent::Started => {
                tracing::info!("Transcription started");
                self.status = TranscriptionStatus::Running;
                self.output.on_status_changed(self.status.clone());
            }
            WorkerEvent::Result(result) => {
                tracing::trace!(?result, "Transcription result received from worker");

                let payload = TranscriptResult {
                    id: Uuid::new_v4(),
                    conversation_id: self.current_conversation_id.unwrap_or_default(),
                    text: result.text.clone(),
                    is_final: result.is_final,
                    confidence: result.confidence,
                    source: result.source.clone(),
                    created_at: Utc::now(),
                };

                self.output.on_transcription_result(&payload);

                if result.is_final {
                    let pool = self.app.state::<SqlitePool>();
                    if let Err(e) = db::transcript::save(&pool, &Transcript::from(payload)).await {
                        tracing::error!(error = %e, "Failed to save trancription");
                    }
                }
            }
            WorkerEvent::Error(e) => {
                tracing::error!(?e, "Worker error");
                self.status = TranscriptionStatus::Idle;
                self.output.on_status_changed(self.status.clone());
                self.cancellation_token = None;
                self.output.on_error(e);
            }
            WorkerEvent::Finished => {
                tracing::info!("Worker finished cleanly");
                self.status = TranscriptionStatus::Idle;
                self.cancellation_token = None;
                self.current_conversation_id = None;
                self.output.on_status_changed(self.status.clone());
            }
        }
    }

    pub async fn handle_start(
        &mut self,
        stt_provider: String,
        audio_config: AudioCaptureConfig,
    ) -> Result<(), AppError> {
        if self.status != TranscriptionStatus::Idle {
            return Err(AppError::TranscriptionAlreadyRunning);
        }
        if !audio_config.capture_system_audio && !audio_config.capture_microphone {
            return Err(AppError::AtLeactOneAudioSourceMustBeEnabled);
        }
        if self.current_conversation_id.is_none() {
            return Err(AppError::TranscriptionConversationIdNotSet);
        }

        let pool = self.app.state::<SqlitePool>();
        let settings = db::stt_provider_settings::get(&pool, &stt_provider)
            .await?
            .ok_or_else(|| AppError::SttProviderNotConfigured)?;

        let provider = create_stt_provider(&settings);

        self.status = TranscriptionStatus::Starting;
        self.output.on_status_changed(self.status.clone());

        let ct = CancellationToken::new();
        self.cancellation_token = Some(ct.clone());

        let actor_tx = self.tx.clone();

        tokio::spawn(async move {
            worker::run_transcription(provider, audio_config, actor_tx, ct).await;
        });

        Ok(())
    }

    pub async fn handle_stop(&mut self) {
        if let Some(token) = &self.cancellation_token {
            self.status = TranscriptionStatus::Stopping;
            self.output.on_status_changed(self.status.clone());
            token.cancel();
        } else {
            tracing::warn!("Transcription was not running but stop was requested");
        }
    }
}
