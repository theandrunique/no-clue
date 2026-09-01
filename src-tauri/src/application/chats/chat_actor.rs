use std::sync::Arc;

use chrono::Utc;
use sqlx::SqlitePool;
use tauri::{AppHandle, Manager};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::{
    application::chats::generation::{build_generation_request, start_generation},
    domain::chats::{
        actor::{ChatActorCommand, ChatActorOutput, ChatActorState, ChatGenerationEvent},
        Message, MessageRole,
    },
    errors::AppError,
    infra::{
        db,
        screenshot::{
            capture_screenshot as do_capture_screenshot, read_screenshot_base64, ScreenshotResult,
        },
    },
};

pub struct ChatActor {
    app: AppHandle,
    conversation_id: Uuid,
    state: ChatActorState,
    assistant_message_id: Option<Uuid>,
    generated_content: String,
    output: Arc<dyn ChatActorOutput>,
    tx: mpsc::Sender<ChatActorCommand>,
    rx: mpsc::Receiver<ChatActorCommand>,
}

impl ChatActor {
    pub fn new(
        app: AppHandle,
        conversation_id: Uuid,
        output: Arc<dyn ChatActorOutput>,
    ) -> (Self, mpsc::Sender<ChatActorCommand>) {
        let (tx, rx) = mpsc::channel::<ChatActorCommand>(100);

        (
            Self {
                app,
                conversation_id,
                state: ChatActorState::Idle,
                assistant_message_id: None,
                generated_content: String::new(),
                output,
                tx: tx.clone(),
                rx,
            },
            tx,
        )
    }

    pub async fn run(mut self) {
        while let Some(command) = self.rx.recv().await {
            match command {
                ChatActorCommand::SendMessage {
                    provider,
                    capture_screenshot,
                    system_prompt_id,
                    user_message,
                    reply,
                } => {
                    tracing::trace!(
                        provider,
                        capture_screenshot,
                        ?system_prompt_id,
                        %user_message,
                        "Command 'SendMessage' received"
                    );
                    let result = self
                        .handle_send_message(
                            provider,
                            capture_screenshot,
                            system_prompt_id,
                            user_message,
                        )
                        .await;
                    let _ = reply.send(result);
                }
                ChatActorCommand::Regenerate {
                    provider,
                    capture_screenshot,
                    system_prompt_id,
                    user_message_id,
                    reply,
                } => {
                    tracing::trace!(
                        provider,
                        capture_screenshot,
                        ?system_prompt_id,
                        %user_message_id,
                        "Command 'Regenerate' received"
                    );
                    let result = self
                        .handle_regenerate(
                            provider,
                            capture_screenshot,
                            system_prompt_id,
                            user_message_id,
                        )
                        .await;
                    let _ = reply.send(result);
                }
                ChatActorCommand::StopGeneration { reply } => {
                    tracing::trace!("Command 'StopGeneration' received");
                    self.handle_stop_generation();
                    let _ = reply.send(Ok(()));
                }
                ChatActorCommand::GenerationEvent(event) => {
                    self.handle_generation_event(event).await;
                }
            }
        }
    }

    async fn handle_send_message(
        &mut self,
        provider: String,
        capture_screenshot: bool,
        system_prompt_id: Option<Uuid>,
        user_message: String,
    ) -> Result<Message, AppError> {
        if matches!(self.state, ChatActorState::Generation { .. }) {
            tracing::warn!("Already streaming, ignoring request");
            return Err(AppError::LlmProviderAlreadyRunning);
        }

        let pool = self.app.state::<SqlitePool>();

        let screenshot_result: Option<ScreenshotResult> = if capture_screenshot {
            match do_capture_screenshot(self.app.clone()) {
                Ok(result) => Some(result),
                Err(e) => {
                    tracing::error!(error = %e, "Failed to capture screenshot");
                    None
                }
            }
        } else {
            None
        };

        let screenshot_path = screenshot_result.as_ref().map(|r| r.relative_path.clone());
        let screenshot_base64 = screenshot_result.map(|r| r.base64);

        let message = Message {
            id: Uuid::new_v4(),
            conversation_id: self.conversation_id.clone(),
            role: MessageRole::User,
            content: user_message.clone(),
            screenshot_path,
            finish_reason: None,
            created_at: Utc::now(),
        };

        db::message::save(&pool, &message).await?;

        let assistant_message_id = Uuid::new_v4();
        self.assistant_message_id = Some(assistant_message_id);

        let (request, llm_provider) = build_generation_request(
            &self.app,
            &self.conversation_id,
            &provider,
            system_prompt_id,
            capture_screenshot,
            screenshot_base64,
        )
        .await?;

        let ct = CancellationToken::new();
        self.state = ChatActorState::Generation {
            cancel_token: ct.clone(),
        };

        let actor_tx = self.tx.clone();
        tokio::spawn({
            let token = ct.clone();
            async move {
                start_generation(request, llm_provider, token, actor_tx).await;
            }
        });

        Ok(message)
    }

    async fn handle_regenerate(
        &mut self,
        provider: String,
        capture_screenshot: bool,
        system_prompt_id: Option<Uuid>,
        user_message_id: Uuid,
    ) -> Result<(), AppError> {
        if matches!(self.state, ChatActorState::Generation { .. }) {
            tracing::warn!("Already streaming, ignoring request");
            return Err(AppError::LlmProviderAlreadyRunning);
        }

        let pool = self.app.state::<SqlitePool>();

        let user_message_row = db::message::get_by_id(&pool, &user_message_id)
            .await?
            .filter(|m| m.conversation_id == self.conversation_id)
            .ok_or(AppError::MessageNotFound)?;

        db::message::delete_after(&pool, &self.conversation_id, &user_message_id).await?;

        let screenshot_base64 = if capture_screenshot {
            match &user_message_row.screenshot_path {
                Some(path) => match read_screenshot_base64(&self.app, path) {
                    Ok(Some(b64)) => Some(b64),
                    Ok(None) => {
                        tracing::warn!(path, "Stored screenshot not found, capturing a new one");
                        capture_screenshot_opt(&self.app)
                    }
                    Err(e) => {
                        tracing::error!(error = %e, "Failed to read stored screenshot");
                        capture_screenshot_opt(&self.app)
                    }
                },
                None => capture_screenshot_opt(&self.app),
            }
        } else {
            None
        };

        let assistant_message_id = Uuid::new_v4();
        self.assistant_message_id = Some(assistant_message_id);

        let (request, llm_provider) = build_generation_request(
            &self.app,
            &self.conversation_id,
            &provider,
            system_prompt_id,
            capture_screenshot,
            screenshot_base64,
        )
        .await?;

        let ct = CancellationToken::new();
        self.state = ChatActorState::Generation {
            cancel_token: ct.clone(),
        };

        let actor_tx = self.tx.clone();
        tokio::spawn({
            let token = ct.clone();
            async move {
                start_generation(request, llm_provider, token, actor_tx).await;
            }
        });

        Ok(())
    }

    async fn handle_generation_event(&mut self, event: ChatGenerationEvent) {
        let assistant_message_id = match self.assistant_message_id {
            Some(id) => id,
            None => {
                tracing::error!("Received generation event but no assistant message id is set");
                return;
            }
        };

        match event {
            ChatGenerationEvent::Started => {
                self.generated_content.clear();
                self.output
                    .on_generation_start(self.conversation_id, assistant_message_id);
            }
            ChatGenerationEvent::Chunk { delta } => {
                self.generated_content.push_str(&delta);
                self.output
                    .on_generation_chunk(self.conversation_id, assistant_message_id, delta);
            }
            ChatGenerationEvent::Finished {
                finish_reason,
                usage,
            } => {
                let created_at = Utc::now();
                let pool = self.app.state::<SqlitePool>();
                if let Err(e) = db::message::save(
                    &pool,
                    &Message {
                        id: assistant_message_id,
                        conversation_id: self.conversation_id,
                        role: MessageRole::Assistant,
                        content: std::mem::take(&mut self.generated_content),
                        screenshot_path: None,
                        finish_reason: Some(finish_reason.clone()),
                        created_at,
                    },
                )
                .await
                {
                    tracing::error!(error = %e, "Error saving assistant message");
                }

                self.state = ChatActorState::Idle;
                self.output.on_generation_finish(
                    self.conversation_id,
                    assistant_message_id,
                    finish_reason,
                    created_at,
                    usage,
                );
            }
        }
    }

    fn handle_stop_generation(&self) {
        match &self.state {
            ChatActorState::Idle => {
                tracing::warn!("LLM provider was not running but stop was requested");
            }
            ChatActorState::Generation { cancel_token } => {
                cancel_token.cancel();
            }
        };
    }
}

fn capture_screenshot_opt(app: &AppHandle) -> Option<String> {
    match do_capture_screenshot(app.clone()) {
        Ok(result) => Some(result.base64),
        Err(e) => {
            tracing::error!(error = %e, "Failed to capture screenshot");
            None
        }
    }
}
