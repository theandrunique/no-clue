use tauri::State;
use uuid::Uuid;

use crate::{application::chats::ChatRouter, domain::chat::Message, errors::AppError};

#[tauri::command]
pub async fn send_message(
    chat_router: State<'_, ChatRouter>,
    provider: String,
    conversation_id: Uuid,
    user_message: String,
    capture_screenshot: bool,
    system_prompt_id: Option<Uuid>,
) -> Result<Message, AppError> {
    tracing::trace!(
        provider,
        %conversation_id,
        %user_message,
        capture_screenshot,
        ?system_prompt_id,
        "send_message called"
    );
    let chat_handle = chat_router.get_or_create(conversation_id).await;
    chat_handle
        .send_message(provider, capture_screenshot, system_prompt_id, user_message)
        .await
}

#[tauri::command]
pub async fn retry_generation(
    chat_router: State<'_, ChatRouter>,
    provider: String,
    conversation_id: Uuid,
    user_message_id: Uuid,
    capture_screenshot: bool,
    system_prompt_id: Option<Uuid>,
) -> Result<(), AppError> {
    tracing::trace!(
        provider,
        %conversation_id,
        %user_message_id,
        capture_screenshot,
        ?system_prompt_id,
        "retry_generation called"
    );
    let chat_handle = chat_router.get_or_create(conversation_id).await;
    chat_handle
        .regenerate(
            provider,
            capture_screenshot,
            system_prompt_id,
            user_message_id,
        )
        .await
}

#[tauri::command]
pub async fn stop_generation(
    chat_router: State<'_, ChatRouter>,
    conversation_id: Uuid,
) -> Result<(), AppError> {
    tracing::trace!(%conversation_id, "stop_generation called");
    let chat_handle = chat_router.get_or_create(conversation_id).await;
    chat_handle.stop_generation().await
}
