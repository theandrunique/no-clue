use crate::application::chats::SESSION;
use crate::db::llm_provider_settings as provider_repo;
use crate::db::message as msg_repo;
use crate::db::system_prompt as system_prompt_repo;
use crate::domain::conversations::ChatStreamEvent;
use crate::domain::llm::LlmChatCompletionRequest;
use crate::domain::llm::LlmProvider;
use crate::domain::messages::{FinishReason, Message, MessageRole, TokenUsage};
use crate::errors::AppError;
use crate::infra::llm_providers::create_llm_provider;
use chrono::Utc;
use futures_util::StreamExt;
use sqlx::SqlitePool;
use tauri::Manager;
use tauri::{AppHandle, Emitter};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

pub async fn start_generation(
    app: AppHandle,
    conversation_id: Uuid,
    provider: String,
    system_prompt_id: Option<Uuid>,
    capture_screenshot: bool,
    screenshot_base64: Option<String>,
    token: CancellationToken,
) {
    let assistant_message_id = Uuid::new_v4();

    let _ = app.emit(
        "chat-stream",
        ChatStreamEvent::Start {
            message_id: assistant_message_id,
            conversation_id,
        },
    );

    let (request, llm_provider) = match build_generation_request(
        &app,
        &conversation_id,
        &provider,
        system_prompt_id,
        capture_screenshot,
        screenshot_base64,
    )
    .await
    {
        Ok(setup) => setup,
        Err(error) => {
            tracing::error!(%error, "Failed to build generation request");
            emit_generation_error(&app, conversation_id, assistant_message_id, error.to_string()).await;
            *SESSION.lock().await = None;
            return;
        }
    };

    run_chat_completion(
        &app,
        conversation_id,
        assistant_message_id,
        request,
        llm_provider,
        token,
    )
    .await;

    *SESSION.lock().await = None;
}

async fn build_generation_request(
    app: &AppHandle,
    conversation_id: &Uuid,
    provider: &str,
    system_prompt_id: Option<Uuid>,
    capture_screenshot: bool,
    screenshot_base64: Option<String>,
) -> Result<(LlmChatCompletionRequest, Box<dyn LlmProvider>), AppError> {
    let pool = app.state::<SqlitePool>();

    let provider_settings = provider_repo::get(&pool, provider)
        .await?
        .ok_or(AppError::LlmProviderNotConfigured)?;

    let llm_provider = create_llm_provider(&provider_settings)?;

    let history = msg_repo::get_by_conversation(&pool, conversation_id).await?;

    let system_prompt_text = if let Some(ref id) = system_prompt_id {
        system_prompt_repo::get_by_id(&pool, id)
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

async fn emit_generation_error(
    app: &AppHandle,
    conversation_id: Uuid,
    message_id: Uuid,
    message: String,
) {
    let _ = app.emit(
        "chat-stream",
        ChatStreamEvent::Finish {
            message_id,
            conversation_id,
            finish_reason: FinishReason::Error {
                message: message.clone(),
            },
            created_at: Utc::now(),
            usage: None,
        },
    );

    let pool = app.state::<SqlitePool>();
    if let Err(e) = msg_repo::save(
        &pool,
        &Message {
            id: message_id,
            conversation_id,
            role: MessageRole::Assistant,
            content: String::new(),
            screenshot_path: None,
            finish_reason: Some(FinishReason::Error { message }),
            created_at: Utc::now(),
        },
    )
    .await
    {
        tracing::error!(error = %e, "Error saving assistant message");
    }
}

async fn run_chat_completion(
    app: &AppHandle,
    conversation_id: Uuid,
    assistant_message_id: Uuid,
    request: LlmChatCompletionRequest,
    llm_provider: Box<dyn LlmProvider>,
    ct: CancellationToken,
) {
    let mut assistant_response = String::new();
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
                            assistant_response.push_str(&chunk.content);

                            if chunk.usage.is_some() {
                                usage = chunk.usage.clone();
                            }

                            let _ = app.emit(
                                "chat-stream",
                                ChatStreamEvent::Chunk {
                                    message_id: assistant_message_id,
                                    conversation_id,
                                    delta: chunk.content,
                                },
                            );

                            if chunk.is_finish {
                                break;
                            }
                        },
                        Some(Err(e)) => {
                            tracing::error!(error = ?e, "LLM provider stream error");
                            finish_reason = FinishReason::Error {
                                message: e.to_string(),
                            };
                            break;
                        },
                        None => break,
                    }
                },
                _ = ct.cancelled() => {
                    finish_reason = FinishReason::Cancelled;
                    break;
                }
            }
        }
    }

    let pool = app.state::<SqlitePool>();
    if let Err(e) = msg_repo::save(
        &pool,
        &Message {
            id: assistant_message_id,
            conversation_id,
            role: MessageRole::Assistant,
            content: assistant_response,
            screenshot_path: None,
            finish_reason: Some(finish_reason.clone()),
            created_at: Utc::now(),
        },
    )
    .await
    {
        tracing::error!(error = %e, "Error saving assistant message");
    }

    tracing::trace!(?finish_reason, "Stream completed");

    let _ = app.emit(
        "chat-stream",
        ChatStreamEvent::Finish {
            message_id: assistant_message_id,
            conversation_id,
            finish_reason,
            created_at: Utc::now(),
            usage,
        },
    );
}
