use crate::db::conversation as conversation_repo;
use crate::db::llm_provider_settings as provider_repo;
use crate::db::message as msg_repo;
use crate::db::system_prompt as system_prompt_repo;
use crate::domain::conversations::ChatStreamEvent;
use crate::domain::llm::LlmChatCompletionRequest;
use crate::domain::llm::LlmProvider;
use crate::domain::messages::{FinishReason, Message, MessageRole};
use crate::errors::AppError;
use crate::infra::llm_providers::create_llm_provider;
use crate::infra::screenshot::capture_screenshot as do_capture_screenshot;
use crate::infra::screenshot::read_screenshot_base64;
use crate::infra::screenshot::ScreenshotResult;
use chrono::Utc;
use futures_util::StreamExt;
use serde::Serialize;
use sqlx::SqlitePool;
use std::sync::LazyLock;
use tauri::Manager;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

static SESSION: LazyLock<Mutex<Option<CancellationToken>>> = LazyLock::new(|| Mutex::new(None));

#[derive(Serialize)]
pub struct SendMessageResult {
    pub user_message_id: Uuid,
}

#[tauri::command]
pub async fn send_message(
    app: AppHandle,
    provider: String,
    conversation_id: Uuid,
    user_message: String,
    capture_screenshot: bool,
    system_prompt_id: Option<Uuid>,
) -> Result<SendMessageResult, AppError> {
    tracing::trace!(
        provider,
        %conversation_id,
        %user_message,
        capture_screenshot,
        ?system_prompt_id,
        "send_message called"
    );

    let mut guard = SESSION.lock().await;
    if guard.is_some() {
        tracing::warn!("Already streaming, ignoring request");
        return Err(AppError::LlmProviderAlreadyRunning);
    }

    let pool = app.state::<SqlitePool>();

    if conversation_repo::get_by_id(&pool, &conversation_id)
        .await?
        .is_none()
    {
        return Err(AppError::ConversationNotFound);
    }

    let screenshot_result: Option<ScreenshotResult> = if capture_screenshot {
        match do_capture_screenshot(app.clone()) {
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

    let user_message_id = Uuid::new_v4();
    msg_repo::save(
        &pool,
        &Message {
            id: user_message_id,
            conversation_id: conversation_id.clone(),
            role: MessageRole::User,
            content: user_message.clone(),
            screenshot_path,
            finish_reason: None,
            created_at: Utc::now(),
        },
    )
    .await?;

    let token = start_generation(
        app,
        conversation_id,
        provider,
        system_prompt_id,
        capture_screenshot,
        screenshot_base64,
    )
    .await?;

    *guard = Some(token);

    Ok(SendMessageResult { user_message_id })
}

#[tauri::command]
pub async fn retry_generation(
    app: AppHandle,
    provider: String,
    conversation_id: Uuid,
    user_message_id: Uuid,
    capture_screenshot: bool,
    system_prompt_id: Option<Uuid>,
) -> Result<SendMessageResult, AppError> {
    tracing::trace!(
        provider,
        %conversation_id,
        %user_message_id,
        capture_screenshot,
        ?system_prompt_id,
        "retry_generation called"
    );

    let mut guard = SESSION.lock().await;
    if guard.is_some() {
        tracing::warn!("Already streaming, ignoring request");
        return Err(AppError::LlmProviderAlreadyRunning);
    }

    let pool = app.state::<SqlitePool>();

    if conversation_repo::get_by_id(&pool, &conversation_id)
        .await?
        .is_none()
    {
        return Err(AppError::ConversationNotFound);
    }

    let user_message_row = msg_repo::get_by_id(&pool, &user_message_id)
        .await?
        .filter(|m| m.conversation_id == conversation_id)
        .ok_or(AppError::MessageNotFound)?;

    msg_repo::delete_after(&pool, &conversation_id, &user_message_id).await?;

    let screenshot_base64 = if capture_screenshot {
        match &user_message_row.screenshot_path {
            Some(path) => match read_screenshot_base64(&app, path) {
                Ok(Some(b64)) => Some(b64),
                Ok(None) => {
                    tracing::warn!(
                        path,
                        "Stored screenshot not found, capturing a new one"
                    );
                    capture_screenshot_opt(&app)
                }
                Err(e) => {
                    tracing::error!(error = %e, "Failed to read stored screenshot");
                    capture_screenshot_opt(&app)
                }
            },
            None => capture_screenshot_opt(&app),
        }
    } else {
        None
    };

    let token = start_generation(
        app,
        conversation_id,
        provider,
        system_prompt_id,
        capture_screenshot,
        screenshot_base64,
    )
    .await?;

    *guard = Some(token);

    Ok(SendMessageResult { user_message_id })
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

async fn start_generation(
    app: AppHandle,
    conversation_id: Uuid,
    provider: String,
    system_prompt_id: Option<Uuid>,
    capture_screenshot: bool,
    screenshot_base64: Option<String>,
) -> Result<CancellationToken, AppError> {
    let pool = app.state::<SqlitePool>();

    let provider_settings = provider_repo::get(&pool, &provider)
        .await?
        .ok_or(AppError::LlmProviderNotConfigured)?;
    let llm_provider = create_llm_provider(&provider_settings)?;

    let history = msg_repo::get_by_conversation(&pool, &conversation_id).await?;

    let system_prompt_text = if let Some(ref id) = system_prompt_id {
        system_prompt_repo::get_by_id(&pool, id).await?.map(|x| x.prompt)
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

    let cancellation_token = CancellationToken::new();
    tokio::spawn({
        let app = app.clone();
        let token = cancellation_token.clone();
        async move {
            run_chat_completion(app, conversation_id, request, llm_provider, token).await;
        }
    });

    Ok(cancellation_token)
}

async fn run_chat_completion(
    app: AppHandle,
    conversation_id: Uuid,
    request: LlmChatCompletionRequest,
    llm_provider: Box<dyn LlmProvider>,
    ct: CancellationToken,
) {
    let mut assistant_response = String::new();
    let mut finish_reason = FinishReason::Done;

    let mut stream = match llm_provider.stream_chat_completion(request).await {
        Ok(stream) => Some(stream),
        Err(err) => {
            tracing::error!(error = ?err, "Provider error");
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

                            let _ = app.emit(
                                "chat-stream",
                                ChatStreamEvent::Chunk {
                                    conversation_id: conversation_id.clone(),
                                    content: chunk.content,
                                    is_finish: chunk.is_finish,
                                    usage: chunk.usage,
                                    timestamp: Utc::now(),
                                },
                            );
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

    save_assistant_message(&app, conversation_id, assistant_response, finish_reason.clone()).await;

    if let FinishReason::Error { message } = &finish_reason {
        let _ = app.emit(
            "chat-stream",
            ChatStreamEvent::Error {
                conversation_id: conversation_id.clone(),
                code: "generation_error".to_string(),
                message: message.clone(),
            },
        );
    }

    let _ = app.emit(
        "chat-stream",
        ChatStreamEvent::Chunk {
            conversation_id: conversation_id.clone(),
            content: String::new(),
            is_finish: true,
            usage: None,
            timestamp: Utc::now(),
        },
    );

    tracing::trace!(?finish_reason, "Stream completed");
    finish().await;
}

async fn save_assistant_message(
    app: &AppHandle,
    conversation_id: Uuid,
    content: String,
    finish_reason: FinishReason,
) {
    let should_save = match &finish_reason {
        FinishReason::Error { .. } => true,
        FinishReason::Done | FinishReason::Cancelled => !content.is_empty(),
    };

    if !should_save {
        tracing::trace!(
            ?finish_reason,
            content_len = content.len(),
            "Not saving empty assistant message"
        );
        return;
    }

    let pool = app.state::<SqlitePool>();
    if let Err(e) = msg_repo::save(
        &pool,
        &Message {
            id: Uuid::new_v4(),
            conversation_id,
            role: MessageRole::Assistant,
            content,
            screenshot_path: None,
            finish_reason: Some(finish_reason),
            created_at: Utc::now(),
        },
    )
    .await
    {
        tracing::error!(error = %e, "Error saving assistant message");
    }
}

async fn finish() {
    *SESSION.lock().await = None;
}

#[tauri::command]
pub async fn stop_stream() -> Result<(), AppError> {
    tracing::trace!("stop_stream called");

    if let Some(session) = SESSION.lock().await.as_ref() {
        session.cancel();
    } else {
        tracing::warn!("LLM provider was not running but stop was requested");
    }

    Ok(())
}
