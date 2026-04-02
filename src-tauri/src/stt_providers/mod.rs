use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod commands;
pub mod deepgram;
pub mod fake;

pub use commands::*;

use crate::audio_capture::AudioSource;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioCaptureConfig {
    pub capture_system_audio: bool,
    pub system_audio_device_id: Option<String>,
    pub capture_microphone: bool,
    pub microphone_device_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttTranscriptResult {
    pub text: String,
    pub speaker: AudioSource,
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

pub fn create_stt_provider(settings: &SttProviderSettings) -> Result<Box<dyn SttProvider>, String> {
    match settings {
        SttProviderSettings::Fake => Ok(Box::new(fake::FakeSttProvider::new())),
        SttProviderSettings::Deepgram {
            api_key,
            language,
            model,
        } => Ok(Box::new(deepgram::DeepgramProvider::new(
            api_key.clone(),
            language.clone(),
            model.clone(),
        ))),
    }
}
