use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::domain::provider_schema::{
    FieldType, LlmCapabilities, LlmDescriptor, LlmProviderDescriptor, ModelRuntimeSettingsSchema, ProviderSettingsField, ProviderSettingsSchema, RuntimeFieldType, RuntimeSettingsField,
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
                runtime_settings: ModelRuntimeSettingsSchema {
                    fields: vec![RuntimeSettingsField {
                        key: "duration".to_string(),
                        display_name: "Duration".to_string(),
                        field_type: RuntimeFieldType::Select {
                            options: vec!["6s", "12s", "24s", "30s"],
                        },
                        default_value: serde_json::json!("12s"),
                    }],
                },
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
                runtime_settings: ModelRuntimeSettingsSchema {
                    fields: vec![RuntimeSettingsField {
                        key: "temperature".to_string(),
                        display_name: "Temperature".to_string(),
                        field_type: RuntimeFieldType::Select {
                            options: vec!["0.5", "0.7", "0.9"],
                        },
                        default_value: serde_json::json!("0.7"),
                    }],
                },
            }],
        },
    ]
});

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LlmProviderSettings {
    TestingProvider,
    AiTunnel { api_key: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LlmSettings {
    Fake { duration: String },
    Qwen3_5Flash { temperature: f32 },
}

impl LlmSettings {
    pub fn model_id(&self) -> &'static str {
        match self {
            LlmSettings::Fake { .. } => "fake",
            LlmSettings::Qwen3_5Flash { .. } => "qwen3.5-flash-02-23",
        }
    }
}

pub fn find_llm_model(
    model_id: &str,
) -> Option<(&'static LlmProviderDescriptor, &'static LlmDescriptor)> {
    LLM_PROVIDER_DESCRIPTORS
        .iter()
        .find_map(|provider| {
            provider
                .models
                .iter()
                .find(|model| model.id == model_id)
                .map(|model| (provider, model))
        })
}
