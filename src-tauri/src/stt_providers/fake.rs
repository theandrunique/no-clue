use super::{FieldDescriptor, FieldType, SttProvider, SttProviderDescriptor, SttResultCallback, SttTranscriptResult};
use async_trait::async_trait;
use chrono::Utc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use uuid::Uuid;

pub fn fake_stt_descriptor() -> SttProviderDescriptor {
    SttProviderDescriptor {
        id: "fake".to_string(),
        label: "Fake (Testing)".to_string(),
        fields: vec![],
    }
}

pub struct FakeSttProvider {
    running: Arc<AtomicBool>,
    callback: Option<SttResultCallback>,
    conversation_id: String,
}

impl FakeSttProvider {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            callback: None,
            conversation_id: String::new(),
        }
    }
}

impl Default for FakeSttProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SttProvider for FakeSttProvider {
    async fn start(&mut self) -> Result<(), String> {
        if self.running.load(Ordering::SeqCst) {
            return Err("Already running".to_string());
        }
        self.running.store(true, Ordering::SeqCst);
        tracing::info!("Fake STT provider started");
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), String> {
        self.running.store(false, Ordering::SeqCst);
        tracing::info!("Fake STT provider stopped");
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    async fn send_audio(&mut self, _audio_data: &[u8]) -> Result<(), String> {
        if !self.running.load(Ordering::SeqCst) {
            return Err("Not running".to_string());
        }

        // Simulate receiving transcription results (mock)
        // In real implementation, this would come from actual STT processing
        let Some(callback) = &self.callback else {
            return Ok(());
        };

        // Generate mock transcription
        let mock_phrases = vec![
            ("Can you help me with this code", "user"),
            ("Let me explain what I mean", "user"),
            ("That's exactly what I wanted", "user"),
            ("Could you summarize this", "user"),
            ("Thank you for your help", "user"),
            ("System notification: Update available", "system"),
            ("Email received from John", "system"),
            ("Meeting starts in 5 minutes", "system"),
            ("File download complete", "system"),
            ("New message in Slack", "system"),
        ];

        // Use a simple hash of audio data to pick a phrase (deterministic)
        let phrase_index = _audio_data.len() % mock_phrases.len();
        let (text, speaker) = &mock_phrases[phrase_index];

        // Send interim result
        let interim_result = SttTranscriptResult {
            id: Uuid::new_v4().to_string(),
            conversation_id: self.conversation_id.clone(),
            text: text[..text.len() / 2].to_string(),
            is_final: false,
            confidence: 0.7,
            speaker: speaker.to_string(),
            timestamp: Utc::now().timestamp(),
        };
        callback(interim_result);

        // Send final result after a delay (simulated)
        let final_result = SttTranscriptResult {
            id: Uuid::new_v4().to_string(),
            conversation_id: self.conversation_id.clone(),
            text: text.to_string(),
            is_final: true,
            confidence: 0.95,
            speaker: speaker.to_string(),
            timestamp: Utc::now().timestamp(),
        };
        callback(final_result);

        Ok(())
    }

    fn set_result_callback(&mut self, callback: SttResultCallback) {
        self.callback = Some(callback);
    }
}

impl FakeSttProvider {
    pub fn set_conversation_id(&mut self, conversation_id: String) {
        self.conversation_id = conversation_id;
    }
}