use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;

use crate::domain::providers::{FieldDescriptor, FieldType, ProviderDescriptor};
use crate::domain::stt::{
    AudioChunk, AudioChunkStream, SttProvider, SttResultStream, SttTranscriptResult,
};
use crate::domain::transcriptions::AudioSource;

use async_trait::async_trait;
use tokio::task::JoinHandle;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Message;

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
    task: Option<JoinHandle<()>>,
}

impl DeepgramProvider {
    pub fn new(api_key: Option<String>, language: Option<String>, model: Option<String>) -> Self {
        Self {
            api_key,
            language,
            model,
            task: None,
        }
    }
}

impl Drop for DeepgramProvider {
    fn drop(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }
}

#[async_trait]
impl SttProvider for DeepgramProvider {
    async fn transcribe(&mut self, mut audio: AudioChunkStream) -> Result<SttResultStream, String> {
        let api_key = self.api_key.clone().ok_or("API key not provided")?;
        let language = self.language.clone().unwrap_or_else(|| "en".to_string());
        let model = self.model.clone().unwrap_or_else(|| "nova-2".to_string());

        let url = format!(
            "wss://api.deepgram.com/v1/listen?language={}&model={}&encoding=linear16&sample_rate=16000&channels=2&multichannel=true&interim_results=true&punctuate=true",
            language, model
        );

        tracing::info!("Connecting to Deepgram WebSocket...");

        let mut request = url.into_client_request().map_err(|e| {
            tracing::error!("Failed to create request: {}", e);
            format!("Failed to create request: {}", e)
        })?;
        request.headers_mut().insert(
            "Authorization",
            format!("Token {}", api_key).parse().map_err(|e| {
                tracing::error!("Failed to parse header: {}", e);
                format!("Failed to parse header: {}", e)
            })?,
        );

        let (ws_stream, response) = connect_async(request).await.map_err(|e| {
            tracing::error!("Failed to connect to Deepgram: {}", e);
            format!("Failed to connect to Deepgram: {}", e)
        })?;

        tracing::info!(
            "Connected to Deepgram WebSocket, response: {:?}",
            response.status()
        );

        let (mut write, mut read) = ws_stream.split();
        let (tx, rx) = async_channel::bounded::<SttTranscriptResult>(32);

        let task = tokio::spawn(async move {
            let audio_task = tokio::spawn(async move {
                while let Some(AudioChunk(audio_data)) = audio.next().await {
                    let msg = Message::Binary(audio_data);
                    if let Err(e) = write.send(msg).await {
                        tracing::error!("Failed to send audio to Deepgram: {}", e);
                        break;
                    }
                }
                // Весь аудио-поток исчерпан — закрываем websocket.
                let _ = write.close().await;
            });

            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        tracing::trace!(
                            message_len = text.len(),
                            message = %text,
                            "Deepgram WebSocket text message received"
                        );
                        if let Some(result) = parse_deepgram_response(&text) {
                            if tx.send(result).await.is_err() {
                                break;
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
        });

        if let Some(old) = self.task.replace(task) {
            old.abort();
        }

        let stream = futures_util::stream::unfold(rx, |rx| async move {
            rx.recv().await.ok().map(|result| (result, rx))
        });

        Ok(Box::pin(stream))
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
        #[allow(dead_code)]
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
        Err(e) => {
            tracing::warn!(error = %e, message = %text, "Failed to parse Deepgram response");
            return None;
        }
    };

    if response.msg_type.as_deref() == Some("Metadata") {
        tracing::trace!("Deepgram metadata message received");
        return None;
    }

    let is_final_result = response.is_final.unwrap_or(false);

    let source = match response.channel_index {
        Some(idx) if idx.channel == 0 => AudioSource::System,
        Some(idx) if idx.channel == 1 => AudioSource::Microphone,
        _ => {
            tracing::warn!("Unexpected channel_index in response");
            AudioSource::System
        },
    };

    if let Some(channel) = response.channel {
        if let Some(alt) = channel.alternatives.first() {
            let transcript = alt.transcript.trim();
            if transcript.is_empty() {
                tracing::trace!("Empty Deepgram transcript skipped");
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
