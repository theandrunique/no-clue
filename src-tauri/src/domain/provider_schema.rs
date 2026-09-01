use serde::Serialize;

#[derive(Serialize)]
pub struct SttProviderDescriptor {
    pub id: String,
    pub display_name: String,
    pub settings_schema: ProviderSettingsSchema,
    pub models: Vec<SttDescriptor>,
}

#[derive(Clone, Serialize)]
pub struct SttDescriptor {
    pub id: String,
    pub display_name: String,
    pub runtime_settings: ModelRuntimeSettingsSchema,
}

#[derive(Serialize)]
pub struct LlmProviderDescriptor {
    pub id: String,
    pub display_name: String,
    pub settings_schema: ProviderSettingsSchema,
    pub models: Vec<LlmDescriptor>,
}

#[derive(Clone, Serialize)]
pub struct LlmDescriptor {
    pub id: String,
    pub display_name: String,
    pub capabilities: LlmCapabilities,
    pub runtime_settings: ModelRuntimeSettingsSchema,
}

#[derive(Clone, Serialize)]
pub struct LlmCapabilities {
    pub context_window: u64,
    pub supports_vision: bool,
    pub supports_reasoning: bool,
}

#[derive(Clone, Serialize)]
pub struct ProviderSettingsSchema {
    pub fields: Vec<ProviderSettingsField>,
}

#[derive(Clone, Serialize)]
pub struct ProviderSettingsField {
    pub key: String,
    pub display_name: String,
    pub field_type: FieldType,
    pub required: bool,
    pub placeholder: Option<String>,
}

#[derive(Clone, Serialize)]
pub enum FieldType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "select")]
    Select { options: Vec<&'static str> },
}

#[derive(Clone, Serialize)]
pub struct ModelRuntimeSettingsSchema {
    pub fields: Vec<RuntimeSettingsField>,
}

#[derive(Clone, Serialize)]
pub struct RuntimeSettingsField {
    pub key: String,
    pub display_name: String,
    pub field_type: RuntimeFieldType,
    pub default_value: serde_json::Value,
}

#[derive(Clone, Serialize)]
pub enum RuntimeFieldType {
    Boolean,
    Number,
    Select { options: Vec<&'static str> },
}
