use sqlx::SqlitePool;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::{domain::transcript::Transcript, errors::AppError, infra::db};

#[tauri::command]
pub async fn get_transcripts(
    app: AppHandle,
    conversation_id: Uuid,
) -> Result<Vec<Transcript>, AppError> {
    tracing::trace!(%conversation_id, "get_transcripts called");
    let pool = app.state::<SqlitePool>();
    let transcripts = db::transcript::get_by_conversation(&pool, &conversation_id).await?;
    Ok(transcripts)
}
