use tauri::AppHandle;

use crate::application::shortcuts::get_all_shortcut_bindings;
use crate::application::shortcuts::registry::register_all_shortcuts;
use crate::db::shortcut_overrides as db_shortcut;
use crate::domain::shortcuts::ShortcutBinding;

#[tauri::command]
pub fn get_shortcuts() -> Vec<ShortcutBinding> {
    get_all_shortcut_bindings()
}

#[tauri::command]
pub fn save_shortcut(
    app: AppHandle,
    shortcut_id: String,
    key_override: Option<String>,
    enabled: bool,
) -> Result<(), String> {
    db_shortcut::save_override(&shortcut_id, key_override, enabled).map_err(|e| e.to_string())?;
    register_all_shortcuts(&app)?;
    Ok(())
}

#[tauri::command]
pub fn delete_shortcut_override(shortcut_id: String) -> Result<(), String> {
    db_shortcut::delete_override(&shortcut_id).map_err(|e| e.to_string())
}
