use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::domain::provider_schema::{
    FieldType, ModelRuntimeSettingsSchema, ProviderSettingsField, ProviderSettingsSchema,
    RuntimeFieldType, RuntimeSettingsField, SttDescriptor, SttProviderDescriptor,
};

pub static STT_PROVIDERS: Lazy<Vec<SttProviderDescriptor>> = Lazy::new(|| {
    vec![
        SttProviderDescriptor {
            id: "testing-provider".to_string(),
            display_name: "Testing Provider".to_string(),
            settings_schema: ProviderSettingsSchema { fields: vec![] },
            models: vec![SttDescriptor {
                id: "fake".to_string(),
                display_name: "Fake".to_string(),
                runtime_settings: ModelRuntimeSettingsSchema { fields: vec![] },
            }],
        },
        SttProviderDescriptor {
            id: "deepgram".to_string(),
            display_name: "Deepgram".to_string(),
            settings_schema: ProviderSettingsSchema {
                fields: vec![ProviderSettingsField {
                    key: "api_key".to_string(),
                    display_name: "API Key".to_string(),
                    field_type: FieldType::Password,
                    required: true,
                    placeholder: Some("js33eg...".to_string()),
                }],
            },
            models: vec![
                SttDescriptor {
                    id: "nova-3".to_string(),
                    display_name: "Nova 3".to_string(),
                    runtime_settings: ModelRuntimeSettingsSchema {
                        fields: vec![RuntimeSettingsField {
                            key: "language".to_string(),
                            display_name: "Language".to_string(),
                            field_type: RuntimeFieldType::Select {
                                options: vec!["ru", "en"],
                            },
                            default_value: serde_json::json!("ru"),
                        }],
                    },
                },
                SttDescriptor {
                    id: "nova-2".to_string(),
                    display_name: "Nova 2".to_string(),
                    runtime_settings: ModelRuntimeSettingsSchema {
                        fields: vec![RuntimeSettingsField {
                            key: "language".to_string(),
                            display_name: "Language".to_string(),
                            field_type: RuntimeFieldType::Select {
                                options: vec!["ru", "en"],
                            },
                            default_value: serde_json::json!("ru"),
                        }],
                    },
                },
            ],
        },
    ]
});

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum SttProviderSettings {
    TestingProvider,
    Deepgram { api_key: String },
}
