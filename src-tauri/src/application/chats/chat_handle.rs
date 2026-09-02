use std::sync::Arc;

use tauri::AppHandle;
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use crate::{
    application::chats::chat_actor::ChatActor,
    domain::chats::actor::{ChatActorCommand, TauriChatActorOutput},
    domain::chats::{LlmSettings, Message},
    errors::AppError,
};

#[derive(Clone)]
pub struct ChatHandle {
    tx: mpsc::Sender<ChatActorCommand>,
}

impl ChatHandle {
    pub fn new(app: AppHandle, conversation_id: Uuid) -> Self {
        let (actor, tx) = ChatActor::new(
            app.clone(),
            conversation_id,
            Arc::new(TauriChatActorOutput { app }),
        );

        tauri::async_runtime::spawn(async move {
            actor.run().await;
        });

        Self { tx }
    }

    pub async fn send_message(
        &self,
        model: LlmSettings,
        capture_screenshot: bool,
        system_prompt_id: Option<Uuid>,
        user_message: String,
    ) -> Result<Message, AppError> {
        let (tx, rx) = oneshot::channel();
        let _ = self
            .tx
            .send(ChatActorCommand::SendMessage {
                model,
                capture_screenshot,
                system_prompt_id,
                user_message,
                reply: tx,
            })
            .await;
        rx.await.map_err(|_| AppError::ChatActorDead)?
    }

    pub async fn regenerate(
        &self,
        model: LlmSettings,
        capture_screenshot: bool,
        system_prompt_id: Option<Uuid>,
        user_message_id: Uuid,
    ) -> Result<(), AppError> {
        let (tx, rx) = oneshot::channel();
        let _ = self
            .tx
            .send(ChatActorCommand::Regenerate {
                model,
                capture_screenshot,
                system_prompt_id,
                user_message_id,
                reply: tx,
            })
            .await;
        rx.await.map_err(|_| AppError::ChatActorDead)?
    }

    pub async fn stop_generation(&self) -> Result<(), AppError> {
        let (tx, rx) = oneshot::channel();
        let _ = self
            .tx
            .send(ChatActorCommand::StopGeneration { reply: tx })
            .await;
        rx.await.map_err(|_| AppError::ChatActorDead)?
    }
}
