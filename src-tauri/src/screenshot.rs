use tauri::Manager;
use xcap::Monitor;

pub fn capture_screenshot(app: tauri::AppHandle) -> Result<String, String> {
    println!("[COMMAND] capture_screenshot called");

    let monitors = Monitor::all().map_err(|e| e.to_string())?;

    if monitors.is_empty() {
        return Err("No monitors found".to_string());
    }

    let monitor = &monitors[0];
    let image = monitor.capture_image().map_err(|e| e.to_string())?;

    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let screenshots_dir = app_data_dir.join("screenshots");
    std::fs::create_dir_all(&screenshots_dir).map_err(|e| e.to_string())?;

    let filename = format!("screenshot_{}.png", chrono::Utc::now().timestamp_millis());
    let path = screenshots_dir.join(&filename);

    image.save(&path).map_err(|e| e.to_string())?;

    println!("[COMMAND] Screenshot saved: {}", filename);

    Ok(format!("screenshots/{}", filename))
}
