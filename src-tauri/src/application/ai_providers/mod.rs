use async_trait::async_trait;
use futures_util::Stream;
use serde::{Deserialize, Serialize};

mod ai_tunnel;
mod commands;
mod fake;
mod ollama;
mod utils;

pub use commands::*;

use crate::domain::{conversations::TokenUsage, messages::Message, ModelInfo};

#[async_trait]
pub trait AiProvider: Send + Sync {
    async fn stream(
        &self,
        request: AiRequest,
    ) -> Result<Box<dyn Stream<Item = AiStreamEvent> + Send + Unpin>, String>;
    async fn get_model_info(&self) -> Result<ModelInfo, String>;
}

pub enum AiStreamEvent {
    Chunk {
        content: String,
        is_finish: bool,
        usage: Option<TokenUsage>,
    },
    Error {
        code: String,
        message: String,
    },
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum AiProviderSettings {
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

pub fn create_ai_provider(settings: &AiProviderSettings) -> Result<Box<dyn AiProvider>, String> {
    match settings {
        AiProviderSettings::Fake => Ok(Box::new(fake::FakeProvider)),
        AiProviderSettings::Ollama { base_url, model } => Ok(Box::new(ollama::OllamaProvider {
            base_url: base_url
                .clone()
                .unwrap_or_else(|| "http://localhost:11434".into()),
            model: model.clone(),
            model_info: None,
        })),
        AiProviderSettings::AiTunnel { .. } => {
            Err("AiTunnel provider is not implemented yet".to_string())
        }
    }
}
