use sqlx::SqlitePool;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::{domain::chat::Message, errors::AppError, infra::db};

#[tauri::command]
pub async fn get_messages(app: AppHandle, conversation_id: Uuid) -> Result<Vec<Message>, AppError> {
    tracing::trace!(%conversation_id, "get_messages called");
    let pool = app.state::<SqlitePool>();
    let messages = db::message::get_by_conversation(&pool, &conversation_id).await?;
    Ok(messages)
}
