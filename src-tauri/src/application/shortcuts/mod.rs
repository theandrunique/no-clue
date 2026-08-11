use crate::{
    application::shortcuts::definitions::{get_default_for_platform, get_shortcut_definitions},
    db::shortcut_overrides as db_shortcut,
    domain::shortcuts::ShortcutBinding,
};

pub mod actions;
pub mod commands;
mod definitions;
pub mod registry;

pub use commands::*;
pub use registry::register_all_shortcuts;
use sqlx::SqlitePool;

pub async fn get_all_shortcut_bindings(pool: &SqlitePool) -> Vec<ShortcutBinding> {
    let mut bindings = Vec::new();

    for def in get_shortcut_definitions() {
        let override_ = db_shortcut::get_by_id(pool, def.id).await.ok().flatten();
        let (key, enabled, is_custom) = match override_ {
            Some(o) => (
                o.key_override
                    .clone()
                    .unwrap_or_else(|| get_default_for_platform(&def).to_string()),
                o.enabled,
                o.key_override.is_some(),
            ),
            None => (get_default_for_platform(&def).to_string(), true, false),
        };
        bindings.push(ShortcutBinding {
            id: def.id.to_string(),
            name: def.name.to_string(),
            description: def.description.to_string(),
            key,
            enabled,
            is_custom,
        });
    }

    bindings
}
