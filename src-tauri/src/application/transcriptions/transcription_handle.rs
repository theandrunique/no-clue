use std::sync::Arc;

use tauri::AppHandle;
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use crate::{
    application::transcriptions::{
        transcription_actor::TranscriptionActor, TauriTranscriptionOutput,
        TranscriptionActorCommand, TranscriptionStatus,
    },
    domain::transcript::AudioCaptureConfig,
    errors::AppError,
};

pub struct TranscriptionHandle {
    tx: mpsc::Sender<TranscriptionActorCommand>,
}

impl TranscriptionHandle {
    pub fn new(app: AppHandle) -> Self {
        let (actor, tx) =
            TranscriptionActor::new(app.clone(), Arc::new(TauriTranscriptionOutput { app }));

        tauri::async_runtime::spawn(async move {
            actor.run().await;
        });

        Self { tx }
    }

    pub async fn start_transcription(
        &self,
        stt_provider: String,
        audio_config: AudioCaptureConfig,
    ) -> Result<(), AppError> {
        let (tx, rx) = oneshot::channel();
        let _ = self
            .tx
            .send(TranscriptionActorCommand::Start {
                stt_provider,
                audio_config,
                reply: tx,
            })
            .await;
        rx.await.map_err(|_| AppError::TranscriptionActorDead)?
    }

    pub async fn stop_transcription(&self) -> Result<(), AppError> {
        let (tx, rx) = oneshot::channel();
        let _ = self
            .tx
            .send(TranscriptionActorCommand::Stop { reply: tx })
            .await;
        rx.await.map_err(|_| AppError::TranscriptionActorDead)?
    }

    pub async fn switch_conversation(&self, conversation_id: Uuid) -> Result<(), AppError> {
        let (tx, rx) = oneshot::channel();
        let _ = self
            .tx
            .send(TranscriptionActorCommand::SwitchConversation {
                conversation_id,
                reply: tx,
            })
            .await;
        rx.await.map_err(|_| AppError::TranscriptionActorDead)?
    }

    pub async fn get_current_status(&self) -> Result<TranscriptionStatus, AppError> {
        let (tx, rx) = oneshot::channel();
        let _ = self
            .tx
            .send(TranscriptionActorCommand::GetState { reply: tx })
            .await;
        rx.await.map_err(|_| AppError::TranscriptionActorDead)
    }
}
