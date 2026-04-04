use crate::audio_capture::{AudioInput, AudioStream};
use crate::audio_processing::AudioProcessor;
use crate::db::stt_provider as stt_provider_repo;
use crate::db::transcript as transcript_repo;
use crate::error::log_err;
use crate::models::TranscriptionResult;
use crate::stt_providers::{
    create_stt_provider, AudioCaptureConfig, SttProviderSettings, SttResultCallback,
    SttTranscriptResult,
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
        SttProviderSettings::Deepgram {
            api_key,
            language,
            model,
        } => {
            let masked_key = api_key
                .as_ref()
                .map(|k| format!("{}****", &k[..4.min(k.len())]));
            format!(
                "Deepgram {{ api_key: {:?}, language: {:?}, model: {:?} }}",
                masked_key, language, model
            )
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
                    let stream = input.stream();
                    tracing::info!(
                        "System audio stream created, sample rate: {}",
                        stream.sample_rate()
                    );
                    system_stream = Some(stream);
                }
                Err(e) => {
                    tracing::error!("Failed to create system audio stream: {}", e);
                }
            }
        }

        if audio_config.capture_microphone {
            match AudioInput::microphone(audio_config.microphone_device_id.clone()) {
                Ok(input) => {
                    let stream = input.stream();
                    tracing::info!(
                        "Microphone stream created, sample rate: {}",
                        stream.sample_rate()
                    );
                    mic_stream = Some(stream);
                }
                Err(e) => {
                    tracing::error!("Failed to create microphone stream: {}", e);
                }
            }
        }

        let actual_source_rate = if let Some(ref stream) = system_stream {
            stream.sample_rate()
        } else if let Some(ref stream) = mic_stream {
            stream.sample_rate()
        } else {
            tracing::error!("No audio streams available");
            return;
        };

        const TARGET_SAMPLE_RATE: u32 = 16000;
        const CHUNK_DURATION_MS: u32 = 100;

        let mut processor =
            AudioProcessor::new(actual_source_rate, TARGET_SAMPLE_RATE, CHUNK_DURATION_MS);
        let chunk_size = processor.chunk_samples();

        let (system_tx, system_rx) = async_channel::bounded(2048);
        let (mic_tx, mic_rx) = async_channel::bounded(2048);

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
                tracing::info!("System audio capture task ended");
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
                tracing::info!("Microphone capture task ended");
            });
        }

        drop(system_tx);
        drop(mic_tx);

        let mut system_buffer: Vec<i16> = Vec::with_capacity(chunk_size * 2);
        let mut mic_buffer: Vec<i16> = Vec::with_capacity(chunk_size * 2);

        let capture_system = audio_config.capture_system_audio;
        let capture_mic = audio_config.capture_microphone;

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

            let has_system = capture_system && system_buffer.len() >= chunk_size;
            let has_mic = capture_mic && mic_buffer.len() >= chunk_size;

            let should_process = if capture_system && capture_mic {
                has_system && has_mic
            } else {
                has_system || has_mic
            };

            if should_process {
                let sys_chunk = if has_system {
                    Some(system_buffer.drain(..chunk_size).collect::<Vec<_>>())
                } else {
                    None
                };
                let mic_chunk = if has_mic {
                    Some(mic_buffer.drain(..chunk_size).collect::<Vec<_>>())
                } else {
                    None
                };

                let audio_data =
                    processor.process_chunk(sys_chunk.as_deref(), mic_chunk.as_deref());

                if !audio_data.is_empty() {
                    if let Err(e) = stt_provider.send_audio(&audio_data).await {
                        tracing::error!("Failed to send audio to STT provider: {}", e);
                    }
                }
            }
        }

        if !system_buffer.is_empty() || !mic_buffer.is_empty() {
            let sys_chunk = if !system_buffer.is_empty() && capture_system {
                Some(std::mem::take(&mut system_buffer))
            } else {
                None
            };
            let mic_chunk = if !mic_buffer.is_empty() && capture_mic {
                Some(std::mem::take(&mut mic_buffer))
            } else {
                None
            };
            let audio_data = processor.process_chunk(sys_chunk.as_deref(), mic_chunk.as_deref());
            if !audio_data.is_empty() {
                let _ = stt_provider.send_audio(&audio_data).await;
            }
        }

        running.store(false, Ordering::SeqCst);

        let metrics = processor.get_metrics();
        tracing::info!(
            chunks = metrics.chunks_processed,
            bytes_sent = metrics.bytes_sent,
            system_chunks = metrics.system_chunks,
            mic_chunks = metrics.mic_chunks,
            mixed_chunks = metrics.mixed_chunks,
            underruns = metrics.buffer_underruns,
            "Audio processing metrics"
        );

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
