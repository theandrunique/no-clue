use futures_util::StreamExt;
use tauri::{AppHandle, Emitter};

use crate::{
    domain::events,
    errors::AppError,
    infra::audio_capture::{list_input_devices, list_output_devices, AudioDevice, AudioInput},
};

#[tauri::command]
pub fn get_input_devices() -> Result<Vec<AudioDevice>, AppError> {
    tracing::trace!("get_input_devices called");
    Ok(list_input_devices()?)
}

#[tauri::command]
pub fn get_output_devices() -> Result<Vec<AudioDevice>, AppError> {
    tracing::trace!("get_output_devices called");
    Ok(list_output_devices()?)
}

#[tauri::command]
pub async fn test_system_audio(
    app: AppHandle,
    device_id: Option<String>,
    duration_secs: u64,
) -> Result<(), String> {
    tracing::trace!(
        "test_system_audio called, device_id={:?}, duration={}",
        device_id,
        duration_secs
    );

    let app_clone = app.clone();

    tokio::spawn(async move {
        tracing::trace!("spawned task, creating AudioInput for system...");

        let input = match AudioInput::system(device_id) {
            Ok(i) => {
                tracing::trace!("AudioInput created successfully");
                i
            }
            Err(e) => {
                tracing::error!("Failed to create AudioInput: {}", e);
                let _ = app_clone.emit(events::TEST_STREAM_ERROR, format!("Failed: {}", e));
                return;
            }
        };

        tracing::trace!("calling stream()...");
        let stream = input.stream();
        let sample_rate = stream.sample_rate();
        tracing::trace!("stream created, sample_rate={}", sample_rate);

        let max_samples = (sample_rate as u64 * duration_secs) as usize;

        let _ = app_clone.emit(events::TEST_STREAM_STARTED, sample_rate);

        let mut collected = 0;
        let mut count = 0usize;

        tracing::trace!("starting stream loop...");
        let mut stream = stream;
        while let Some(sample) = stream.next().await {
            count += 1;
            if sample.abs() > 0.01 {
                collected += 1;
            }

            if count >= max_samples {
                break;
            }

            if count % (sample_rate as usize) == 0 {
                let _ = app_clone.emit(
                    events::TEST_STREAM_PROGRESS,
                    (count / (sample_rate as usize), collected),
                );
            }
        }

        tracing::trace!(
            "stream loop ended, count={}, collected={}",
            count,
            collected
        );
        let _ = app_clone.emit(events::TEST_STREAM_STOPPED, (count, collected));
    });

    tracing::trace!("returning from test_system_audio command");
    Ok(())
}

#[tauri::command]
pub async fn test_microphone_audio(
    app: AppHandle,
    device_id: Option<String>,
    duration_secs: u64,
) -> Result<(), String> {
    tracing::trace!(
        "test_microphone_audio called, device_id={:?}, duration={}",
        device_id,
        duration_secs
    );

    let app_clone = app.clone();

    tokio::spawn(async move {
        tracing::trace!("spawned task, creating AudioInput for microphone...");

        let input = match AudioInput::microphone(device_id) {
            Ok(i) => {
                tracing::trace!("AudioInput (microphone) created successfully");
                i
            }
            Err(e) => {
                tracing::error!("Failed to create microphone AudioInput: {}", e);
                let _ = app_clone.emit(events::TEST_MIC_ERROR, format!("Failed: {}", e));
                return;
            }
        };

        tracing::trace!("calling stream()...");
        let stream = input.stream();
        let sample_rate = stream.sample_rate();
        tracing::trace!("microphone stream created, sample_rate={}", sample_rate);

        let max_samples = (sample_rate as u64 * duration_secs) as usize;

        let _ = app_clone.emit(events::TEST_MIC_STARTED, sample_rate);

        let mut collected = 0;
        let mut count = 0usize;

        tracing::trace!("starting microphone stream loop...");
        let mut stream = stream;
        while let Some(sample) = stream.next().await {
            count += 1;
            if sample.abs() > 0.01 {
                collected += 1;
            }

            if count >= max_samples {
                break;
            }

            if count % (sample_rate as usize) == 0 {
                let _ = app_clone.emit(
                    events::TEST_MIC_PROGRESS,
                    (count / (sample_rate as usize), collected),
                );
            }
        }

        tracing::trace!(
            "microphone stream loop ended, count={}, collected={}",
            count,
            collected
        );
        let _ = app_clone.emit(events::TEST_MIC_STOPPED, (count, collected));
    });

    tracing::trace!("returning from test_microphone_audio command");
    Ok(())
}
