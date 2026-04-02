use crate::ai_providers::{create_ai_provider, AiRequest, AiStreamEvent};
use crate::db::ai_provider as provider_repo;
use crate::db::message as msg_repo;
use crate::error::log_err;
use crate::models::{ChatStreamEvent, MessageRole};
use crate::screenshot::{capture_screenshot as do_capture_screenshot, ScreenshotResult};
use chrono::Utc;
use std::sync::atomic::{AtomicBool, Ordering};
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
) -> Result<(), String> {
    tracing::trace!(
        provider = %provider,
        conversation_id = %conversation_id,
        capture_screenshot,
        user_message = %user_message,
        "send_message called"
    );

    if STREAMING.load(Ordering::SeqCst) {
        tracing::warn!("Already streaming, ignoring request");
        return Err("Already streaming".to_string());
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

    // Save user message immediately
    let conv_id_clone = conversation_id.clone();
    let user_msg_clone = user_message.clone();
    let screenshot_clone = screenshot_path.clone();
    if let Err(e) = tokio::task::spawn_blocking(move || {
        msg_repo::create(
            conv_id_clone,
            Uuid::new_v4().to_string(),
            MessageRole::User,
            user_msg_clone,
            screenshot_clone,
            Utc::now().timestamp(),
        )
    })
    .await
    {
        tracing::error!(error = %e, "Error saving user message");
    }

    // Get provider settings and create provider instance
    let provider_clone = provider.clone();
    let provider_settings =
        tokio::task::spawn_blocking(move || provider_repo::get_provider_settings(&provider_clone))
            .await
            .map_err(|e| log_err(e, "get_provider_settings"))?
            .map_err(|e| log_err(e, "get_provider_settings"))?
            .ok_or_else(|| {
                log_err(
                    format!("Provider '{}' not configured", provider),
                    "get_provider_settings",
                )
            })?;

    let ai_provider =
        create_ai_provider(&provider_settings).map_err(|e| log_err(e, "create_provider"))?;

    // Get chat history for context
    let conv_id_for_history = conversation_id.clone();
    let history =
        tokio::task::spawn_blocking(move || msg_repo::get_by_conversation(&conv_id_for_history))
            .await
            .map_err(|e| log_err(e, "get_chat_history"))?
            .map_err(|e| log_err(e, "get_chat_history"))?;

    // Build AI request
    let mut request = AiRequest::new(history);

    // Add screenshot if captured (as base64)
    if let Some(b64) = screenshot_base64 {
        request = request.with_screenshot(b64);
    } else if capture_screenshot {
        tracing::error!("capture_screenshot=true but no base64 available");
    }

    STREAMING.store(true, Ordering::SeqCst);

    let mut assistant_response = String::new();

    // Stream from provider
    let stream_result = ai_provider.stream(request).await;

    match stream_result {
        Ok(mut stream) => {
            use futures_util::StreamExt;
            while let Some(event) = stream.next().await {
                if !STREAMING.load(Ordering::SeqCst) {
                    tracing::info!("Stream stopped by user");
                    break;
                }

                match event {
                    AiStreamEvent::Chunk { content, is_finish } => {
                        assistant_response.push_str(&content);

                        let _ = app.emit(
                            "chat-stream",
                            ChatStreamEvent::Chunk {
                                conversation_id: conversation_id.clone(),
                                content: content,
                                is_finish: is_finish,
                                timestamp: Utc::now().timestamp(),
                            },
                        );

                        if is_finish {
                            break;
                        }
                    }
                    AiStreamEvent::Error { code, message } => {
                        tracing::error!(code = %code, message = %message, "Stream error");
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

    // Save assistant response
    let conv_id_for_save = conversation_id.clone();
    let assistant_final = assistant_response.clone();
    if let Err(e) = tokio::task::spawn_blocking(move || {
        msg_repo::create(
            conv_id_for_save,
            Uuid::new_v4().to_string(),
            MessageRole::Assistant,
            assistant_final,
            None,
            Utc::now().timestamp(),
        )
    })
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
