use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResult {
    pub id: String,
    pub conversation_id: String,
    pub source: AudioSource,
    pub text: String,
    pub is_final: bool,
    pub confidence: f64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Transcript {
    pub id: String,
    pub conversation_id: String,
    pub source: AudioSource,
    pub text: String,
    pub confidence: f64,
    pub timestamp: i64,
}

impl From<Transcript> for TranscriptionResult {
    fn from(t: Transcript) -> Self {
        TranscriptionResult {
            id: t.id,
            conversation_id: t.conversation_id,
            source: t.source,
            text: t.text,
            is_final: true,
            confidence: t.confidence,
            timestamp: t.timestamp,
        }
    }
}

impl From<TranscriptionResult> for Transcript {
    fn from(value: TranscriptionResult) -> Self {
        Transcript {
            id: value.id,
            conversation_id: value.conversation_id,
            source: value.source,
            text: value.text,
            confidence: value.confidence,
            timestamp: value.timestamp,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
pub enum AudioSource {
    System,
    Microphone,
}

impl std::fmt::Display for AudioSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioSource::System => write!(f, "system"),
            AudioSource::Microphone => write!(f, "microphone"),
        }
    }
}

impl std::str::FromStr for AudioSource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "system" => Ok(AudioSource::System),
            "microphone" => Ok(AudioSource::Microphone),
            _ => Err(format!("Unknown audio source: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioCaptureConfig {
    pub capture_system_audio: bool,
    pub system_audio_device_id: Option<String>,
    pub capture_microphone: bool,
    pub microphone_device_id: Option<String>,
}
