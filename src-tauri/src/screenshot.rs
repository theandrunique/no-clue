use base64::Engine;
use tauri::Manager;
use xcap::Monitor;

pub struct ScreenshotResult {
    pub relative_path: String,
    pub base64: String,
}

pub fn capture_screenshot(app: tauri::AppHandle) -> Result<ScreenshotResult, String> {
    tracing::info!("capture_screenshot called");

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

    tracing::debug!("Screenshot saved: {}", path.display());

    let mut buffer = Vec::new();
    use image::ImageEncoder;
    image::codecs::png::PngEncoder::new(&mut buffer)
        .write_image(
            image.as_raw(),
            image.width(),
            image.height(),
            image::ExtendedColorType::Rgba8,
        )
        .map_err(|e| e.to_string())?;

    let base64 = base64::engine::general_purpose::STANDARD.encode(&buffer);

    Ok(ScreenshotResult {
        relative_path: format!("screenshots/{}", filename),
        base64,
    })
}
