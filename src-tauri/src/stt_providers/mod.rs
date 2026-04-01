use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod fake;
pub mod deepgram;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum SttProviderConfig {
    Fake,
    Deepgram {
        api_key: Option<String>,
        language: Option<String>,
        model: Option<String>,
    },
}

impl Default for SttProviderConfig {
    fn default() -> Self {
        Self::Fake
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioCaptureConfig {
    pub capture_system_audio: bool,
    pub system_audio_device_id: Option<String>,
    pub capture_microphone: bool,
    pub microphone_device_id: Option<String>,
}

impl Default for AudioCaptureConfig {
    fn default() -> Self {
        Self {
            capture_system_audio: false,
            system_audio_device_id: None,
            capture_microphone: false,
            microphone_device_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDescriptor {
    pub key: String,
    pub label: String,
    pub field_type: FieldType,
    pub required: bool,
    pub placeholder: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "select")]
    Select { options: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttProviderDescriptor {
    pub id: String,
    pub label: String,
    pub fields: Vec<FieldDescriptor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttTranscriptResult {
    pub id: String,
    pub conversation_id: String,
    pub text: String,
    pub is_final: bool,
    pub confidence: f64,
    pub speaker: String,
    pub timestamp: i64,
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

pub fn create_stt_provider(config: &SttProviderConfig) -> Result<Box<dyn SttProvider>, String> {
    match config {
        SttProviderConfig::Fake => Ok(Box::new(fake::FakeSttProvider::new())),
        SttProviderConfig::Deepgram {
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

pub fn get_stt_descriptors() -> Vec<SttProviderDescriptor> {
    vec![fake::fake_stt_descriptor(), deepgram::deepgram_descriptor()]
}