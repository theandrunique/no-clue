use std::fmt;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, LogicalPosition, Manager, Position};

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

pub fn move_overlay(app: AppHandle, direction: Direction, step: f64) -> Result<(), String> {
    tracing::debug!(%direction, step, "move_overlay called");

    let window = app.get_webview_window("overlay").ok_or_else(|| {
        tracing::error!("Overlay window not found");
        "Overlay window not found".to_string()
    })?;

    let (delta_x, delta_y) = match direction {
        Direction::Up => (0.0, -step),
        Direction::Down => (0.0, step),
        Direction::Left => (-step, 0.0),
        Direction::Right => (step, 0.0),
    };

    let position = window.outer_position().map_err(|e| {
        tracing::error!(?e, "Failed to get window outer position");
        "Failed to get window outer position".to_string()
    })?;

    let scale_factor = window.scale_factor().map_err(|e| {
        tracing::error!(?e, "Failed to get window scale factor");
        "Failed to get window scale factor".to_string()
    })?;

    let logical_pos = position.to_logical::<f64>(scale_factor);

    let new_pos = LogicalPosition {
        x: logical_pos.x + delta_x,
        y: logical_pos.y + delta_y,
    };

    window
        .set_position(Position::Logical(new_pos))
        .map_err(|e| {
            tracing::error!(?e, "Failed to set window position");
            "Failed to set window position".to_string()
        })?;

    Ok(())
}

pub fn toggle_overlay(app: &AppHandle) -> Result<(), String> {
    tracing::debug!("toggle_overlay called");

    let window = app.get_webview_window("overlay").ok_or_else(|| {
        tracing::error!("Overlay window not found");
        "Overlay window not found".to_string()
    })?;

    let is_visible = window.is_visible().map_err(|e| {
        tracing::error!(?e, "Error getting is_visible");
        "Error getting is_visible".to_string()
    })?;

    if is_visible {
        if let Err(e) = window.hide() {
            tracing::error!(?e, "Error hiding overlay window");
            return Err("Error hiding overlay window".to_string());
        }
    } else {
        if let Err(e) = window.show() {
            tracing::error!(?e, "Error showing overlay window");
            return Err("Error showing overlay window".to_string());
        }
    }

    Ok(())
}
