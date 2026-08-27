use futures_util::StreamExt;
use sqlx::SqlitePool;
use tauri::{AppHandle, Manager};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::{
    application::chats::chat_actor::{ChatActorCommand, ChatGenerationEvent},
    domain::chat::{FinishReason, TokenUsage},
    domain::llm::{LlmChatCompletionRequest, LlmProvider},
    errors::AppError,
    infra::{db, llm_providers::create_llm_provider},
};

pub async fn start_generation(
    request: LlmChatCompletionRequest,
    llm_provider: Box<dyn LlmProvider>,
    token: CancellationToken,
    actor_tx: mpsc::Sender<ChatActorCommand>,
) {
    let _ = actor_tx
        .send(ChatActorCommand::GenerationEvent(
            ChatGenerationEvent::Started,
        ))
        .await;

    run_chat_completion(request, llm_provider, actor_tx, token).await;
}

pub async fn build_generation_request(
    app: &AppHandle,
    conversation_id: &Uuid,
    provider: &str,
    system_prompt_id: Option<Uuid>,
    capture_screenshot: bool,
    screenshot_base64: Option<String>,
) -> Result<(LlmChatCompletionRequest, Box<dyn LlmProvider>), AppError> {
    let pool = app.state::<SqlitePool>();

    let provider_settings = db::llm_provider_settings::get(&pool, provider)
        .await?
        .ok_or(AppError::LlmProviderNotConfigured)?;

    let llm_provider = create_llm_provider(&provider_settings)?;

    let history = db::message::get_by_conversation(&pool, conversation_id).await?;

    let system_prompt_text = if let Some(ref id) = system_prompt_id {
        db::system_prompt::get_by_id(&pool, id)
            .await?
            .map(|x| x.prompt)
    } else {
        None
    };

    let mut request = LlmChatCompletionRequest::new(history);

    if let Some(sp) = system_prompt_text {
        request = request.with_system_prompt(sp);
    }

    if let Some(b64) = screenshot_base64 {
        request = request.with_screenshot(b64);
    } else if capture_screenshot {
        tracing::error!("capture_screenshot=true but no base64 available");
    }

    Ok((request, llm_provider))
}

async fn run_chat_completion(
    request: LlmChatCompletionRequest,
    llm_provider: Box<dyn LlmProvider>,
    actor_tx: mpsc::Sender<ChatActorCommand>,
    ct: CancellationToken,
) {
    let mut finish_reason = FinishReason::Done;
    let mut usage: Option<TokenUsage> = None;

    let mut stream = match llm_provider.stream_chat_completion(request).await {
        Ok(stream) => Some(stream),
        Err(err) => {
            tracing::error!(error = ?err, "Provider stream chat completion start error");
            finish_reason = FinishReason::Error {
                message: err.to_string(),
            };
            None
        }
    };

    if let Some(stream) = stream.as_mut() {
        loop {
            tokio::select! {
                event = stream.next() => {
                    match event {
                        Some(Ok(chunk)) => {
                            if chunk.usage.is_some() {
                                usage = chunk.usage.clone();
                            }
                            let _ = actor_tx
                                .send(ChatActorCommand::GenerationEvent(ChatGenerationEvent::Chunk {
                                    delta: chunk.content,
                                }))
                                .await;

                            if chunk.is_finish {
                                break;
                            }
                        }
                        Some(Err(e)) => {
                            tracing::error!(error = ?e, "LLM provider stream error");
                            finish_reason = FinishReason::Error {
                                message: e.to_string(),
                            };
                            break;
                        }
                        None => break,
                    }
                }
                _ = ct.cancelled() => {
                    finish_reason = FinishReason::Cancelled;
                    break;
                }
            }
        }
    }

    let _ = actor_tx
        .send(ChatActorCommand::GenerationEvent(
            ChatGenerationEvent::Finished {
                finish_reason,
                usage,
            },
        ))
        .await;
}
