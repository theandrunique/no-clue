use tauri::State;
use uuid::Uuid;

use crate::{
    application::chats::ChatRouter,
    domain::chats::{LlmSettings, Message},
    errors::AppError,
};

#[tauri::command]
pub async fn send_message(
    chat_router: State<'_, ChatRouter>,
    model: LlmSettings,
    conversation_id: Uuid,
    user_message: String,
    capture_screenshot: bool,
    system_prompt_id: Option<Uuid>,
) -> Result<Message, AppError> {
    tracing::trace!(
        model_id = %model.model_id(),
        %conversation_id,
        %user_message,
        capture_screenshot,
        ?system_prompt_id,
        "send_message called"
    );
    let chat_handle = chat_router.get_or_create(conversation_id).await;
    chat_handle
        .send_message(model, capture_screenshot, system_prompt_id, user_message)
        .await
}

#[tauri::command]
pub async fn retry_generation(
    chat_router: State<'_, ChatRouter>,
    model: LlmSettings,
    conversation_id: Uuid,
    user_message_id: Uuid,
    capture_screenshot: bool,
    system_prompt_id: Option<Uuid>,
) -> Result<(), AppError> {
    tracing::trace!(
        model_id = %model.model_id(),
        %conversation_id,
        %user_message_id,
        capture_screenshot,
        ?system_prompt_id,
        "retry_generation called"
    );
    let chat_handle = chat_router.get_or_create(conversation_id).await;
    chat_handle
        .regenerate(
            model,
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
