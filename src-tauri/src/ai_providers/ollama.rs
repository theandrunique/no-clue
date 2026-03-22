use async_trait::async_trait;
use futures_util::{Stream, StreamExt};
use crate::ai_providers::{AiProvider, AiStreamEvent, FieldDescriptor, FieldType, ProviderDescriptor};
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
        prompt: String,
    ) -> Result<Box<dyn Stream<Item = AiStreamEvent> + Send + Unpin>, String> {
        let client = Client::new();

        let res = client
            .post(format!("{}/v1/chat/completions", self.base_url))
            .json(&serde_json::json!({
                "model": self.model,
                "messages": [{"role": "user", "content": prompt}],
                "stream": true
            }))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let stream = res.bytes_stream().map(|chunk| {
            let chunk = match chunk {
                Ok(c) => c,
                Err(e) => return AiStreamEvent::Error { code: "reqwest".into(), message: e.to_string() },
            };
            let text = String::from_utf8_lossy(&chunk);

            let mut result = String::new();
            for line in text.lines() {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                    if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                        result.push_str(content);
                    }
                    if let Some(finish) = json["choices"][0]["finish_reason"].as_str() {
                        if finish != "null" {
                            return AiStreamEvent::Chunk {
                                content: String::new(),
                                is_finish: true,
                            };
                        }
                    }
                }
            }

            if result.is_empty() {
                return AiStreamEvent::Chunk {
                    content: String::new(),
                    is_finish: false,
                };
            }

            AiStreamEvent::Chunk { content: result, is_finish: false }
        });

        Ok(Box::new(stream))
    }
}
