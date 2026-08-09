use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub created_at: i64,
    pub updated_at: i64,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenUsage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}
