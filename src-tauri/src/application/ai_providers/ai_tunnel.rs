use crate::domain::providers::{FieldDescriptor, FieldType, ProviderDescriptor};

pub fn ai_tunnel_descriptor() -> ProviderDescriptor {
    ProviderDescriptor {
        id: "aitunnel".into(),
        label: "AI Tunnel".into(),
        fields: vec![
            FieldDescriptor {
                key: "model".into(),
                label: "Model".into(),
                field_type: FieldType::Text,
                required: true,
                placeholder: Some("llama3".into()),
            },
            FieldDescriptor {
                key: "api_key".into(),
                label: "API Key".into(),
                field_type: FieldType::Password,
                required: true,
                placeholder: None,
            },
        ],
    }
}
