use crate::db::llm_provider_settings as provider_repo;
use crate::db::message as msg_repo;
use crate::db::system_prompt as system_prompt_repo;
use crate::domain::conversations::ChatStreamEvent;
use crate::domain::llm::LlmChatCompletionRequest;
use crate::domain::llm::LlmChatCompletionStreamEvent;
use crate::domain::messages::Message;
use crate::domain::messages::MessageRole;
use crate::errors::AppError;
use crate::infra::llm_providers::create_llm_provider;
use crate::infra::screenshot::capture_screenshot as do_capture_screenshot;
use crate::infra::screenshot::ScreenshotResult;
use chrono::Utc;
use sqlx::SqlitePool;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::Manager;
use tauri::{AppHandle, Emitter};
use uuid::Uuid;

static STREAMING: AtomicBool = AtomicBool::new(false);

// AI Chat
#[tauri::command]
pub async fn send_message(
    app: AppHandle,
    provider: String,
    conversation_id: String,
    user_message: String,
    capture_screenshot: bool,
    system_prompt_id: Option<String>,
) -> Result<(), AppError> {
    tracing::trace!(
        provider = %provider,
        conversation_id = %conversation_id,
        capture_screenshot,
        system_prompt_id = ?system_prompt_id,
        user_message = %user_message,
        "send_message called"
    );

    if STREAMING.load(Ordering::SeqCst) {
        tracing::warn!("Already streaming, ignoring request");
        return Err(AppError::LlmAlreadyRunning);
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

    let pool = app.state::<SqlitePool>();
    if let Err(e) = msg_repo::create(
        &pool,
        &Message {
            id: Uuid::new_v4().to_string(),
            conversation_id: conversation_id.clone(),
            role: MessageRole::User,
            content: user_message.clone(),
            screenshot_path: screenshot_path.clone(),
            timestamp: Utc::now().timestamp(),
        },
    )
    .await
    {
        tracing::error!(error = %e, "Error saving user message");
    }

    let provider_settings = provider_repo::get(&pool, &provider)
        .await?
        .ok_or_else(|| AppError::LlmProviderNotConfigured)?;
    let llm_provider = create_llm_provider(&provider_settings)?;

    let history = msg_repo::get_by_conversation(&pool, &conversation_id).await?;

    let system_prompt_text = if let Some(ref id) = system_prompt_id {
        system_prompt_repo::get_by_id(&pool, &id)
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

    STREAMING.store(true, Ordering::SeqCst);

    let mut assistant_response = String::new();

    let stream_result = llm_provider.stream_chat_completion(request).await;

    match stream_result {
        Ok(mut stream) => {
            use futures_util::StreamExt;
            while let Some(event) = stream.next().await {
                if !STREAMING.load(Ordering::SeqCst) {
                    tracing::info!("Stream stopped by user");
                    break;
                }

                match event {
                    LlmChatCompletionStreamEvent::Chunk {
                        content,
                        is_finish,
                        usage,
                    } => {
                        assistant_response.push_str(&content);

                        let _ = app.emit(
                            "chat-stream",
                            ChatStreamEvent::Chunk {
                                conversation_id: conversation_id.clone(),
                                content: content,
                                is_finish: is_finish,
                                timestamp: Utc::now().timestamp(),
                                usage: usage,
                            },
                        );
                    }
                    LlmChatCompletionStreamEvent::Error { code, message } => {
                        tracing::error!(code = %code, message = %message, "Stream error");
                        let _ = app.emit(
                            "chat-stream",
                            ChatStreamEvent::Error {
                                code: code,
                                message: message,
                            },
                        );
                    }
                }
            }
        }
        Err(e) => {
            tracing::error!(error = %e, "Provider error");
        }
    }

    STREAMING.store(false, Ordering::SeqCst);

    tracing::trace!("Stream completed");

    if let Err(e) = msg_repo::create(
        &pool,
        &Message {
            id: Uuid::new_v4().to_string(),
            conversation_id: conversation_id.clone(),
            role: MessageRole::Assistant,
            content: assistant_response,
            screenshot_path: None,
            timestamp: Utc::now().timestamp(),
        },
    )
    .await
    {
        tracing::error!(error = %e, "Error saving assistant message");
    }

    Ok(())
}

#[tauri::command]
pub async fn stop_stream() -> Result<(), String> {
    tracing::trace!("stop_stream called");
    STREAMING.store(false, Ordering::SeqCst);
    Ok(())
}
