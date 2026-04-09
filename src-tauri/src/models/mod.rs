use serde::{Deserialize, Serialize};

mod provider;
pub mod shortcut;
pub use provider::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPrompt {
    pub id: String,
    pub name: String,
    pub prompt: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
    pub role: MessageRole,
    pub content: String,
    #[serde(rename = "screenshotPath", skip_serializing_if = "Option::is_none")]
    pub screenshot_path: Option<String>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenUsage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "event_type", content = "payload")]
pub enum ChatStreamEvent {
    #[serde(rename = "message:chunk")]
    Chunk {
        #[serde(rename = "conversationId")]
        conversation_id: String,
        content: String,
        #[serde(rename = "isFinish")]
        is_finish: bool,
        usage: Option<TokenUsage>,
        timestamp: i64,
    },
    #[serde(rename = "message:error")]
    Error { code: String, message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelInfo {
    pub model_name: String,
    pub context_window: u64,
    pub supports_vision: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

impl std::fmt::Display for MessageRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageRole::User => write!(f, "user"),
            MessageRole::Assistant => write!(f, "assistant"),
            MessageRole::System => write!(f, "system"),
        }
    }
}

impl std::str::FromStr for MessageRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "user" => Ok(MessageRole::User),
            "assistant" => Ok(MessageRole::Assistant),
            "system" => Ok(MessageRole::System),
            _ => Err(format!("Unknown role: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transcript {
    pub id: String,
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
    pub source: AudioSource,
    pub text: String,
    pub confidence: f64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResult {
    pub id: String,
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
    pub source: AudioSource,
    pub text: String,
    #[serde(rename = "isFinal")]
    pub is_final: bool,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
