use crate::db::conversation as conv_repo;
use crate::db::message as msg_repo;
use crate::db::transcript as transcript_repo;
use crate::models::{Conversation, Message, Transcript};

fn log_err<E: std::fmt::Display>(e: E, context: &str) -> String {
    tracing::error!(error = %e, context);
    e.to_string()
}

// Conversation management
#[tauri::command]
pub async fn create_conversation() -> Result<Conversation, String> {
    tracing::info!("create_conversation called");

    let title = "New conversation".to_string();
    let id = tokio::task::spawn_blocking(move || conv_repo::create(title))
        .await
        .map_err(|e| log_err(e, "create_conversation"))?
        .map_err(|e| log_err(e, "create_conversation"))?;

    let conversation = tokio::task::spawn_blocking(move || conv_repo::get_by_id(&id))
        .await
        .map_err(|e| log_err(e, "get_conversation"))?
        .map_err(|e| log_err(e, "get_conversation"))?;

    match conversation {
        Some(c) => {
            tracing::info!(conversation_id = %c.id, "Conversation created");
            Ok(c)
        }
        None => Err(log_err("Failed to get conversation", "create_conversation")),
    }
}

#[tauri::command]
pub async fn get_conversations() -> Result<Vec<Conversation>, String> {
    tracing::debug!("get_conversations called");
    let conversations = tokio::task::spawn_blocking(|| conv_repo::get_all())
        .await
        .map_err(|e| log_err(e, "get_conversations"))?
        .map_err(|e| log_err(e, "get_conversations"))?;
    Ok(conversations)
}

#[tauri::command]
pub async fn get_conversation(id: String) -> Result<Conversation, String> {
    tracing::debug!(conversation_id = %id, "get_conversation called");
    let id_clone = id.clone();
    let result = tokio::task::spawn_blocking(move || conv_repo::get_by_id(&id_clone))
        .await
        .map_err(|e| log_err(e, "get_conversation"))?
        .map_err(|e| log_err(e, "get_conversation"))?
        .ok_or_else(|| log_err("Conversation not found", "get_conversation"))?;
    Ok(result)
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
