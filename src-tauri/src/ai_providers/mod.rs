use async_trait::async_trait;
use futures_util::Stream;
use serde::{Deserialize, Serialize};
use crate::{ai_providers::{fake::fake_provider_descriptor, ollama::ollama_descriptor, ai_tunnel::ai_tunnel_descriptor}, models::Message};

mod ai_tunnel;
mod fake;
mod ollama;

#[async_trait]
pub trait AiProvider: Send + Sync {
    async fn stream(
        &self,
        request: AiRequest,
    ) -> Result<Box<dyn Stream<Item = AiStreamEvent> + Send + Unpin>, String>;
}

pub struct AiRequest {
    pub messages: Vec<Message>,
    pub system_prompt: Option<String>,
    pub screenshot_base64: Option<String>,
}

impl AiRequest {
    pub fn new(messages: Vec<Message>) -> Self {
        Self {
            messages,
            system_prompt: None,
            screenshot_base64: None,
        }
    }

    pub fn with_system_prompt(mut self, prompt: String) -> Self {
        self.system_prompt = Some(prompt);
        self
    }

    pub fn with_screenshot(mut self, screenshot_base64: String) -> Self {
        self.screenshot_base64 = Some(screenshot_base64);
        self
    }
}

pub enum AiStreamEvent {
    Chunk { content: String, is_finish: bool },
    Error { code: String, message: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ProviderSettings {
    Fake,
    Ollama {
        base_url: Option<String>,
        model: String,
    },
    AiTunnel {
        api_key: String,
        model: String,
    },
}

#[derive(Serialize)]
pub struct ProviderDescriptor {
    pub id: String,
    pub label: String,
    pub fields: Vec<FieldDescriptor>,
}

#[derive(Serialize)]
pub struct FieldDescriptor {
    pub key: String,
    pub label: String,
    pub field_type: FieldType,
    pub required: bool,
    pub placeholder: Option<String>,
}

#[derive(Serialize)]
pub enum FieldType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "select")]
    Select { options: Vec<String> },
}

pub fn create_provider(settings: &ProviderSettings) -> Box<dyn AiProvider> {
    match settings {
        ProviderSettings::Fake => Box::new(fake::FakeProvider),
        ProviderSettings::Ollama { base_url, model } => {
            Box::new(ollama::OllamaProvider {
                base_url: base_url.clone().unwrap_or_else(|| "http://localhost:11434".into()),
                model: model.clone(),
            })
        }
        ProviderSettings::AiTunnel { .. } => {
            Box::new(fake::FakeProvider)
        }
    }
}

#[tauri::command]
pub fn get_providers() -> Vec<ProviderDescriptor> {
    vec![
        fake_provider_descriptor(),
        ollama_descriptor(),
        ai_tunnel_descriptor(),
    ]
}
