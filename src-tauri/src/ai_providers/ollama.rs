use async_trait::async_trait;
use futures_util::{Stream, StreamExt};
use crate::ai_providers::{AiProvider, AiRequest, AiStreamEvent, FieldDescriptor, FieldType, ProviderDescriptor};
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

fn build_ollama_messages(request: &AiRequest) -> Vec<serde_json::Value> {
    let mut messages: Vec<serde_json::Value> = Vec::new();

    println!("[Ollama] Building messages - has screenshot: {}, has system_prompt: {}, message_count: {}", 
        request.screenshot_base64.is_some(), 
        request.system_prompt.is_some(),
        request.messages.len()
    );

    if let Some(ref system_prompt) = request.system_prompt {
        messages.push(serde_json::json!({
            "role": "system",
            "content": system_prompt
        }));
    }

    let screenshot_b64 = request.screenshot_base64.clone();

    for msg in &request.messages {
        let role: &str = match msg.role {
            crate::models::MessageRole::User => "user",
            crate::models::MessageRole::Assistant => "assistant",
            crate::models::MessageRole::System => "system",
        };

        if let Some(ref ss) = screenshot_b64 {
            let ss_short = &ss[..50.min(ss.len())];
            println!("[Ollama] Adding screenshot: {}..., length: {}", ss_short, ss.len());
            messages.push(serde_json::json!({
                "role": role,
                "content": msg.content.clone(),
                "images": [ss]
            }));
        } else {
            messages.push(serde_json::json!({
                "role": role,
                "content": msg.content.clone()
            }));
        }
    }

    messages
}

#[async_trait]
impl AiProvider for OllamaProvider {
    async fn stream(
        &self,
        request: AiRequest,
    ) -> Result<Box<dyn Stream<Item = AiStreamEvent> + Send + Unpin>, String> {
        let client = Client::new();
        let messages = build_ollama_messages(&request);

        let url = format!("{}/api/chat", self.base_url);
        let body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "stream": true
        });

        println!("[Ollama] Request URL: {}", url);
        
        fn truncate_log(value: &serde_json::Value, max_len: usize) -> serde_json::Value {
            match value {
                serde_json::Value::String(s) if s.len() > max_len => {
                    serde_json::Value::String(format!("{}...[{} chars]", &s[..max_len], s.len()))
                }
                serde_json::Value::Array(arr) => {
                    serde_json::Value::Array(arr.iter().map(|v| truncate_log(v, max_len)).collect())
                }
                serde_json::Value::Object(obj) => {
                    serde_json::Value::Object(obj.iter()
                        .map(|(k, v)| (k.clone(), truncate_log(v, max_len)))
                        .collect())
                }
                _ => value.clone()
            }
        }
        
        let body_truncated = truncate_log(&body, 50);
        println!("[Ollama] Request body: {}", body_truncated);

        let res = client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        println!("[Ollama] Response status: {}", res.status());

        let stream = res.bytes_stream().map(|chunk| {
            let chunk = match chunk {
                Ok(c) => c,
                Err(e) => {
                    println!("[Ollama] Chunk error: {}", e);
                    return AiStreamEvent::Error { code: "reqwest".into(), message: e.to_string() };
                }
            };
            let text = String::from_utf8_lossy(&chunk);
            println!("[Ollama] Raw chunk: {:?}", text);

            let mut result = String::new();
            for line in text.lines() {
                if line.is_empty() || !line.starts_with('{') {
                    continue;
                }
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                    println!("[Ollama] Parsed JSON: {:?}", json);
                    
                    if let Some(content) = json["message"]["content"].as_str() {
                        result.push_str(content);
                    }
                    
                    if json["done"].as_bool() == Some(true) {
                        println!("[Ollama] Stream finished");
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

            AiStreamEvent::Chunk { content: result, is_finish: false }
        });

        Ok(Box::new(stream))
    }
}
