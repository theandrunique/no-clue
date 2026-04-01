use crate::audio_capture::{AudioInput, AudioStream};
use crate::db::transcript as transcript_repo;
use crate::models::Speaker;
use crate::stt_providers::{
    create_stt_provider, get_stt_descriptors, AudioCaptureConfig, SttProviderConfig, SttProviderDescriptor, SttResultCallback, SttTranscriptResult,
};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Serialize, Deserialize)]
pub struct SttSettingsCommand {
    pub stt_type: SttProviderConfig,
}

impl From<SttSettingsCommand> for SttProviderConfig {
    fn from(cmd: SttSettingsCommand) -> Self {
        cmd.stt_type
    }
}

#[tauri::command]
pub async fn save_stt_settings(settings: SttSettingsCommand) -> Result<(), String> {
    tracing::info!(stt_type = ?settings.stt_type, "save_stt_settings called");
    // TODO: Save to database
    Ok(())
}

#[tauri::command]
pub async fn get_stt_settings() -> Result<SttSettingsCommand, String> {
    tracing::info!("get_stt_settings called");
    Ok(SttSettingsCommand {
        stt_type: SttProviderConfig::Fake,
    })
}

#[tauri::command]
pub fn get_stt_providers() -> Vec<SttProviderDescriptor> {
    get_stt_descriptors()
}

static TRANSCRIPTION_RUNNING: AtomicBool = AtomicBool::new(false);
static CURRENT_CONVERSATION_ID: Mutex<Option<String>> = Mutex::new(None);

#[tauri::command]
pub async fn update_transcription_session(conversation_id: String) -> Result<(), String> {
    tracing::debug!(
        conversation_id = %conversation_id,
        "update_transcription_session called"
    );
    let mut current = CURRENT_CONVERSATION_ID.lock().map_err(|e| e.to_string())?;
    *current = Some(conversation_id);
    Ok(())
}

#[tauri::command]
pub async fn start_transcription(
    app: AppHandle,
    audio_config: AudioCaptureConfig,
    stt_settings: SttSettingsCommand,
) -> Result<(), String> {
    let conversation_id = {
        let current = CURRENT_CONVERSATION_ID.lock().map_err(|e| e.to_string())?;
        current.clone().ok_or("No conversation ID set")?
    };

    tracing::info!(
        conversation_id = %conversation_id,
        capture_system = audio_config.capture_system_audio,
        capture_mic = audio_config.capture_microphone,
        stt_type = ?stt_settings.stt_type,
        "start_transcription called"
    );

    if TRANSCRIPTION_RUNNING.load(Ordering::SeqCst) {
        tracing::warn!("Transcription already running");
        return Err("Transcription already running".to_string());
    }

    // Validate that at least one audio source is enabled
    if !audio_config.capture_system_audio && !audio_config.capture_microphone {
        return Err("At least one audio source must be enabled".to_string());
    }

    TRANSCRIPTION_RUNNING.store(true, Ordering::SeqCst);
    let _ = app.emit("transcription-started", ());

    // Create STT provider
    let stt_config: SttProviderConfig = stt_settings.into();
    let mut stt_provider = create_stt_provider(&stt_config)
        .map_err(|e| format!("Failed to create STT provider: {}", e))?;

    // Set up callback to receive transcription results
    let app_for_callback = app.clone();
    let callback: SttResultCallback = Arc::new(move |result: SttTranscriptResult| {
        tracing::trace!(
            text = %result.text,
            is_final = result.is_final,
            speaker = %result.speaker,
            "STT result received"
        );

        // Emit to frontend
        let _ = app_for_callback.emit("transcription-result", &result);

        // Save to database if final
        if result.is_final {
            let speaker = if result.speaker == "user" {
                Speaker::User
            } else {
                Speaker::System
            };

            let conv_id = result.conversation_id.clone();
            let text = result.text.clone();
            let confidence = result.confidence;
            let timestamp = result.timestamp;
            let id = result.id.clone();

            tokio::spawn(async move {
                let _ = tokio::task::spawn_blocking(move || {
                    transcript_repo::create(id, conv_id, speaker, text, confidence, timestamp)
                })
                .await;
            });
        }
    });

    stt_provider.set_result_callback(callback);

    // Start STT provider
    stt_provider
        .start()
        .await
        .map_err(|e| format!("Failed to start STT provider: {}", e))?;

    // Create audio input streams
    let app_for_stream = app.clone();

    tokio::spawn(async move {
        // Create audio streams based on config
        let mut system_stream: Option<AudioStream> = None;
        let mut mic_stream: Option<AudioStream> = None;

        if audio_config.capture_system_audio {
            match AudioInput::system(audio_config.system_audio_device_id.clone()) {
                Ok(input) => {
                    tracing::info!("System audio stream created");
                    system_stream = Some(input.stream());
                }
                Err(e) => {
                    tracing::error!("Failed to create system audio stream: {}", e);
                }
            }
        }

        if audio_config.capture_microphone {
            match AudioInput::microphone(audio_config.microphone_device_id.clone()) {
                Ok(input) => {
                    tracing::info!("Microphone stream created");
                    mic_stream = Some(input.stream());
                }
                Err(e) => {
                    tracing::error!("Failed to create microphone stream: {}", e);
                }
            }
        }

        // Buffer for collecting audio samples
        let mut audio_buffer: Vec<u8> = Vec::new();
        let sample_rate = 48000u32; // Default, will be updated
        let max_samples = sample_rate as usize * 2; // 2 seconds buffer
        let mut sample_count = 0usize;

        loop {
            if !TRANSCRIPTION_RUNNING.load(Ordering::SeqCst) {
                break;
            }

            let mut received_audio = false;

            // Read from system audio stream
            if let Some(ref mut stream) = system_stream {
                if let Some(sample) = stream.next().await {
                    received_audio = true;
                    // Convert f32 to i16 and add to buffer
                    let sample_i16 =
                        (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
                    audio_buffer.extend_from_slice(&sample_i16.to_le_bytes());
                    sample_count += 1;
                }
            }

            // Read from microphone stream
            if let Some(ref mut stream) = mic_stream {
                if let Some(sample) = stream.next().await {
                    received_audio = true;
                    let sample_i16 =
                        (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
                    audio_buffer.extend_from_slice(&sample_i16.to_le_bytes());
                    sample_count += 1;
                }
            }

            if !received_audio {
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                continue;
            }

            // Send audio to STT provider every 2 seconds
            if sample_count >= max_samples {
                if !audio_buffer.is_empty() {
                    if let Err(e) = stt_provider.send_audio(&audio_buffer).await {
                        tracing::error!("Failed to send audio to STT provider: {}", e);
                    }
                    audio_buffer.clear();
                    sample_count = 0;
                }
            }
        }

        // Stop STT provider
        if let Err(e) = stt_provider.stop().await {
            tracing::error!("Failed to stop STT provider: {}", e);
        }

        TRANSCRIPTION_RUNNING.store(false, Ordering::SeqCst);
        let _ = app_for_stream.emit("transcription-stopped", ());
        tracing::info!("Transcription stopped");
    });

    Ok(())
}

#[tauri::command]
pub async fn stop_transcription(app: AppHandle) -> Result<(), String> {
    tracing::info!("stop_transcription called");

    TRANSCRIPTION_RUNNING.store(false, Ordering::SeqCst);
    let _ = app.emit("transcription-stopping", ());

    tracing::info!("Transcription stop requested");
    Ok(())
}
