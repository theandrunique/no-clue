use crate::models::AudioSource;
use crate::models::{FieldDescriptor, FieldType, ProviderDescriptor};
use async_channel::Sender;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;

use super::{SttProvider, SttResultCallback, SttTranscriptResult};
use async_trait::async_trait;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Message;

const TARGET_SAMPLE_RATE: u32 = 16000;

fn convert_f32_to_i16(samples: &[f32]) -> Vec<u8> {
    let mut result = Vec::with_capacity(samples.len() * 2);
    for &sample in samples {
        let s = sample.max(-1.0).min(1.0);
        let i16 = (s * 32767.0) as i16;
        result.extend_from_slice(&i16.to_le_bytes());
    }
    result
}

pub fn deepgram_descriptor() -> ProviderDescriptor {
    ProviderDescriptor {
        id: "deepgram".to_string(),
        label: "Deepgram".to_string(),
        fields: vec![
            FieldDescriptor {
                key: "api_key".to_string(),
                label: "API Key".to_string(),
                field_type: FieldType::Password,
                required: true,
                placeholder: Some("dg...".to_string()),
            },
            FieldDescriptor {
                key: "language".to_string(),
                label: "Language".to_string(),
                field_type: FieldType::Text,
                required: false,
                placeholder: Some("en".to_string()),
            },
            FieldDescriptor {
                key: "model".to_string(),
                label: "Model".to_string(),
                field_type: FieldType::Text,
                required: false,
                placeholder: Some("nova-2".to_string()),
            },
        ],
    }
}

pub struct DeepgramProvider {
    api_key: Option<String>,
    language: Option<String>,
    model: Option<String>,
    running: Arc<AtomicBool>,
    callback: Option<SttResultCallback>,
    ws_task: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
    audio_sender: Arc<Mutex<Option<Sender<Vec<u8>>>>>,
}

impl DeepgramProvider {
    pub fn new(api_key: Option<String>, language: Option<String>, model: Option<String>) -> Self {
        Self {
            api_key,
            language,
            model,
            running: Arc::new(AtomicBool::new(false)),
            callback: None,
            ws_task: Arc::new(Mutex::new(None)),
            audio_sender: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl SttProvider for DeepgramProvider {
    async fn start(&mut self) -> Result<(), String> {
        if self.running.load(Ordering::SeqCst) {
            return Err("Already running".to_string());
        }

        let api_key = self.api_key.clone().ok_or("API key not provided")?;
        let language = self.language.clone().unwrap_or_else(|| "en".to_string());
        let model = self.model.clone().unwrap_or_else(|| "nova-2".to_string());

        let url = format!(
            "wss://api.deepgram.com/v1/listen?language={}&model={}&encoding=linear16&sample_rate=16000&channels=2&multichannel=true&interim_results=true&punctuate=true",
            language, model
        );

        tracing::info!("Connecting to Deepgram WebSocket...");

        let mut request = url
            .into_client_request()
            .map_err(|e| {
                tracing::error!("Failed to create request: {}", e);
                format!("Failed to create request: {}", e)
            })?;
        request.headers_mut().insert(
            "Authorization",
            format!("Token {}", api_key)
                .parse()
                .map_err(|e| {
                    tracing::error!("Failed to parse header: {}", e);
                    format!("Failed to parse header: {}", e)
                })?,
        );

        let (ws_stream, response) = connect_async(request)
            .await
            .map_err(|e| {
                tracing::error!("Failed to connect to Deepgram: {}", e);
                format!("Failed to connect to Deepgram: {}", e)
            })?;

        tracing::info!("Connected to Deepgram WebSocket, response: {:?}", response.status());

        let (mut write, mut read) = ws_stream.split();

        let (tx, rx) = async_channel::bounded(32);

        {
            let mut sender = self.audio_sender.lock().map_err(|e| e.to_string())?;
            *sender = Some(tx);
        }

        let callback = self.callback.clone();
        let running = self.running.clone();
        let running_for_audio = running.clone();

        let task = tokio::spawn(async move {
            let running_for_audio = running_for_audio.clone();
            let audio_task = tokio::spawn(async move {
                while let Ok(audio_data) = rx.recv().await {
                    if !running_for_audio.load(Ordering::SeqCst) {
                        break;
                    }
                    let msg = Message::Binary(audio_data.into());
                    if let Err(e) = write.send(msg).await {
                        tracing::error!("Failed to send audio to Deepgram: {}", e);
                        break;
                    }
                }
            });

            while let Some(msg) = read.next().await {
                if !running.load(Ordering::SeqCst) {
                    break;
                }

                match msg {
                    Ok(Message::Text(text)) => {
                        if let Some(result) = parse_deepgram_response(&text) {
                            if let Some(ref cb) = callback {
                                cb(result);
                            }
                        }
                    }
                    Ok(Message::Close(_)) => {
                        tracing::info!("Deepgram WebSocket closed");
                        break;
                    }
                    Ok(Message::Binary(_)) => {}
                    Err(e) => {
                        tracing::error!("Deepgram WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }

            audio_task.abort();
            running.store(false, Ordering::SeqCst);
        });

        {
            let mut task_guard = self.ws_task.lock().map_err(|e| e.to_string())?;
            *task_guard = Some(task);
        }

        self.running.store(true, Ordering::SeqCst);
        tracing::info!("Deepgram STT provider started");
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), String> {
        self.running.store(false, Ordering::SeqCst);

        {
            let mut sender = self.audio_sender.lock().map_err(|e| e.to_string())?;
            let _ = sender.take();
        }

        let task = {
            let mut task_guard = self.ws_task.lock().map_err(|e| e.to_string())?;
            task_guard.take()
        };

        if let Some(task) = task {
            let _ = task.await;
        }

        tracing::info!("Deepgram STT provider stopped");
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    async fn send_audio(&mut self, audio_data: &[u8]) -> Result<(), String> {
        if !self.running.load(Ordering::SeqCst) {
            return Err("Not running".to_string());
        }

        let tx = {
            let sender = self.audio_sender.lock().map_err(|e| e.to_string())?;
            sender.clone().ok_or("Audio sender not initialized")?
        };

        tx.send(audio_data.to_vec())
            .await
            .map_err(|e| format!("Failed to send audio: {}", e))?;

        Ok(())
    }

    fn set_result_callback(&mut self, callback: SttResultCallback) {
        self.callback = Some(callback);
    }
}

fn parse_deepgram_response(text: &str) -> Option<SttTranscriptResult> {
    #[derive(Deserialize)]
    struct DeepgramResponse {
        #[serde(rename = "type")]
        msg_type: Option<String>,
        #[serde(rename = "is_final")]
        is_final: Option<bool>,
        channel_index: Option<ChannelIndex>,
        channel: Option<Channel>,
    }

    #[derive(Deserialize)]
    struct ChannelIndex {
        #[serde(rename = "0")]
        channel: usize,
        #[serde(rename = "1")]
        total: Option<usize>,
    }

    #[derive(Deserialize)]
    struct Channel {
        alternatives: Vec<Alternative>,
    }

    #[derive(Deserialize)]
    struct Alternative {
        transcript: String,
        confidence: Option<f64>,
    }

    let response: DeepgramResponse = match serde_json::from_str(text) {
        Ok(r) => r,
        Err(_) => return None,
    };

    if response.msg_type.as_deref() == Some("Metadata") {
        return None;
    }

    let is_final_result = response.is_final.unwrap_or(false);

    let source = match response.channel_index {
        Some(idx) if idx.channel == 0 => AudioSource::System,
        Some(idx) if idx.channel == 1 => AudioSource::Microphone,
        _ => AudioSource::System,
    };

    if let Some(channel) = response.channel {
        if let Some(alt) = channel.alternatives.first() {
            let transcript = alt.transcript.trim();
            if transcript.is_empty() {
                return None;
            }

            tracing::trace!(
                text = transcript,
                is_final = is_final_result,
                source = ?source,
                confidence = alt.confidence,
                "Deepgram response received"
            );

            return Some(SttTranscriptResult {
                text: transcript.to_string(),
                is_final: is_final_result,
                confidence: alt.confidence.unwrap_or(0.0),
                source,
            });
        }
    }

    None
}
