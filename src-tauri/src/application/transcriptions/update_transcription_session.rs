use uuid::Uuid;

use crate::application::transcriptions::CURRENT_CONVERSATION_ID;

#[tauri::command]
pub async fn update_transcription_session(conversation_id: Uuid) -> Result<(), String> {
    tracing::trace!(conversation_id = ?conversation_id, "update_transcription_session called");
    let mut current = CURRENT_CONVERSATION_ID.lock().await;
    *current = Some(conversation_id);
    Ok(())
}
