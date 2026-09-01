use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::domain::provider_schema::{
    FieldType, LlmCapabilities, LlmDescriptor, LlmProviderDescriptor, ModelRuntimeSettingsSchema,
    ProviderSettingsField, ProviderSettingsSchema,
};

pub static LLM_PROVIDER_DESCRIPTORS: Lazy<Vec<LlmProviderDescriptor>> = Lazy::new(|| {
    vec![
        LlmProviderDescriptor {
            id: "testing-provider".to_string(),
            display_name: "Testing Provider".to_string(),
            settings_schema: ProviderSettingsSchema { fields: vec![] },
            models: vec![LlmDescriptor {
                id: "fake".to_string(),
                display_name: "Fake".to_string(),
                capabilities: LlmCapabilities {
                    context_window: 8196,
                    supports_reasoning: true,
                    supports_vision: true,
                },
                runtime_settings: ModelRuntimeSettingsSchema { fields: vec![] },
            }],
        },
        LlmProviderDescriptor {
            id: "ai-tunnel".to_string(),
            display_name: "AI Tunnel".to_string(),
            settings_schema: ProviderSettingsSchema {
                fields: vec![ProviderSettingsField {
                    key: "api_key".to_string(),
                    display_name: "API Key".to_string(),
                    field_type: FieldType::Password,
                    required: true,
                    placeholder: Some("ai-tunnel-jfs...".to_string()),
                }],
            },
            models: vec![LlmDescriptor {
                id: "qwen3.5-flash-02-23".to_string(),
                display_name: "Qwen3.5 Flash 02-23".to_string(),
                capabilities: LlmCapabilities {
                    context_window: 1_000_000,
                    supports_reasoning: true,
                    supports_vision: true,
                },
                runtime_settings: ModelRuntimeSettingsSchema { fields: vec![] },
            }],
        },
    ]
});

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum LlmProviderSettings {
    TestingProvider,
    AiTunnel { api_key: String },
}
