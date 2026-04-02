use super::{FieldDescriptor, FieldType, SttProvider, SttProviderDescriptor, SttResultCallback};
use async_trait::async_trait;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn deepgram_descriptor() -> SttProviderDescriptor {
    SttProviderDescriptor {
        id: "deepgram".to_string(),
        label: "Deepgram".to_string(),
        fields: vec![
            FieldDescriptor {
                key: "api_key".to_string(),
                label: "API Key".to_string(),
                field_type: FieldType::Password,
                required: true,
                placeholder: Some("dg...".to_string()),
            },
            FieldDescriptor {
                key: "language".to_string(),
                label: "Language".to_string(),
                field_type: FieldType::Text,
                required: false,
                placeholder: Some("en".to_string()),
            },
            FieldDescriptor {
                key: "model".to_string(),
                label: "Model".to_string(),
                field_type: FieldType::Text,
                required: false,
                placeholder: Some("nova-2".to_string()),
            },
        ],
    }
}

pub struct DeepgramProvider {
    api_key: Option<String>,
    language: Option<String>,
    model: Option<String>,
    running: Arc<AtomicBool>,
    callback: Option<SttResultCallback>,
    client: reqwest::Client,
}

impl DeepgramProvider {
    pub fn new(api_key: Option<String>, language: Option<String>, model: Option<String>) -> Self {
        Self {
            api_key,
            language,
            model,
            running: Arc::new(AtomicBool::new(false)),
            callback: None,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl SttProvider for DeepgramProvider {
    async fn start(&mut self) -> Result<(), String> {
        if self.running.load(Ordering::SeqCst) {
            return Err("Already running".to_string());
        }

        let api_key = self.api_key.clone().ok_or("API key not provided")?;

        // Test API key with a simple request
        let url = "https://api.deepgram.com/v1/projects";
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Token {}", api_key))
            .send()
            .await
            .map_err(|e| format!("Failed to connect to Deepgram: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Deepgram API error: {}", response.status()));
        }

        self.running.store(true, Ordering::SeqCst);
        tracing::info!("Deepgram STT provider started");
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), String> {
        self.running.store(false, Ordering::SeqCst);
        tracing::info!("Deepgram STT provider stopped");
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    async fn send_audio(&mut self, _audio_data: &[u8]) -> Result<(), String> {
        if !self.running.load(Ordering::SeqCst) {
            return Err("Not running".to_string());
        }

        // Deepgram uses WebSocket for real-time transcription
        // This is a simplified implementation - full WebSocket streaming
        // would need to be implemented for actual real-time transcription
        tracing::debug!("Received {} bytes of audio for Deepgram", _audio_data.len());

        // For now, just acknowledge receipt
        // Full implementation would:
        // 1. Connect to Deepgram WebSocket endpoint
        // 2. Send audio chunks
        // 3. Receive transcription results via WebSocket
        // 4. Emit events back via callback

        Ok(())
    }

    fn set_result_callback(&mut self, callback: SttResultCallback) {
        self.callback = Some(callback);
    }
}
