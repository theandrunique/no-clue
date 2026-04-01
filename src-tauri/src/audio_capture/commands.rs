use crate::{
    audio_capture::{AudioDevice, SpeakerInput},
    error::log_err,
};
use futures_util::StreamExt;
use tauri::{AppHandle, Emitter};

#[tauri::command]
pub fn get_input_devices() -> Result<Vec<AudioDevice>, String> {
    tracing::trace!("get_input_devices called");
    crate::audio_capture::list_input_devices()
        .map_err(|e| log_err(e, "Failed to get input devices"))
}

#[tauri::command]
pub fn get_output_devices() -> Result<Vec<AudioDevice>, String> {
    tracing::trace!("get_output_devices called");
    crate::audio_capture::list_output_devices()
        .map_err(|e| log_err(e, "Failed to get output devices"))
}

#[tauri::command]
pub async fn test_stream_audio(
    app: AppHandle,
    device_id: Option<String>,
    duration_secs: u64,
) -> Result<(), String> {
    tracing::trace!(
        "test_stream_audio called, device_id={:?}, duration={}",
        device_id,
        duration_secs
    );

    let app_clone = app.clone();

    tokio::spawn(async move {
        tracing::trace!("spawned task, creating SpeakerInput...");

        let input = match SpeakerInput::new_with_device(device_id) {
            Ok(i) => {
                tracing::trace!("SpeakerInput created successfully");
                i
            }
            Err(e) => {
                tracing::error!("Failed to create speaker input: {}", e);
                return;
            }
        };

        tracing::trace!("calling stream()...");
        let stream = input.stream();
        let sample_rate = stream.sample_rate();
        tracing::trace!("stream created, sample_rate={}", sample_rate);

        let max_samples = (sample_rate as u64 * duration_secs) as usize;

        let _ = app_clone.emit("test-stream-started", sample_rate);

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
                    "test-stream-progress",
                    (count / (sample_rate as usize), collected),
                );
            }
        }

        tracing::trace!(
            "stream loop ended, count={}, collected={}",
            count,
            collected
        );
        let _ = app_clone.emit("test-stream-stopped", (count, collected));
    });

    tracing::trace!("returning from command");
    Ok(())
}
