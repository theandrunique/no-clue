use crate::application::chats::generation::start_generation;
use crate::db::conversation as conversation_repo;
use crate::db::message as msg_repo;
use crate::domain::messages::MessageRole;
use crate::infra::screenshot::capture_screenshot as do_capture_screenshot;
use crate::infra::screenshot::ScreenshotResult;
use chrono::Utc;
use sqlx::SqlitePool;
use tauri::AppHandle;
use tauri::Manager;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::{application::chats::SESSION, domain::messages::Message, errors::AppError};

#[tauri::command]
pub async fn send_message(
    app: AppHandle,
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

    let mut guard = SESSION.lock().await;
    if guard.is_some() {
        tracing::warn!("Already streaming, ignoring request");
        return Err(AppError::LlmProviderAlreadyRunning);
    }

    let pool = app.state::<SqlitePool>();

    if conversation_repo::get_by_id(&pool, &conversation_id)
        .await?
        .is_none()
    {
        return Err(AppError::ConversationNotFound);
    }

    let screenshot_result: Option<ScreenshotResult> = if capture_screenshot {
        match do_capture_screenshot(app.clone()) {
            Ok(result) => Some(result),
            Err(e) => {
                tracing::error!(error = %e, "Failed to capture screenshot");
                None
            }
        }
    } else {
        None
    };

    let screenshot_path = screenshot_result.as_ref().map(|r| r.relative_path.clone());
    let screenshot_base64 = screenshot_result.map(|r| r.base64);

    let message = Message {
        id: Uuid::new_v4(),
        conversation_id: conversation_id.clone(),
        role: MessageRole::User,
        content: user_message.clone(),
        screenshot_path,
        finish_reason: None,
        created_at: Utc::now(),
    };

    msg_repo::save(&pool, &message).await?;

    let cancellation_token = CancellationToken::new();

    tokio::spawn({
        let app = app.clone();
        let token = cancellation_token.clone();
        async move {
            start_generation(
                app,
                conversation_id,
                provider,
                system_prompt_id,
                capture_screenshot,
                screenshot_base64,
                token,
            )
            .await;
        }
    });

    *guard = Some(cancellation_token);

    Ok(message)
}
