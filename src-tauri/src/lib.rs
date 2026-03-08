use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn open_dashboard(app: AppHandle) -> Result<(), String> {
    // Check if dashboard already exists
    if let Some(window) = app.get_webview_window("dashboard") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    // Create new dashboard window
    WebviewWindowBuilder::new(&app, "dashboard", WebviewUrl::App("/dashboard".into()))
        .title("No-Clue Settings")
        .inner_size(900.0, 700.0)
        .center()
        .decorations(true)
        .resizable(true)
        .content_protected(true)
        .build()
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![greet, open_dashboard])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
