use sqlx::SqlitePool;
use tauri::AppHandle;
use tauri::Manager;

use crate::db::conversation as conv_repo;
use crate::db::message as msg_repo;
use crate::db::transcript as transcript_repo;
use crate::domain::conversations::Conversation;
use crate::domain::messages::Message;
use crate::domain::transcriptions::Transcript;
use crate::errors::AppError;

#[tauri::command]
pub async fn create_conversation(app: AppHandle) -> Result<Conversation, AppError> {
    tracing::trace!("create_conversation called");
    let pool = app.state::<SqlitePool>();

    let timestamp = chrono::Utc::now().timestamp();
    let new_conversation = Conversation {
        id: uuid::Uuid::new_v4().to_string(),
        title: "New conversation".to_string(),
        created_at: timestamp,
        updated_at: timestamp,
    };
    conv_repo::create(&pool, &new_conversation).await?;

    Ok(new_conversation)
}

#[tauri::command]
pub async fn get_conversations(app: AppHandle) -> Result<Vec<Conversation>, AppError> {
    tracing::trace!("get_conversations called");
    let pool = app.state::<SqlitePool>();
    Ok(conv_repo::get_all(&pool).await?)
}

#[tauri::command]
pub async fn get_conversation(app: AppHandle, id: &str) -> Result<Option<Conversation>, AppError> {
    tracing::trace!(conversation_id = %id, "get_conversation called");
    let pool = app.state::<SqlitePool>();
    Ok(conv_repo::get_by_id(&pool, id).await?)
}

#[tauri::command]
pub async fn delete_conversation(app: AppHandle, id: &str) -> Result<(), AppError> {
    tracing::trace!(conversation_id = %id, "delete_conversation called");
    let pool = app.state::<SqlitePool>();
    let result = conv_repo::delete(&pool, id).await?;
    if !result {
        return Err(AppError::ConversationNotFound);
    }
    Ok(())
}

#[tauri::command]
pub async fn get_messages(app: AppHandle, conversation_id: &str) -> Result<Vec<Message>, AppError> {
    tracing::trace!(conversation_id, "get_messages called");
    let pool = app.state::<SqlitePool>();
    let messages = msg_repo::get_by_conversation(&pool, conversation_id).await?;
    Ok(messages)
}

#[tauri::command]
pub async fn get_transcripts(
    app: AppHandle,
    conversation_id: &str,
) -> Result<Vec<Transcript>, AppError> {
    tracing::trace!(conversation_id = %conversation_id, "get_transcripts called");
    let pool = app.state::<SqlitePool>();
    let transcripts = transcript_repo::get_by_conversation(&pool, conversation_id).await?;
    Ok(transcripts)
}
