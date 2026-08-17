use async_trait::async_trait;
use futures_util::Stream;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

use crate::domain::transcripts::AudioSource;

/// Кусок захваченного звука в формате PCM16 (signed 16-bit LE), stereo interleaved
/// ([L,R,L,R,...]), 16 кГц — именно то, что выдаёт audio_processing.
pub struct AudioChunk(pub Vec<u8>);

pub type AudioChunkStream = Pin<Box<dyn Stream<Item = AudioChunk> + Send>>;
pub type SttResultStream = Pin<Box<dyn Stream<Item = SttTranscriptResult> + Send>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttTranscriptResult {
    pub text: String,
    pub source: AudioSource,
    pub is_final: bool,
    pub confidence: f64,
}

#[async_trait]
pub trait SttProvider: Send + Sync {
    /// Подаёт поток аудио, возвращает поток результатов транскрибации.
    /// Жизнь сессии = время жизни потоков; закрытие = дроп стрима/провайдера.
    async fn transcribe(&mut self, audio: AudioChunkStream) -> Result<SttResultStream, String>;
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
