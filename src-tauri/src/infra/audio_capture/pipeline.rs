use crate::domain::stt::{AudioChunk, AudioChunkStream};
use crate::domain::transcripts::AudioCaptureConfig;
use crate::infra::audio_capture::{AudioInput, AudioStream};
use crate::infra::audio_processing::AudioProcessor;
use async_channel::Sender;
use futures_util::StreamExt;
use tokio_util::sync::CancellationToken;

const TARGET_SAMPLE_RATE: u32 = 16000;
const CHUNK_DURATION_MS: u32 = 100;

/// Открывает аудио-устройства по конфигурации, ресемплит/миксует семплы через
/// `AudioProcessor` и отдаёт поток готовых `AudioChunk`.
///
/// Поток завершается, когда отменён `token` либо закрылись источники аудио.
pub fn start_capture_pipeline(
    config: &AudioCaptureConfig,
    token: CancellationToken,
) -> Result<AudioChunkStream, String> {
    let mut system_stream: Option<AudioStream> = None;
    let mut mic_stream: Option<AudioStream> = None;

    if config.capture_system_audio {
        let input = AudioInput::system(config.system_audio_device_id.clone())
            .map_err(|e| format!("Failed to create system audio stream: {e}"))?;
        system_stream = Some(input.stream());
    }
    if config.capture_microphone {
        let input = AudioInput::microphone(config.microphone_device_id.clone())
            .map_err(|e| format!("Failed to create microphone stream: {e}"))?;
        mic_stream = Some(input.stream());
    }

    let actual_source_rate = system_stream
        .as_ref()
        .map(AudioStream::sample_rate)
        .or_else(|| mic_stream.as_ref().map(AudioStream::sample_rate))
        .ok_or_else(|| "No audio streams available".to_string())?;

    let processor = AudioProcessor::new(actual_source_rate, TARGET_SAMPLE_RATE, CHUNK_DURATION_MS);
    let chunk_size = processor.chunk_samples();

    let (tx, rx) = async_channel::bounded::<AudioChunk>(32);

    let capture_system = config.capture_system_audio;
    let capture_mic = config.capture_microphone;

    tokio::spawn(async move {
        run_capture_loop(
            system_stream,
            mic_stream,
            processor,
            chunk_size,
            capture_system,
            capture_mic,
            token,
            tx,
        )
        .await;
    });

    let stream = futures_util::stream::unfold(rx, |rx| async move {
        rx.recv().await.ok().map(|chunk| (chunk, rx))
    });

    Ok(Box::pin(stream))
}

async fn run_capture_loop(
    system_stream: Option<AudioStream>,
    mic_stream: Option<AudioStream>,
    mut processor: AudioProcessor,
    chunk_size: usize,
    capture_system: bool,
    capture_mic: bool,
    token: CancellationToken,
    chunk_tx: Sender<AudioChunk>,
) {
    let (system_tx, system_rx) = async_channel::bounded::<i16>(2048);
    let (mic_tx, mic_rx) = async_channel::bounded::<i16>(2048);

    if let Some(mut stream) = system_stream {
        let tx = system_tx.clone();
        let token = token.clone();
        tokio::spawn(async move {
            while let Some(sample) = stream.next().await {
                if token.is_cancelled() {
                    break;
                }
                let sample_i16 = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
                if tx.send(sample_i16).await.is_err() {
                    break;
                }
            }
            tracing::info!("System audio capture task ended");
        });
    }

    if let Some(mut stream) = mic_stream {
        let tx = mic_tx.clone();
        let token = token.clone();
        tokio::spawn(async move {
            while let Some(sample) = stream.next().await {
                if token.is_cancelled() {
                    break;
                }
                let sample_i16 = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
                if tx.send(sample_i16).await.is_err() {
                    break;
                }
            }
            tracing::info!("Microphone capture task ended");
        });
    }

    drop(system_tx);
    drop(mic_tx);

    let mut system_buffer: Vec<i16> = Vec::with_capacity(chunk_size * 2);
    let mut mic_buffer: Vec<i16> = Vec::with_capacity(chunk_size * 2);

    loop {
        if token.is_cancelled() {
            break;
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

            let audio_data = processor.process_chunk(sys_chunk.as_deref(), mic_chunk.as_deref());

            if !audio_data.is_empty() && chunk_tx.send(AudioChunk(audio_data)).await.is_err() {
                break;
            }
            continue;
        }

        tokio::select! {
            result = system_rx.recv(), if capture_system => {
                if let Ok(sample) = result {
                    system_buffer.push(sample);
                }
            }
            result = mic_rx.recv(), if capture_mic => {
                if let Ok(sample) = result {
                    mic_buffer.push(sample);
                }
            }
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(10)) => {}
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
            let _ = chunk_tx.send(AudioChunk(audio_data)).await;
        }
    }

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
}
