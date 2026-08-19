use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::messages::{FinishReason, TokenUsage};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Conversation {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "payload", rename_all = "lowercase")]
pub enum ChatStreamEvent {
    Start {
        message_id: Uuid,
        conversation_id: Uuid,
    },
    Chunk {
        message_id: Uuid,
        conversation_id: Uuid,
        delta: String,
    },
    Finish {
        message_id: Uuid,
        conversation_id: Uuid,
        finish_reason: FinishReason,
        created_at: DateTime<Utc>,
        usage: Option<TokenUsage>,
    },
}
