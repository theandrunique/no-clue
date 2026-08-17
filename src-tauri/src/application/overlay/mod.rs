use anyhow::Context;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::{application::conversations::create_conversation, errors::AppError};

#[tauri::command]
pub async fn start_overlay_session(app: AppHandle) -> Result<(), AppError> {
    tracing::trace!("start_overlay_session called");
    let conversation = create_conversation(app.clone()).await?;

    WebviewWindowBuilder::new(
        &app,
        "overlay",
        WebviewUrl::App(format!("/overlay?conversationId={}", &conversation.id).into()),
    )
    .title("No-Clue Overlay")
    .inner_size(500.0, 54.0)
    .center()
    .decorations(false)
    .transparent(true)
    .always_on_top(true)
    .skip_taskbar(true)
    .resizable(false)
    .visible_on_all_workspaces(true)
    .content_protected(true)
    .focusable(false)
    .shadow(false)
    .build()
    .context("Failed to create overlay window")?;

    if let Some(window) = app.get_webview_window("dashboard") {
        if let Err(e) = window.hide() {
            tracing::error!(error = ?e, "Failed to hide dashboard");
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn close_overlay_session(app: AppHandle) {
    if let Some(window) = app.get_webview_window("overlay") {
        if let Err(e) = window.close() {
            tracing::error!(error = ?e, "Failed to close overlay");
        }
    }
        
    if let Some(window) = app.get_webview_window("dashboard") {
        if let Err(e) = window.show() {
            tracing::error!(error = ?e, "Failed to show dashboard");
        }
        if let Err(e) = window.set_focus() {
            tracing::error!(error = ?e, "Failed to focus dashboard");
        }
    }
}
