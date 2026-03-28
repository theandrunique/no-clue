use crate::ai_providers::{create_provider, AiRequest, AiStreamEvent};
use crate::db::ai_provider as provider_repo;
use crate::db::conversation as conv_repo;
use crate::db::message as msg_repo;
use crate::db::transcript as transcript_repo;
use crate::models::{ChatStreamEvent, Conversation, Message, MessageRole, Transcript};
use crate::screenshot::{capture_screenshot as do_capture_screenshot, ScreenshotResult};
use chrono::Utc;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Emitter};
use uuid::Uuid;

static STREAMING: AtomicBool = AtomicBool::new(false);

// Conversation management
#[tauri::command]
pub async fn create_conversation() -> Result<Conversation, String> {
    tracing::info!("create_conversation called");

    let title = "New conversation".to_string();
    let id = tokio::task::spawn_blocking(move || conv_repo::create(title))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;

    let conversation = tokio::task::spawn_blocking(move || conv_repo::get_by_id(&id))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;

    match conversation {
        Some(c) => {
            tracing::info!(conversation_id = %c.id, "Conversation created");
            Ok(c)
        }
        None => Err("Failed to get conversation".to_string()),
    }
}

#[tauri::command]
pub async fn get_conversations() -> Result<Vec<Conversation>, String> {
    tracing::debug!("get_conversations called");
    let conversations = tokio::task::spawn_blocking(|| conv_repo::get_all())
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
    Ok(conversations)
}

#[tauri::command]
pub async fn get_conversation(id: String) -> Result<Conversation, String> {
    tracing::debug!(conversation_id = %id, "get_conversation called");
    let id_clone = id.clone();
    let result = tokio::task::spawn_blocking(move || conv_repo::get_by_id(&id_clone))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Conversation not found".to_string())?;
    Ok(result)
}

// AI Chat
#[tauri::command]
pub async fn send_message(
    app: AppHandle,
    provider: String,
    conversation_id: String,
    user_message: String,
    capture_screenshot: bool,
) -> Result<(), String> {
    tracing::info!(
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
            Ok(result) => {
                tracing::debug!(path = %result.relative_path, "Screenshot captured");
                Some(result)
            }
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
            .map_err(|e| e.to_string())?
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("Provider '{}' not configured", provider))?;

    let ai_provider = create_provider(&provider_settings);

    // Get chat history for context
    let conv_id_for_history = conversation_id.clone();
    let history =
        tokio::task::spawn_blocking(move || msg_repo::get_by_conversation(&conv_id_for_history))
            .await
            .map_err(|e| e.to_string())?
            .map_err(|e| e.to_string())?;

    // Build AI request
    let mut request = AiRequest::new(history);

    // Add screenshot if captured (as base64)
    if let Some(b64) = screenshot_base64 {
        tracing::debug!(base64_length = %b64.len(), "Using screenshot base64");
        request = request.with_screenshot(b64);
    } else if capture_screenshot {
        tracing::warn!("capture_screenshot=true but no base64 available");
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

    tracing::info!("Stream completed");

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
    tracing::info!("stop_stream called");
    STREAMING.store(false, Ordering::SeqCst);
    Ok(())
}

#[tauri::command]
pub async fn get_messages(conversation_id: String) -> Result<Vec<Message>, String> {
    tracing::debug!(conversation_id = %conversation_id, "get_messages called");
    let messages =
        tokio::task::spawn_blocking(move || msg_repo::get_by_conversation(&conversation_id))
            .await
            .map_err(|e| e.to_string())?
            .map_err(|e| e.to_string())?;
    Ok(messages)
}

#[tauri::command]
pub async fn get_transcripts(conversation_id: String) -> Result<Vec<Transcript>, String> {
    tracing::debug!(conversation_id = %conversation_id, "get_transcripts called");
    let transcripts =
        tokio::task::spawn_blocking(move || transcript_repo::get_by_conversation(&conversation_id))
            .await
            .map_err(|e| e.to_string())?
            .map_err(|e| e.to_string())?;
    Ok(transcripts)
}

#[tauri::command]
pub async fn delete_conversation(id: String) -> Result<(), String> {
    tracing::info!(conversation_id = %id, "delete_conversation called");
    tokio::task::spawn_blocking(move || conv_repo::delete(&id))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
    Ok(())
}
