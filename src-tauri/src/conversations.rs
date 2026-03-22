use crate::ai_providers::AiStreamEvent;
use crate::db::conversation as conv_repo;
use crate::db::message as msg_repo;
use crate::db::transcript as transcript_repo;
use crate::db::ai_provider as provider_repo;
use crate::ai_providers::{create_provider};
use crate::models::{ChatStreamEvent, Conversation, Message, Transcript, MessageRole};
use crate::screenshot::capture_screenshot as do_capture_screenshot;
use std::sync::atomic::{AtomicBool, Ordering};
use chrono::Utc;
use tauri::{AppHandle, Emitter};
use uuid::Uuid;

static STREAMING: AtomicBool = AtomicBool::new(false);

// Conversation management
#[tauri::command]
pub async fn create_conversation() -> Result<Conversation, String> {
    println!("[COMMAND] create_conversation called");

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
            println!("[COMMAND] Created conversation: {}", c.id);
            Ok(c)
        }
        None => Err("Failed to get conversation".to_string()),
    }
}

#[tauri::command]
pub async fn get_conversations() -> Result<Vec<Conversation>, String> {
    println!("[COMMAND] get_conversations called");
    let conversations = tokio::task::spawn_blocking(|| conv_repo::get_all())
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
    Ok(conversations)
}

#[tauri::command]
pub async fn get_conversation(id: String) -> Result<Conversation, String> {
    println!("[COMMAND] get_conversation called: id={}", id);
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
    println!(
        "[COMMAND] send_message called: provider={}, conversation_id={}, capture_screenshot={}",
        provider, conversation_id, capture_screenshot
    );
    println!("[COMMAND] user_message: {}", user_message);

    if STREAMING.load(Ordering::SeqCst) {
        println!("[COMMAND] Already streaming, ignoring");
        return Err("Already streaming".to_string());
    }

    let screenshot_path = if capture_screenshot {
        match do_capture_screenshot(app.clone()) {
            Ok(path) => {
                println!("[COMMAND] Screenshot captured: {}", path);
                Some(path)
            }
            Err(e) => {
                println!("[COMMAND] Failed to capture screenshot: {}", e);
                None
            }
        }
    } else {
        None
    };

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
        println!("[DB] Error saving user message: {}", e);
    }

    // Get provider config and create provider instance
    let provider_clone = provider.clone();
    let provider_config = tokio::task::spawn_blocking(move || {
        provider_repo::get_provider_config(&provider_clone)
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())?
    .ok_or_else(|| format!("Provider '{}' not configured", provider))?;

    let ai_provider = create_provider(&provider_config);

    STREAMING.store(true, Ordering::SeqCst);

    let mut assistant_response = String::new();

    // Stream from provider
    let stream_result = ai_provider.stream(user_message).await;

    match stream_result {
        Ok(mut stream) => {
            use futures_util::StreamExt;
            while let Some(event) = stream.next().await {
                if !STREAMING.load(Ordering::SeqCst) {
                    println!("[COMMAND] Stream stopped by user");
                    break;
                }

                match event {
                    AiStreamEvent::Chunk { content, is_finish } => {
                        assistant_response.push_str(&content);

                        let _ = app.emit("chat-stream", ChatStreamEvent::Chunk {
                            conversation_id: conversation_id.clone(),
                            content: content,
                            is_finish: is_finish,
                            timestamp: Utc::now().timestamp(),
                        });

                        if is_finish {
                            break;
                        }
                    }
                    AiStreamEvent::Error { code, message } => {
                        println!("[COMMAND] Stream error: {} - {}", code, message);
                    }
                }
            }
        }
        Err(e) => {
            println!("[COMMAND] Provider error: {}", e);
        }
    }

    STREAMING.store(false, Ordering::SeqCst);

    println!("[COMMAND] Stream completed");

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
        println!("[DB] Error saving assistant message: {}", e);
    }

    Ok(())
}

#[tauri::command]
pub async fn stop_stream() -> Result<(), String> {
    println!("[COMMAND] stop_stream called");
    STREAMING.store(false, Ordering::SeqCst);
    Ok(())
}

#[tauri::command]
pub async fn get_messages(conversation_id: String) -> Result<Vec<Message>, String> {
    println!("[COMMAND] get_messages called: conversation_id={}", conversation_id);
    let messages = tokio::task::spawn_blocking(move || msg_repo::get_by_conversation(&conversation_id))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
    Ok(messages)
}

#[tauri::command]
pub async fn get_transcripts(conversation_id: String) -> Result<Vec<Transcript>, String> {
    println!("[COMMAND] get_transcripts called: conversation_id={}", conversation_id);
    let transcripts = tokio::task::spawn_blocking(move || transcript_repo::get_by_conversation(&conversation_id))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
    Ok(transcripts)
}

#[tauri::command]
pub async fn delete_conversation(id: String) -> Result<(), String> {
    println!("[COMMAND] delete_conversation called: id={}", id);
    tokio::task::spawn_blocking(move || conv_repo::delete(&id))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
    Ok(())
}
