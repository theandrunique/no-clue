use async_trait::async_trait;
use futures_util::Stream;
use crate::{ai_providers::{AiProvider, FieldDescriptor, FieldType, ProviderDescriptor}, models::ChatStreamEvent};
use reqwest::{Client};

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
    ) -> Result<Box<dyn Stream<Item = ChatStreamEvent> + Send + Unpin>, String> {
        let client = Client::new();

        let res = client
            .post(format!("{}/v1/chat/completions", self.base_url))
            .json(&serde_json::json!({
                "model": self.model,
                "prompt": prompt,
                "stream": true
            }))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let stream = res.bytes_stream().map(|chunk| {
            let chunk = chunk.map_err(|e| e.to_string())?;
            let text = String::from_utf8_lossy(&chunk);

            let mut result = String::new();
            for line in text.lines() {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                    if let Some(content) = json["response"].as_str() {
                        result.push_str(content);
                    }
                }
            }

            Ok(ChatStreamEvent {
                content: result,
                done: false,
            })
        });

        Ok(Box::new(stream))
    }
}
