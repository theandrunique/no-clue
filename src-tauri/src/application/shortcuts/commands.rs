use sqlx::SqlitePool;
use tauri::{AppHandle, Manager};

use crate::application::shortcuts::get_all_shortcut_bindings;
use crate::application::shortcuts::registry::register_all_shortcuts;
use crate::db::shortcut_overrides as db_shortcut;
use crate::domain::shortcuts::ShortcutBinding;
use crate::errors::AppError;

#[tauri::command]
pub async fn get_shortcuts(app: AppHandle) -> Vec<ShortcutBinding> {
    let pool = app.state::<SqlitePool>();
    get_all_shortcut_bindings(&pool).await
}

#[tauri::command]
pub async fn save_shortcut(
    app: AppHandle,
    shortcut_id: &str,
    key_override: Option<String>,
    enabled: bool,
) -> Result<(), AppError> {
    let pool = app.state::<SqlitePool>();
    db_shortcut::save_override(&pool, &shortcut_id, key_override, enabled).await?;
    register_all_shortcuts(&app).await?;
    Ok(())
}

#[tauri::command]
pub async fn delete_shortcut_override(app: AppHandle, shortcut_id: String) -> Result<(), AppError> {
    let pool = app.state::<SqlitePool>();
    let deleted = db_shortcut::delete_override(&pool, &shortcut_id).await?;
    if !deleted {
        return Err(AppError::ShourtcutOverrideNotFound);
    }
    Ok(())
}
