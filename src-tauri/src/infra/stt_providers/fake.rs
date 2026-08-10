use crate::domain::providers::ProviderDescriptor;
use crate::domain::stt::{SttResultCallback, SttTranscriptResult};
use crate::domain::transcriptions::AudioSource;

use super::SttProvider;
use async_trait::async_trait;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub fn fake_stt_descriptor() -> ProviderDescriptor {
    ProviderDescriptor {
        id: "fake".to_string(),
        label: "Fake (Testing)".to_string(),
        fields: vec![],
    }
}

pub struct FakeSttProvider {
    running: Arc<AtomicBool>,
    callback: Option<SttResultCallback>,
    call_count: Arc<AtomicUsize>,
}

impl FakeSttProvider {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            callback: None,
            call_count: Arc::new(AtomicUsize::new(0)),
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

        let call_count = self.call_count.fetch_add(1, Ordering::SeqCst);

        if call_count % 5 != 0 {
            return Ok(());
        }

        let Some(callback) = &self.callback else {
            return Ok(());
        };

        let mock_phrases = vec![
            "Can you help me with this code",
            "Let me explain what I mean",
            "That's exactly what I wanted",
            "Could you summarize this",
            "Thank you for your help",
            "System notification: Update available",
            "Email received from John",
            "Meeting starts in 5 minutes",
            "File download complete",
            "New message in Slack",
        ];

        let phrase_index = (call_count / 5) % mock_phrases.len();
        let text = &mock_phrases[phrase_index];

        let source = if phrase_index % 2 == 0 {
            AudioSource::System
        } else {
            AudioSource::Microphone
        };

        let interim_result = SttTranscriptResult {
            text: text[..text.len() / 2].to_string(),
            is_final: false,
            confidence: 0.7,
            source: source.clone(),
        };
        callback(interim_result);

        sleep(Duration::from_millis(500)).await;

        let final_result = SttTranscriptResult {
            text: text.to_string(),
            is_final: true,
            confidence: 0.95,
            source,
        };
        callback(final_result);

        Ok(())
    }

    fn set_result_callback(&mut self, callback: SttResultCallback) {
        self.callback = Some(callback);
    }
}
