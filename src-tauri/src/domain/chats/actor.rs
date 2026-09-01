use chrono::{DateTime, Utc};
use tauri::{AppHandle, Emitter};
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::{
    domain::chats::{ChatStreamEvent, FinishReason, Message, TokenUsage},
    errors::AppError,
};

pub trait ChatActorOutput: Send + Sync + 'static {
    fn on_generation_start(&self, conversation_id: Uuid, message_id: Uuid);
    fn on_generation_chunk(&self, conversation_id: Uuid, message_id: Uuid, delta: String);
    fn on_generation_finish(
        &self,
        conversation_id: Uuid,
        message_id: Uuid,
        finish_reason: FinishReason,
        created_at: DateTime<Utc>,
        usage: Option<TokenUsage>,
    );
}

pub struct TauriChatActorOutput {
    pub app: AppHandle,
}

impl ChatActorOutput for TauriChatActorOutput {
    fn on_generation_start(&self, conversation_id: Uuid, message_id: Uuid) {
        let _ = self.app.emit(
            "chat-stream",
            ChatStreamEvent::Start {
                message_id,
                conversation_id,
            },
        );
    }

    fn on_generation_chunk(&self, conversation_id: Uuid, message_id: Uuid, delta: String) {
        let _ = self.app.emit(
            "chat-stream",
            ChatStreamEvent::Chunk {
                message_id,
                conversation_id,
                delta,
            },
        );
    }

    fn on_generation_finish(
        &self,
        conversation_id: Uuid,
        message_id: Uuid,
        finish_reason: FinishReason,
        created_at: DateTime<Utc>,
        usage: Option<TokenUsage>,
    ) {
        let _ = self.app.emit(
            "chat-stream",
            ChatStreamEvent::Finish {
                message_id,
                conversation_id,
                finish_reason,
                created_at,
                usage,
            },
        );
    }
}

#[derive(PartialEq)]
pub enum ChatActorState {
    Idle,
    Generation { cancel_token: CancellationToken },
}

pub enum ChatActorCommand {
    SendMessage {
        provider: String,
        capture_screenshot: bool,
        system_prompt_id: Option<Uuid>,
        user_message: String,
        reply: oneshot::Sender<Result<Message, AppError>>,
    },
    Regenerate {
        provider: String,
        capture_screenshot: bool,
        system_prompt_id: Option<Uuid>,
        user_message_id: Uuid,
        reply: oneshot::Sender<Result<(), AppError>>,
    },
    StopGeneration {
        reply: oneshot::Sender<Result<(), AppError>>,
    },
    GenerationEvent(ChatGenerationEvent),
}

pub enum ChatGenerationEvent {
    Started,
    Chunk {
        delta: String,
    },
    Finished {
        finish_reason: FinishReason,
        usage: Option<TokenUsage>,
    },
}
