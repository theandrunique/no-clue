use chrono::Utc;
use sqlx::SqlitePool;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::{db::conversation as conv_repo, domain::conversation::Conversation, errors::AppError};

#[tauri::command]
pub async fn create_conversation(app: AppHandle) -> Result<Conversation, AppError> {
    tracing::trace!("create_conversation called");
    let pool = app.state::<SqlitePool>();

    let now = Utc::now();
    let new_conversation = Conversation {
        id: Uuid::new_v4(),
        title: "New conversation".to_string(),
        created_at: now,
        updated_at: now,
    };
    conv_repo::save(&pool, &new_conversation).await?;

    Ok(new_conversation)
}

#[tauri::command]
pub async fn get_conversations(app: AppHandle) -> Result<Vec<Conversation>, AppError> {
    tracing::trace!("get_conversations called");
    let pool = app.state::<SqlitePool>();
    Ok(conv_repo::get_all(&pool).await?)
}

#[tauri::command]
pub async fn get_conversation(app: AppHandle, id: Uuid) -> Result<Option<Conversation>, AppError> {
    tracing::trace!(conversation_id = %id, "get_conversation called");
    let pool = app.state::<SqlitePool>();
    Ok(conv_repo::get_by_id(&pool, &id).await?)
}

#[tauri::command]
pub async fn delete_conversation(app: AppHandle, id: Uuid) -> Result<(), AppError> {
    tracing::trace!(conversation_id = %id, "delete_conversation called");
    let pool = app.state::<SqlitePool>();
    let result = conv_repo::delete(&pool, &id).await?;
    if !result {
        return Err(AppError::ConversationNotFound);
    }
    Ok(())
}
