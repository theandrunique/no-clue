use once_cell::sync::Lazy;
use serde::{Serialize};

#[derive(Serialize)]
pub struct ProviderSettingsSchema {
    pub fields: Vec<SettingsField>,
}

#[derive(Serialize)]
pub struct SettingsField {
    pub key: String,
    pub display_name: String,
    pub field_type: FieldType,
    pub required: bool,
    pub placeholder: Option<String>,
}

#[derive(Serialize)]
pub enum FieldType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "select")]
    Select { options: Vec<&'static str> },
}

#[derive(Serialize)]
pub struct ModelRuntimeSettingsSchema {
    pub fields: Vec<RuntimeSettingsField>,
}

#[derive(Serialize)]
pub struct RuntimeSettingsField {
    pub key: String,
    pub display_name: String,
    pub field_type: RuntimeFieldType,
    pub default_value: serde_json::Value,
}

#[derive(Serialize)]
pub enum RuntimeFieldType {
    Boolean,
    Number,
    Select { options: Vec<&'static str> }
}

#[derive(Serialize)]
pub struct ModelDescriptor {
    pub id: String,
    pub display_name: String,
    pub runtime_config: ModelRuntimeSettingsSchema,
}

#[derive(Serialize)]
pub struct ProviderDescriptor {
    pub id: String,
    pub display_name: String,
    pub config_schema: ProviderSettingsSchema,
    pub models: Vec<ModelDescriptor>,
}

pub static STT_PROVIDERS: Lazy<Vec<ProviderDescriptor>> =
    Lazy::new(|| {
        vec![
            ProviderDescriptor {
                id: "testing-provider".to_string(),
                display_name: "Testing Provider".to_string(),
                config_schema: ProviderSettingsSchema {
                    fields: vec![]
                },
                models: vec![
                    ModelDescriptor {
                        id: "fake".to_string(),
                        display_name: "Fake".to_string(),
                        runtime_config: ModelRuntimeSettingsSchema {
                            fields: vec![]
                        }
                    }
                ]
            },
            ProviderDescriptor {
                id: "deepgram".to_string(),
                display_name: "Deepgram".to_string(),
                config_schema: ProviderSettingsSchema {
                    fields: vec![
                        SettingsField {
                            key: "api-key".to_string(),
                            display_name: "API Key".to_string(),
                            field_type: FieldType::Password,
                            required: true,
                            placeholder: None,
                        }
                    ]
                },
                models: vec![
                    ModelDescriptor {
                        id: "nova-3".to_string(),
                        display_name: "Nova 3".to_string(),
                        runtime_config: ModelRuntimeSettingsSchema {
                            fields: vec![
                                RuntimeSettingsField {
                                    key: "language".to_string(),
                                    display_name: "Language".to_string(),
                                    field_type: RuntimeFieldType::Select { options: vec!["ru", "en"] },
                                    default_value: serde_json::json!("ru"),
                                },
                            ]
                        }
                    }
                ]
            }
        ]
    });
