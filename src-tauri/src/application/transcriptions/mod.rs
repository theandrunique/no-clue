mod get_transcripts;
mod stt_providers;
mod transcription_actor;
mod transcription_handle;
mod worker;

pub use get_transcripts::get_transcripts;
use serde::Serialize;
pub use stt_providers::{get_stt_provider_settings, get_stt_providers, save_stt_provider_settings};
use tauri::{AppHandle, Emitter};
use tokio::sync::oneshot;
pub use transcription_handle::TranscriptionHandle;
use uuid::Uuid;

use crate::{
    application::transcriptions::worker::WorkerEvent,
    domain::transcript::{AudioCaptureConfig, TranscriptResult},
    errors::AppError,
};

pub trait TranscriptionOutput: Send + Sync + 'static {
    fn on_status_changed(&self, status: TranscriptionStatus);
    fn on_transcription_result(&self, result: &TranscriptResult);
    fn on_error(&self, error: String);
}

pub struct TauriTranscriptionOutput {
    pub app: AppHandle,
}

impl TranscriptionOutput for TauriTranscriptionOutput {
    fn on_status_changed(&self, status: TranscriptionStatus) {
        let _ = self.app.emit("transcription-status", status);
    }

    fn on_transcription_result(&self, result: &TranscriptResult) {
        let _ = self.app.emit("transcription-result", result);
    }

    fn on_error(&self, error: String) {
        let _ = self.app.emit("transcription-error", error);
    }
}

#[derive(Clone, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TranscriptionStatus {
    Idle,
    Starting,
    Running,
    Stopping,
}

pub enum TranscriptionActorCommand {
    Start {
        stt_provider: String,
        audio_config: AudioCaptureConfig,
        reply: oneshot::Sender<Result<(), AppError>>,
    },
    Stop {
        reply: oneshot::Sender<Result<(), AppError>>,
    },
    SwitchConversation {
        conversation_id: Uuid,
        reply: oneshot::Sender<Result<(), AppError>>,
    },
    GetState {
        reply: oneshot::Sender<TranscriptionStatus>,
    },
    WorkerUpdate(WorkerEvent),
}
