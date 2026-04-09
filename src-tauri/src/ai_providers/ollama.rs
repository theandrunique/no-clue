use crate::{
    ai_providers::{
        AiProvider, AiRequest, AiStreamEvent, utils::{build_json_messages, truncate_json_body}
    },
    models::{FieldDescriptor, FieldType, ProviderDescriptor, TokenUsage},
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

        let url = format!("{}/v1/chat/completions", self.base_url);
        let body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "stream": true,
            "stream_options": { "include_usage": true }
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
            let mut is_finish = false;
            let mut usage: Option<TokenUsage> = None;

            for line in text.lines() {
                if line.is_empty() || line == "data: [DONE]" {
                    continue;
                }
                let data = line.strip_prefix("data: ").unwrap_or(line);
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                    if let Some(content) = json["choices"]
                        .get(0)
                        .and_then(|c| c.get("delta"))
                        .and_then(|d| d.get("content"))
                        .and_then(|c| c.as_str())
                    {
                        result.push_str(content);
                    }

                    if json["choices"]
                        .get(0)
                        .and_then(|c| c.get("finish_reason"))
                        .and_then(|f| f.as_str())
                        .map(|f| f == "stop")
                        .unwrap_or(false)
                    {
                        is_finish = true;
                    }

                    if let Some(usage_obj) = json.get("usage") {
                        let prompt = usage_obj["prompt_tokens"].as_u64().unwrap_or(0);
                        let completion = usage_obj["completion_tokens"].as_u64().unwrap_or(0);
                        let total = usage_obj["total_tokens"].as_u64().unwrap_or(prompt + completion);
                        usage = Some(TokenUsage { prompt_tokens: prompt, completion_tokens: completion, total_tokens: total });
                    }
                }
            }

            AiStreamEvent::Chunk {
                content: result,
                is_finish,
                usage
            }
        });

        Ok(Box::new(stream))
    }
}
