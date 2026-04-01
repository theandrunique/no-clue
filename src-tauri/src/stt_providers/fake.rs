use super::{FieldDescriptor, FieldType, SttProvider, SttProviderDescriptor, SttSettings};
use async_trait::async_trait;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn fake_stt_descriptor() -> SttProviderDescriptor {
    SttProviderDescriptor {
        id: "fake".to_string(),
        label: "Fake (Testing)".to_string(),
        fields: vec![],
    }
}

pub struct FakeSttProvider {
    running: Arc<AtomicBool>,
}

impl FakeSttProvider {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
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
    fn descriptor(&self) -> SttProviderDescriptor {
        fake_stt_descriptor()
    }

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
        // Fake provider ignores audio data
        Ok(())
    }
}
