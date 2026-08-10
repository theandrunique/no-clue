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
                    let msg = Message::Binary(audio_data.into());
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
    struct Response {
        channel: Channel,
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

    let parsed: Response = serde_json::from_str(text).ok()?;
    let alternative = parsed.channel.alternatives.first()?;
    let is_final = text.contains("\"is_final\":true");

    Some(SttTranscriptResult {
        text: alternative.transcript.clone(),
        source: AudioSource::System,
        confidence: alternative.confidence.unwrap_or(0.0),
        is_final,
    })
}
