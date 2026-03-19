use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

// Window management
#[tauri::command]
pub async fn move_overlay(
    window: WebviewWindow,
    direction: String,
    step: i32,
) -> Result<(), String> {
    println!(
        "[COMMAND] move_overlay called: direction={}, step={}",
        direction, step
    );

    let (delta_x, delta_y) = match direction.as_str() {
        "up" => (0, -step),
        "down" => (0, step),
        "left" => (-step, 0),
        "right" => (step, 0),
        _ => return Err("Invalid direction".to_string()),
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

#[tauri::command]
pub async fn set_overlay_visible(window: WebviewWindow, visible: bool) -> Result<(), String> {
    println!("[COMMAND] set_overlay_visible called: visible={}", visible);
    if visible {
        window.show().map_err(|e| e.to_string())?;
    } else {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

// Dashboard
#[tauri::command]
pub async fn open_dashboard(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("dashboard") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    WebviewWindowBuilder::new(&app, "dashboard", WebviewUrl::App("/dashboard/conversations".into()))
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
