use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{domain::transcript::{AudioCaptureConfig, TranscriptResult}, errors::AppError};

pub enum TranscriptionCommand {
    Start {
        conversation_id: Uuid,
        stt_provider: String,
        audio_config: AudioCaptureConfig,
        reply: oneshot::Sender<Result<(), AppError>>
    },
    Stop {
        reply: oneshot::Sender<Result<(), AppError>>
    },
    GetState {
        reply: oneshot::Sender<Result<(), AppError>>
    },
    SwitchConversation {
        conversation_id: Uuid,
        reply: oneshot::Sender<Result<(), AppError>>
    }
}

pub enum TranscriptionStatus {
    Idle,
    Starting,
    Running,
    Stopping,
}

pub enum TranscriptionEvent {
    StatusUpdate {
        status: TranscriptionStatus
    },
    TranscriptionResult {
        trancript: TranscriptResult
    }
}
