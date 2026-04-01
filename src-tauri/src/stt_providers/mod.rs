use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod fake;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttSettings {
    pub stt_type: SttType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SttType {
    Fake,
    Deepgram {
        api_key: Option<String>,
        language: Option<String>,
        model: Option<String>,
    },
}

impl Default for SttSettings {
    fn default() -> Self {
        Self {
            stt_type: SttType::Fake,
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
    Text,
    Password,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttProviderDescriptor {
    pub id: String,
    pub label: String,
    pub fields: Vec<FieldDescriptor>,
}

pub struct SttTranscriptResult {
    pub text: String,
    pub is_final: bool,
    pub confidence: Option<f64>,
    pub speaker: String,
}

#[async_trait]
pub trait SttProvider: Send + Sync {
    fn descriptor(&self) -> SttProviderDescriptor;

    async fn start(&mut self) -> Result<(), String>;

    async fn stop(&mut self) -> Result<(), String>;

    fn is_running(&self) -> bool;

    async fn send_audio(&mut self, audio_data: &[u8]) -> Result<(), String>;
}

pub fn create_stt_provider(settings: &SttSettings) -> Result<Box<dyn SttProvider>, String> {
    match &settings.stt_type {
        SttType::Fake => Ok(Box::new(fake::FakeSttProvider::new())),
        SttType::Deepgram {
            api_key,
            language,
            model,
        } => Err("Deepgram provider not implemented yet".to_string()),
    }
}

pub fn get_stt_descriptors() -> Vec<SttProviderDescriptor> {
    vec![fake::fake_stt_descriptor()]
}
