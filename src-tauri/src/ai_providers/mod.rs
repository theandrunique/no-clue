use async_trait::async_trait;
use futures_util::Stream;
use serde::{Deserialize, Serialize};
use crate::{ai_providers::{ai_tunnel::ai_tunnel_descriptor, fake::fake_provider_descriptor, ollama::ollama_descriptor}};

mod ai_tunnel;
mod fake;
mod ollama;

#[async_trait]
pub trait AiProvider: Send + Sync {
    async fn stream(
        &self,
        prompt: String,
    ) -> Result<Box<dyn Stream<Item = AiStreamEvent> + Send + Unpin>, String>;
}

pub enum AiStreamEvent {
    Chunk { content: String, is_finish: bool },
    Error { code: String, message: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderConfig {
    pub provider: String,
    #[serde(flatten)]
    pub settings: ProviderSettings,
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

impl ProviderConfig {
    pub fn fake() -> Self {
        Self {
            provider: "fake".into(),
            settings: ProviderSettings::Fake,
        }
    }

    pub fn ollama(base_url: Option<String>, model: String) -> Self {
        Self {
            provider: "ollama".into(),
            settings: ProviderSettings::Ollama { base_url, model },
        }
    }
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
    #[serde(rename="text")]
    Text,
    #[serde(rename="password")]
    Password,
    #[serde(rename="select")]
    Select { options: Vec<String> },
}

pub fn create_provider(config: &ProviderConfig) -> Box<dyn AiProvider> {
    match &config.settings {
        ProviderSettings::Fake => Box::new(fake::FakeProvider),
        ProviderSettings::Ollama { base_url, model } => {
            Box::new(ollama::OllamaProvider {
                base_url: base_url.clone().unwrap_or_else(|| "http://localhost:11434".into()),
                model: model.clone()
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

#[tauri::command]
async fn save_provider_settings(
    provider: String,
    _api_key: String,
    model: String,
) -> Result<(), String> {
    println!(
        "[COMMAND] save_provider_settings called: provider={}, model={}",
        provider, model
    );
    Ok(())
}

#[tauri::command]
async fn get_provider_settings(provider: String) -> Result<ProviderConfig, String> {
    println!(
        "[COMMAND] get_provider_settings called: provider={}",
        provider
    );
    Err("Not implemented".to_string())
}
