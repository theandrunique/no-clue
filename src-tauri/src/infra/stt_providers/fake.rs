use std::time::Duration;

use async_trait::async_trait;

use crate::domain::{
    providers::ProviderDescriptor,
    stt::{AudioChunkStream, SttProvider, SttResultStream, SttTranscriptResult},
    transcripts::AudioSource,
};

pub fn fake_stt_descriptor() -> ProviderDescriptor {
    ProviderDescriptor {
        id: "fake".to_string(),
        label: "Fake (Testing)".to_string(),
        fields: vec![],
    }
}

pub struct FakeSttProvider;

impl FakeSttProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FakeSttProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SttProvider for FakeSttProvider {
    async fn transcribe(&mut self, _audio: AudioChunkStream) -> Result<SttResultStream, String> {
        let mock_phrases = [
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

        let stream = futures_util::stream::unfold(0u64, move |mut step| async move {
            tokio::time::sleep(Duration::from_millis(5000)).await;

            let phrase_index = (step / 2) as usize % mock_phrases.len();
            let text = mock_phrases[phrase_index];
            let is_final = step % 2 == 1;
            let source = if phrase_index % 2 == 0 {
                AudioSource::System
            } else {
                AudioSource::Microphone
            };

            step += 1;

            let result = SttTranscriptResult {
                text: if is_final {
                    text.to_string()
                } else {
                    text[..text.len() / 2].to_string()
                },
                is_final,
                confidence: if is_final { 0.95 } else { 0.7 },
                source,
            };

            Some((result, step))
        });

        Ok(Box::pin(stream))
    }
}
