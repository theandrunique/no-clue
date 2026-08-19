use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Conversation {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "payload", rename_all="lowercase")]
pub enum ChatStreamEvent {
    Chunk {
        conversation_id: Uuid,
        content: String,
        is_finish: bool,
        usage: Option<TokenUsage>,
        timestamp: DateTime<Utc>,
    },
    Error {
        conversation_id: Uuid,
        code: String,
        message: String,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenUsage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}
