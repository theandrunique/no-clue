use async_trait::async_trait;
use futures_util::Stream;
use serde::{Deserialize, Serialize};

use crate::domain::{conversations::TokenUsage, messages::Message};

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn stream_chat_completion(
        &self,
        request: LlmChatCompletionRequest,
    ) -> Result<Box<dyn Stream<Item = LlmChatCompletionResult> + Send + Unpin>, anyhow::Error>;

    async fn get_model_info(&self) -> Result<ModelInfo, anyhow::Error>;
}

pub enum LlmChatCompletionResult {
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

pub struct LlmChatCompletionRequest {
    pub messages: Vec<Message>,
    pub system_prompt: Option<String>,
    pub screenshot_base64: Option<String>,
}

impl LlmChatCompletionRequest {
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
pub enum LlmProviderSettings {
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

#[derive(Clone, Serialize)]
pub struct ModelInfo {
    pub model_name: String,
    pub context_window: u64,
    pub supports_vision: bool,
}
