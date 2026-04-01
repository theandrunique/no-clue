use crate::ai_providers::{
    utils::{build_json_messages, truncate_json_body},
    AiProvider, AiRequest, AiStreamEvent, FieldDescriptor, FieldType, ProviderDescriptor,
};
use async_trait::async_trait;
use futures_util::{Stream, StreamExt};
use reqwest::Client;

pub fn ollama_descriptor() -> ProviderDescriptor {
    ProviderDescriptor {
        id: "ollama".into(),
        label: "Ollama".into(),
        fields: vec![
            FieldDescriptor {
                key: "base_url".into(),
                label: "Base URL".into(),
                field_type: FieldType::Text,
                required: false,
                placeholder: Some("http://localhost:11434".into()),
            },
            FieldDescriptor {
                key: "model".into(),
                label: "Model".into(),
                field_type: FieldType::Text,
                required: true,
                placeholder: Some("llama3".into()),
            },
        ],
    }
}

pub struct OllamaProvider {
    pub base_url: String,
    pub model: String,
}

#[async_trait]
impl AiProvider for OllamaProvider {
    async fn stream(
        &self,
        request: AiRequest,
    ) -> Result<Box<dyn Stream<Item = AiStreamEvent> + Send + Unpin>, String> {
        let client = Client::new();
        let messages = build_json_messages(&request);

        let url = format!("{}/api/chat", self.base_url);
        let body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "stream": true
        });

        tracing::trace!(url = %url, "Sending request to Ollama");

        let body_truncated = truncate_json_body(&body, 50);
        tracing::trace!(body = %body_truncated, "Request body");

        let res = client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        tracing::trace!(status = %res.status(), "Response status");

        let stream = res.bytes_stream().map(|chunk| {
            let chunk = match chunk {
                Ok(c) => c,
                Err(e) => {
                    tracing::error!(error = %e, "Chunk error");
                    return AiStreamEvent::Error {
                        code: "reqwest".into(),
                        message: e.to_string(),
                    };
                }
            };
            let text = String::from_utf8_lossy(&chunk);
            tracing::trace!(chunk = %text, "Raw chunk");

            let mut result = String::new();
            for line in text.lines() {
                if line.is_empty() || !line.starts_with('{') {
                    continue;
                }
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                    tracing::trace!(json = %json, "Parsed JSON");

                    if let Some(content) = json["message"]["content"].as_str() {
                        result.push_str(content);
                    }

                    if json["done"].as_bool() == Some(true) {
                        tracing::trace!("Stream finished");
                        return AiStreamEvent::Chunk {
                            content: String::new(),
                            is_finish: true,
                        };
                    }
                }
            }

            if result.is_empty() {
                return AiStreamEvent::Chunk {
                    content: String::new(),
                    is_finish: false,
                };
            }

            AiStreamEvent::Chunk {
                content: result,
                is_finish: false,
            }
        });

        Ok(Box::new(stream))
    }
}
