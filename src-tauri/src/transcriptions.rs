use crate::audio_capture::{AudioInput, AudioStream};
use crate::db::stt_provider as stt_provider_repo;
use crate::db::transcript as transcript_repo;
use crate::error::log_err;
use crate::models::TranscriptionResult;
use crate::stt_providers::{
    create_stt_provider, AudioCaptureConfig, SttProviderSettings, SttResultCallback, SttTranscriptResult,
};
use async_channel;
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

    let (system_tx, system_rx) = async_channel::bounded(1024);
    let (mic_tx, mic_rx) = async_channel::bounded(1024);

    let running = Arc::new(AtomicBool::new(true));

    if let Some(input) = system_stream {
        let running_clone = running.clone();
        let tx = system_tx.clone();
        tokio::spawn(async move {
            let mut stream = input;
            while let Some(sample) = stream.next().await {
                if !running_clone.load(Ordering::SeqCst) {
                    break;
                }
                let sample_i16 = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
                let _ = tx.send(sample_i16).await;
            }
        });
    }

    if let Some(input) = mic_stream {
        let running_clone = running.clone();
        let tx = mic_tx.clone();
        tokio::spawn(async move {
            let mut stream = input;
            while let Some(sample) = stream.next().await {
                if !running_clone.load(Ordering::SeqCst) {
                    break;
                }
                let sample_i16 = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
                let _ = tx.send(sample_i16).await;
            }
        });
    }

    drop(system_tx);
    drop(mic_tx);

    let mut system_buffer: Vec<i16> = Vec::new();
    let mut mic_buffer: Vec<i16> = Vec::new();

    loop {
        if !TRANSCRIPTION_RUNNING.load(Ordering::SeqCst) {
            break;
        }

        tokio::select! {
            result = system_rx.recv() => {
                if let Ok(sample) = result {
                    system_buffer.push(sample);
                }
            }
            result = mic_rx.recv() => {
                if let Ok(sample) = result {
                    mic_buffer.push(sample);
                }
            }
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(10)) => {}
        }

        let min_len = system_buffer.len().min(mic_buffer.len());
        let has_both = min_len >= chunk_size;
        let has_system_only = system_buffer.len() >= chunk_size && mic_buffer.len() < chunk_size;
        let has_mic_only = mic_buffer.len() >= chunk_size && system_buffer.len() < chunk_size;

        if has_both || has_system_only || has_mic_only {
            let mut audio_to_send = Vec::new();

            if has_both {
                let system_chunk: Vec<i16> = system_buffer.drain(..chunk_size).collect();
                let mic_chunk: Vec<i16> = mic_buffer.drain(..chunk_size).collect();

                audio_to_send = if let Some(ratio) = decimation_ratio {
                    let decimated_system: Vec<i16> = system_chunk.iter().step_by(ratio as usize).copied().collect();
                    let decimated_mic: Vec<i16> = mic_chunk.iter().step_by(ratio as usize).copied().collect();
                    let min_dec = decimated_system.len().min(decimated_mic.len());
                    let mut interleaved = Vec::with_capacity(min_dec * 4);
                    for i in 0..min_dec {
                        interleaved.extend_from_slice(&decimated_system[i].to_le_bytes());
                        interleaved.extend_from_slice(&decimated_mic[i].to_le_bytes());
                    }
                    interleaved
                } else {
                    let mut interleaved = Vec::with_capacity(chunk_size * 4);
                    for i in 0..chunk_size {
                        interleaved.extend_from_slice(&system_chunk[i].to_le_bytes());
                        interleaved.extend_from_slice(&mic_chunk[i].to_le_bytes());
                    }
                    interleaved
                };
            } else if has_system_only {
                let system_chunk: Vec<i16> = system_buffer.drain(..chunk_size).collect();
                audio_to_send = if let Some(ratio) = decimation_ratio {
                    let decimated: Vec<i16> = system_chunk.iter().step_by(ratio as usize).copied().collect();
                    let mut interleaved = Vec::with_capacity(decimated.len() * 4);
                    for sample in decimated {
                        interleaved.extend_from_slice(&sample.to_le_bytes());
                        interleaved.extend_from_slice(&0i16.to_le_bytes());
                    }
                    interleaved
                } else {
                    let mut interleaved = Vec::with_capacity(chunk_size * 4);
                    for sample in system_chunk {
                        interleaved.extend_from_slice(&sample.to_le_bytes());
                        interleaved.extend_from_slice(&0i16.to_le_bytes());
                    }
                    interleaved
                };
            } else if has_mic_only {
                let mic_chunk: Vec<i16> = mic_buffer.drain(..chunk_size).collect();
                audio_to_send = if let Some(ratio) = decimation_ratio {
                    let decimated: Vec<i16> = mic_chunk.iter().step_by(ratio as usize).copied().collect();
                    let mut interleaved = Vec::with_capacity(decimated.len() * 4);
                    for sample in decimated {
                        interleaved.extend_from_slice(&0i16.to_le_bytes());
                        interleaved.extend_from_slice(&sample.to_le_bytes());
                    }
                    interleaved
                } else {
                    let mut interleaved = Vec::with_capacity(chunk_size * 4);
                    for sample in mic_chunk {
                        interleaved.extend_from_slice(&0i16.to_le_bytes());
                        interleaved.extend_from_slice(&sample.to_le_bytes());
                    }
                    interleaved
                };
            }

            if !audio_to_send.is_empty() {
                tracing::debug!(bytes = audio_to_send.len(), "Sending interleaved audio chunk to STT provider");
                if let Err(e) = stt_provider.send_audio(&audio_to_send).await {
                    tracing::error!("Failed to send audio to STT provider: {}", e);
                }
            }
        }
    }

    running.store(false, Ordering::SeqCst);

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
