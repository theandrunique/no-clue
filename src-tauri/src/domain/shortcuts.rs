use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutDef {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub defaults: PlatformDefaults,
    pub rules: ValidationRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformDefaults {
    pub windows: &'static str,
    pub macos: &'static str,
    pub linux: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRules {
    pub requires_modifier: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ShortcutOverride {
    pub id: String,
    pub key_override: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutBinding {
    pub id: String,
    pub name: String,
    pub description: String,
    pub key: String,
    pub enabled: bool,
    pub is_custom: bool,
}
