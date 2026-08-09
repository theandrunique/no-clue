use serde::Serialize;

pub mod conversations;
pub mod messages;
pub mod providers;
pub mod shortcuts;
pub mod system_prompts;
pub mod transcriptions;

#[derive(Clone, Serialize)]
pub struct ModelInfo {
    pub model_name: String,
    pub context_window: u64,
    pub supports_vision: bool,
}
