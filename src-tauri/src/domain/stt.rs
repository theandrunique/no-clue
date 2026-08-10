use crate::domain::transcriptions::AudioSource;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttTranscriptResult {
    pub text: String,
    pub source: AudioSource,
    pub is_final: bool,
    pub confidence: f64,
}

pub type SttResultCallback = Arc<dyn Fn(SttTranscriptResult) + Send + Sync>;

#[async_trait]
pub trait SttProvider: Send + Sync {
    async fn start(&mut self) -> Result<(), String>;

    async fn stop(&mut self) -> Result<(), String>;

    fn is_running(&self) -> bool;

    async fn send_audio(&mut self, audio_data: &[u8]) -> Result<(), String>;

    fn set_result_callback(&mut self, callback: SttResultCallback);
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum SttProviderSettings {
    Fake,
    Deepgram {
        api_key: Option<String>,
        language: Option<String>,
        model: Option<String>,
    },
}
