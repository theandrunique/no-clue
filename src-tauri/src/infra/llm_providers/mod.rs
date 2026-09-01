mod ai_tunnel;
mod fake;
mod ollama;
mod utils;

use crate::domain::chats::{LlmProvider, LlmProviderSettings};

pub fn create_llm_provider(
    settings: &LlmProviderSettings,
) -> Result<Box<dyn LlmProvider>, anyhow::Error> {
    match settings {
        LlmProviderSettings::TestingProvider => Ok(Box::new(fake::FakeProvider)),
        LlmProviderSettings::AiTunnel { .. } => {
            Err(anyhow::anyhow!("AiTunnel provider is not implemented yet"))
        }
    }
}
