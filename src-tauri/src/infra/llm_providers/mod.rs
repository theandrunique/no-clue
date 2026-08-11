use crate::domain::llm::{LlmProvider, LlmProviderSettings};

mod ai_tunnel;
mod fake;
mod ollama;
mod utils;

pub use ai_tunnel::ai_tunnel_descriptor;
pub use fake::fake_provider_descriptor;
pub use ollama::ollama_descriptor;

pub fn create_llm_provider(
    settings: &LlmProviderSettings,
) -> Result<Box<dyn LlmProvider>, anyhow::Error> {
    match settings {
        LlmProviderSettings::Fake => Ok(Box::new(fake::FakeProvider)),
        LlmProviderSettings::Ollama { base_url, model } => Ok(Box::new(ollama::OllamaProvider {
            base_url: base_url
                .clone()
                .unwrap_or_else(|| "http://localhost:11434".into()),
            model: model.clone(),
            model_info: None,
        })),
        LlmProviderSettings::AiTunnel { .. } => {
            Err(anyhow::anyhow!("AiTunnel provider is not implemented yet"))
        }
    }
}
