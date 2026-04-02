use crate::audio_capture::{AudioInput, AudioStream};
use crate::db::stt_provider as stt_provider_repo;
use crate::db::transcript as transcript_repo;
use crate::error::log_err;
use crate::models::TranscriptionResult;
use crate::stt_providers::{
    create_stt_provider, AudioCaptureConfig, SttProviderSettings, SttResultCallback, SttTranscriptResult,
};
use futures_util::StreamExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};
use uuid::Uuid;

static TRANSCRIPTION_RUNNING: AtomicBool = AtomicBool::new(false);
static CURRENT_CONVERSATION_ID: Mutex<Option<String>> = Mutex::new(None);

#[tauri::command]
pub async fn update_transcription_session(conversation_id: String) -> Result<(), String> {
    tracing::trace!(conversation_id, "update_transcription_session called");
    let mut current = CURRENT_CONVERSATION_ID.lock().map_err(|e| e.to_string())?;
    *current = Some(conversation_id);
    Ok(())
}

#[tauri::command]
pub async fn start_transcription(
    app: AppHandle,
    stt_provider: String,
    audio_config: AudioCaptureConfig,
) -> Result<(), String> {
    let conversation_id = {
        let current = CURRENT_CONVERSATION_ID.lock().map_err(|e| e.to_string())?;
        current.clone().ok_or("No conversation ID set")?
    };

    let stt_provider_for_error = stt_provider.clone();

    let stt_settings =
        tokio::task::spawn_blocking(move || stt_provider_repo::get_stt_settings(&stt_provider))
            .await
            .map_err(|e| log_err(e, "get_stt_settings"))?
            .map_err(|e| log_err(e, "get_stt_settings"))?
            .ok_or_else(|| {
                log_err(
                    format!("STT provider '{}' not configured", stt_provider_for_error),
                    "get_stt_settings",
                )
            })?;

    let stt_type_log = match &stt_settings {
        SttProviderSettings::Fake => "Fake".to_string(),
        SttProviderSettings::Deepgram { api_key, language, model } => {
            let masked_key = api_key.as_ref().map(|k| format!("{}****", &k[..4.min(k.len())]));
            format!("Deepgram {{ api_key: {:?}, language: {:?}, model: {:?} }}", masked_key, language, model)
        }
    };

    tracing::info!(
        conversation_id,
        capture_system = audio_config.capture_system_audio,
        capture_mic = audio_config.capture_microphone,
        stt_type = %stt_type_log,
        "start_transcription called"
    );

    if TRANSCRIPTION_RUNNING.load(Ordering::SeqCst) {
        tracing::warn!("Transcription already running");
        return Err("Transcription already running".to_string());
    }

    if !audio_config.capture_system_audio && !audio_config.capture_microphone {
        return Err("At least one audio source must be enabled".to_string());
    }

    TRANSCRIPTION_RUNNING.store(true, Ordering::SeqCst);
    let _ = app.emit("transcription-started", ());

    let mut stt_provider = create_stt_provider(&stt_settings)
        .map_err(|e| format!("Failed to create STT provider: {}", e))?;

    let app_for_callback = app.clone();
    let conversation_id_clone = conversation_id.clone();
    let callback: SttResultCallback = Arc::new(move |result: SttTranscriptResult| {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let id = Uuid::new_v4().to_string();

        let result_with_metadata = TranscriptionResult {
            id: id.clone(),
            conversation_id: conversation_id_clone.clone(),
            text: result.text.clone(),
            is_final: result.is_final,
            confidence: result.confidence,
            source: result.source.clone(),
            timestamp,
        };

        tracing::trace!(
            text = %result.text,
            is_final = result.is_final,
            source = ?result.source,
            "STT result received"
        );

        let _ = app_for_callback.emit("transcription-result", &result_with_metadata);

        if result.is_final {
            let source_enum = result.source.clone();

            let text = result.text.clone();
            let confidence = result.confidence;
            let id = id.clone();
            let conv_id = conversation_id_clone.clone();

            tokio::spawn(async move {
                let _ = tokio::task::spawn_blocking(move || {
                    transcript_repo::create(id, conv_id, source_enum, text, confidence, timestamp)
                })
                .await;
            });
        }
    });

    stt_provider.set_result_callback(callback);

    stt_provider
        .start()
        .await
        .map_err(|e| format!("Failed to start STT provider: {}", e))?;

    let app_for_stream = app.clone();

    tokio::spawn(async move {
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

        let source_sample_rate = 48000u32;
        let target_sample_rate = 16000u32;
        let decimation_ratio = if source_sample_rate != target_sample_rate {
            Some(source_sample_rate / target_sample_rate)
        } else {
            None
        };
        let chunk_size = (source_sample_rate / 10) as usize;

        let mut audio_buffer: Vec<u8> = Vec::new();
        let mut sample_count = 0usize;

        loop {
            if !TRANSCRIPTION_RUNNING.load(Ordering::SeqCst) {
                break;
            }

            let mut received_audio = false;

            if let Some(ref mut stream) = system_stream {
                if let Some(sample) = stream.next().await {
                    received_audio = true;
                    tracing::trace!("System audio sample received");
                    let sample_i16 = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
                    audio_buffer.extend_from_slice(&sample_i16.to_le_bytes());
                    sample_count += 1;
                }
            }

            if let Some(ref mut stream) = mic_stream {
                if let Some(sample) = stream.next().await {
                    received_audio = true;
                    tracing::trace!("Microphone sample received");
                    let sample_i16 = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
                    audio_buffer.extend_from_slice(&sample_i16.to_le_bytes());
                    sample_count += 1;
                }
            }

            if !received_audio {
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                continue;
            }

            if sample_count >= chunk_size {
                if !audio_buffer.is_empty() {
                    let audio_to_send: Vec<u8> = if let Some(ratio) = decimation_ratio {
                        audio_buffer
                            .chunks_exact((ratio * 2) as usize)
                            .flat_map(|chunk| {
                                let sample_i16 = i16::from_le_bytes([chunk[0], chunk[1]]);
                                sample_i16.to_le_bytes()
                            })
                            .collect()
                    } else {
                        audio_buffer.clone()
                    };

                    tracing::trace!(bytes = audio_to_send.len(), "Sending audio chunk to STT provider");

                    if let Err(e) = stt_provider.send_audio(&audio_to_send).await {
                        tracing::error!("Failed to send audio to STT provider: {}", e);
                    }
                    audio_buffer.clear();
                    sample_count = 0;
                }
            }
        }

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
