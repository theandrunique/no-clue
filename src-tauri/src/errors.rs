use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum AppError {
    Internal { message: String },
    Database { message: String },
    SystemPromptNotFound,
    ConversationNotFound,
    LlmProviderNotConfigured,
    SttProviderNotConfigured,
    LlmProviderAlreadyRunning,
    SttProviderAlreadyRunning,
    ShourtcutOverrideNotFound,
    AtLeactOneAudioSourceMustBeEnabled,
    TranscriptionConversationIdNotSet,
}

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        tracing::error!("Internal error: {:#?}", value);
        AppError::Internal {
            message: value.to_string(),
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        tracing::error!("Internal error: {:#?}", value);
        AppError::Database {
            message: value.to_string(),
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Internal { message } => write!(f, "Internal error: {message}"),
            AppError::Database { message } => write!(f, "Database error: {message}"),
            AppError::SystemPromptNotFound => write!(f, "System prompt not found"),
            AppError::ConversationNotFound => write!(f, "Conversation not found"),
            AppError::LlmProviderNotConfigured => write!(f, "LLM provider not configured"),
            AppError::ShourtcutOverrideNotFound => write!(f, "Shourtcut override not found"),
            AppError::SttProviderNotConfigured => write!(f, "STT provider not configured"),
            AppError::SttProviderAlreadyRunning => write!(f, "STT provider already running"),
            AppError::AtLeactOneAudioSourceMustBeEnabled => {
                write!(f, "At least one audio source must be enabled")
            }
            AppError::TranscriptionConversationIdNotSet => {
                write!(f, "Transcription conversation ID not set")
            }
            AppError::LlmProviderAlreadyRunning => write!(f, "LLM provider already running"),
        }
    }
}

impl std::error::Error for AppError {}
