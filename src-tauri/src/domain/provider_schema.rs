use serde::Serialize;

#[derive(Serialize)]
pub struct ProviderDescriptor {
    pub id: String,
    pub label: String,
    pub fields: Vec<FieldDescriptor>,
}

#[derive(Serialize)]
pub struct FieldDescriptor {
    pub key: String,
    pub label: String,
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
    Select { options: Vec<String> },
}
