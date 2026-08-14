use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

static OVERLAY_VISIBLE: AtomicBool = AtomicBool::new(true);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

pub fn move_overlay(app: AppHandle, direction: Direction, step: i32) -> Result<(), String> {
    tracing::debug!(direction = %direction, step, "move_overlay called");

    let window = app
        .get_webview_window("overlay")
        .ok_or("Overlay window not found")?;

    let (delta_x, delta_y) = match direction {
        Direction::Up => (0, -step),
        Direction::Down => (0, step),
        Direction::Left => (-step, 0),
        Direction::Right => (step, 0),
    };

    if let Ok(position) = window.outer_position() {
        let new_x = position.x + delta_x;
        let new_y = position.y + delta_y;
        window
            .set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                x: new_x,
                y: new_y,
            }))
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn toggle_overlay(app: &AppHandle) -> Result<bool, String> {
    tracing::debug!("toggle_overlay called");

    let is_visible = OVERLAY_VISIBLE.load(Ordering::SeqCst);

    let window = app
        .get_webview_window("overlay")
        .ok_or("Overlay window not found")?;

    if is_visible {
        // скрываем - перемещаем за пределы экрана мгновенно
        window
            .set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                x: -10000,
                y: -10000,
            }))
            .map_err(|e| e.to_string())?;
        OVERLAY_VISIBLE.store(false, Ordering::SeqCst);
    } else {
        // показываем - возвращаем на центр
        window.center().map_err(|e| e.to_string())?;
        OVERLAY_VISIBLE.store(true, Ordering::SeqCst);
    }

    Ok(!is_visible)
}

#[tauri::command]
pub async fn set_overlay_visible(window: WebviewWindow, visible: bool) -> Result<(), String> {
    tracing::info!(visible, "set_overlay_visible called");
    if visible {
        window.show().map_err(|e| e.to_string())?;
    } else {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn open_dashboard(app: AppHandle) -> Result<(), String> {
    tracing::trace!("open_dashboard called");
    if let Some(window) = app.get_webview_window("dashboard") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    WebviewWindowBuilder::new(
        &app,
        "dashboard",
        WebviewUrl::App("/dashboard/conversations".into()),
    )
    .title("No-Clue Dashboard")
    .inner_size(900.0, 700.0)
    .center()
    .decorations(true)
    .resizable(true)
    .content_protected(true)
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}
