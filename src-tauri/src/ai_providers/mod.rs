use async_trait::async_trait;
use futures_util::Stream;
use serde::{Deserialize, Serialize};
use crate::{ai_providers::{ai_tunnel::ai_tunnel_descriptor, ollama::ollama_descriptor}, models::ChatStreamEvent};

mod ai_tunnel;
mod ollama;

#[async_trait]
pub trait AiProvider: Send + Sync {
    async fn stream(
        &self,
        prompt: String,
    ) -> Result<Box<dyn Stream<Item = ChatStreamEvent> + Send + Unpin>, String>;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ProviderConfig {
    Ollama {
        base_url: Option<String>,
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
    #[serde(rename="text")]
    Text,
    #[serde(rename="password")]
    Password,
    #[serde(rename="select")]
    Select { options: Vec<String> },
}

pub fn create_provider(config: ProviderConfig) -> Box<dyn AiProvider> {
    match config {
        ProviderConfig::Ollama { base_url, model } => {
            Box::new(OllamaProvider { base_url ?? "http://localhost:11434", model })
        }
    }
}

#[tauri::command]
pub fn get_providers() -> Vec<ProviderDescriptor> {
    vec![
        ai_tunnel_descriptor(),
        ollama_descriptor(),
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
async fn get_provider_settings(provider: String) -> Result<ProviderSettings, String> {
    println!(
        "[COMMAND] get_provider_settings called: provider={}",
        provider
    );
    Err("Not implemented".to_string())
}
