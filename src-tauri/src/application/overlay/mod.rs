use std::sync::LazyLock;

use anyhow::Context;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::errors::AppError;

#[derive(PartialEq)]
pub enum OverlayStatus {
    Idle,
    Active { conversation_id: Uuid },
}

static STATE: LazyLock<Mutex<OverlayStatus>> = LazyLock::new(|| Mutex::new(OverlayStatus::Idle));

#[tauri::command]
pub async fn start_overlay_session(app: AppHandle, conversation_id: Uuid) -> Result<(), AppError> {
    tracing::trace!("start_overlay_session called");

    let mut state = STATE.lock().await;
    if *state != OverlayStatus::Idle {
        return Err(AppError::OverlayAlreadyRunning);
    }

    WebviewWindowBuilder::new(
        &app,
        "overlay",
        WebviewUrl::App(format!("/overlay/{}", &conversation_id).into()),
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

    *state = OverlayStatus::Active { conversation_id };
    Ok(())
}

#[tauri::command]
pub async fn stop_overlay_session(app: AppHandle) -> Result<(), AppError> {
    tracing::trace!("stop_overlay_session called");

    let mut state = STATE.lock().await;
    if matches!(*state, OverlayStatus::Idle) {
        return Err(AppError::OverlayNotRunning);
    }

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

    *state = OverlayStatus::Idle;
    Ok(())
}
