use crate::db::conversation as conv_repo;
use crate::db::message as msg_repo;
use crate::db::transcript as transcript_repo;
use crate::error::log_err;
use crate::models::{Conversation, Message, Transcript};

// Conversation management
#[tauri::command]
pub async fn create_conversation() -> Result<Conversation, String> {
    tracing::trace!("create_conversation called");

    let timestamp = chrono::Utc::now().timestamp();
    let new_conversation = Conversation {
        id: uuid::Uuid::new_v4().to_string(),
        title: "New conversation".to_string(),
        created_at: timestamp,
        updated_at: timestamp,
    };

    let conversation_to_save = new_conversation.clone();

    tokio::task::spawn_blocking(move || conv_repo::create(&conversation_to_save))
        .await
        .map_err(|e| log_err(e, "create_conversation"))?
        .map_err(|e| log_err(e, "create_conversation"))?;

    Ok(new_conversation)
}

#[tauri::command]
pub async fn get_conversations() -> Result<Vec<Conversation>, String> {
    tracing::trace!("get_conversations called");
    let conversations = tokio::task::spawn_blocking(|| conv_repo::get_all())
        .await
        .map_err(|e| log_err(e, "get_conversations"))?
        .map_err(|e| log_err(e, "get_conversations"))?;
    Ok(conversations)
}

#[tauri::command]
pub async fn get_conversation(id: String) -> Result<Conversation, String> {
    tracing::trace!(conversation_id = %id, "get_conversation called");
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
    tracing::trace!(conversation_id = %id, "delete_conversation called");
    tokio::task::spawn_blocking(move || conv_repo::delete(&id))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_messages(conversation_id: String) -> Result<Vec<Message>, String> {
    tracing::trace!(conversation_id = %conversation_id, "get_messages called");
    let messages =
        tokio::task::spawn_blocking(move || msg_repo::get_by_conversation(&conversation_id))
            .await
            .map_err(|e| e.to_string())?
            .map_err(|e| e.to_string())?;
    Ok(messages)
}

#[tauri::command]
pub async fn get_transcripts(conversation_id: String) -> Result<Vec<Transcript>, String> {
    tracing::trace!(conversation_id = %conversation_id, "get_transcripts called");
    let transcripts =
        tokio::task::spawn_blocking(move || transcript_repo::get_by_conversation(&conversation_id))
            .await
            .map_err(|e| e.to_string())?
            .map_err(|e| e.to_string())?;
    Ok(transcripts)
}
