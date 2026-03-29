use crate::{
    ai_providers::{
        ai_tunnel::ai_tunnel_descriptor, fake::fake_provider_descriptor, ollama::ollama_descriptor,
    },
    db::ai_provider as provider_repo,
    models::Message,
};
use async_trait::async_trait;
use futures_util::Stream;
use serde::{Deserialize, Serialize};

mod ai_tunnel;
mod fake;
mod ollama;
mod utils;

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

pub fn create_provider(settings: &ProviderSettings) -> Result<Box<dyn AiProvider>, String> {
    match settings {
        ProviderSettings::Fake => Ok(Box::new(fake::FakeProvider)),
        ProviderSettings::Ollama { base_url, model } => Ok(Box::new(ollama::OllamaProvider {
            base_url: base_url
                .clone()
                .unwrap_or_else(|| "http://localhost:11434".into()),
            model: model.clone(),
        })),
        ProviderSettings::AiTunnel { .. } => Err("AiTunnel provider is not implemented yet".to_string()),
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

#[tauri::command]
pub async fn save_provider_settings(
    provider: String,
    settings: ProviderSettings,
) -> Result<(), String> {
    tracing::trace!(provider, "save_provider_settings called");
    tokio::task::spawn_blocking(move || provider_repo::upsert_provider(&provider, &settings))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_provider_settings(provider: String) -> Result<Option<ProviderSettings>, String> {
    tracing::trace!(provider, "get_provider_settings called");
    tokio::task::spawn_blocking(move || provider_repo::get_provider_settings(&provider))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}
