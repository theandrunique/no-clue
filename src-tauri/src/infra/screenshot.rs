use anyhow::Context;
use base64::Engine;
use chrono::Utc;
use tauri::Manager;
use xcap::Monitor;
use image::{ImageEncoder, codecs::png::PngEncoder};

pub struct ScreenshotResult {
    pub relative_path: String,
    pub base64: String,
}

pub fn capture_screenshot(app: tauri::AppHandle) -> Result<ScreenshotResult, anyhow::Error> {
    let monitors = Monitor::all().context("Failed to get monitors")?;

    if monitors.is_empty() {
        return Err(anyhow::anyhow!("No monitors found"));
    }

    let monitor = &monitors[0];
    let image = monitor.capture_image().context("Failed to captrure screenshot")?;

    let app_data_dir = app.path().app_data_dir().context("Failed to get app_data_dir")?;
    let screenshots_dir = app_data_dir.join("screenshots");
    std::fs::create_dir_all(&screenshots_dir).context("Failed to init screenshoots directory")?;

    let filename = format!("screenshot_{}.png", Utc::now().timestamp_millis());
    let path = screenshots_dir.join(&filename);

    image.save(&path).context("Failed to save screenshot")?;

    tracing::trace!("Screenshot saved: {}", path.display());

    let mut buffer = Vec::new();
    PngEncoder::new(&mut buffer)
        .write_image(
            image.as_raw(),
            image.width(),
            image.height(),
            image::ExtendedColorType::Rgba8,
        )
        .context("Failed to encode screenshot")?;

    let base64 = base64::engine::general_purpose::STANDARD.encode(&buffer);

    Ok(ScreenshotResult {
        relative_path: format!("screenshots/{}", filename),
        base64,
    })
}

pub fn read_screenshot_base64(
    app: &tauri::AppHandle,
    relative_path: &str,
) -> Result<Option<String>, anyhow::Error> {
    let app_data_dir = app.path().app_data_dir().context("Failed to get app_data_dir")?;
    let path = app_data_dir.join(relative_path);

    if !path.exists() {
        return Ok(None);
    }

    let bytes = std::fs::read(&path).context("Failed to read screenshot")?;
    Ok(Some(
        base64::engine::general_purpose::STANDARD.encode(&bytes),
    ))
}
