use crate::{application::chats::SESSION, errors::AppError};

#[tauri::command]
pub async fn stop_stream() -> Result<(), AppError> {
    tracing::trace!("stop_stream called");

    if let Some(session) = SESSION.lock().await.as_ref() {
        session.cancel();
    } else {
        tracing::warn!("LLM provider was not running but stop was requested");
    }

    Ok(())
}
