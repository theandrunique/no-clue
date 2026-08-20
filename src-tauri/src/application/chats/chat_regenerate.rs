use sqlx::SqlitePool;
use tauri::{AppHandle, Manager};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::{
    application::chats::{generation::start_generation, SESSION},
    db::{conversation as conversation_repo, message as msg_repo},
    errors::AppError,
    infra::screenshot::{capture_screenshot as do_capture_screenshot, read_screenshot_base64},
};

#[tauri::command]
pub async fn retry_generation(
    app: AppHandle,
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

    let user_message_row = msg_repo::get_by_id(&pool, &user_message_id)
        .await?
        .filter(|m| m.conversation_id == conversation_id)
        .ok_or(AppError::MessageNotFound)?;

    msg_repo::delete_after(&pool, &conversation_id, &user_message_id).await?;

    let screenshot_base64 = if capture_screenshot {
        match &user_message_row.screenshot_path {
            Some(path) => match read_screenshot_base64(&app, path) {
                Ok(Some(b64)) => Some(b64),
                Ok(None) => {
                    tracing::warn!(path, "Stored screenshot not found, capturing a new one");
                    capture_screenshot_opt(&app)
                }
                Err(e) => {
                    tracing::error!(error = %e, "Failed to read stored screenshot");
                    capture_screenshot_opt(&app)
                }
            },
            None => capture_screenshot_opt(&app),
        }
    } else {
        None
    };

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

    Ok(())
}

fn capture_screenshot_opt(app: &AppHandle) -> Option<String> {
    match do_capture_screenshot(app.clone()) {
        Ok(result) => Some(result.base64),
        Err(e) => {
            tracing::error!(error = %e, "Failed to capture screenshot");
            None
        }
    }
}
