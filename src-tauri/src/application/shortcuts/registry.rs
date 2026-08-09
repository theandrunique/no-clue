use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutEvent, ShortcutState};

use crate::application::shortcuts::{actions, get_all_shortcut_bindings};

pub fn register_all_shortcuts(app: &AppHandle) -> Result<(), String> {
    let manager = app.global_shortcut();

    if let Err(e) = manager.unregister_all() {
        tracing::warn!("Failed to unregister old shortcuts: {}", e);
    }

    let bindings = get_all_shortcut_bindings();
    let enabled_count = bindings.iter().filter(|b| b.enabled).count();

    for binding in &bindings {
        if !binding.enabled {
            tracing::trace!("Skipping disabled shortcut: {}", binding.id);
            continue;
        }

        let app_handle = app.clone();
        let shortcut_id = binding.id.clone();
        let shortcut_key = binding.key.clone();
        let shortcut_key_for_error = shortcut_key.clone();

        let shortcut: Shortcut = shortcut_key
            .parse()
            .map_err(|e| format!("Invalid shortcut '{}': {}", shortcut_key, e))?;

        let handler = move |_app: &AppHandle, _shortcut: &Shortcut, event: ShortcutEvent| {
            if event.state() == ShortcutState::Pressed {
                tracing::trace!("Shortcut pressed: {} ({})", shortcut_id, shortcut_key);
                actions::on_shortcut_pressed(&app_handle, &shortcut_id);
            } else if event.state() == ShortcutState::Released {
                tracing::trace!("Shortcut released: {}", shortcut_id);
                actions::on_shortcut_released(&shortcut_id);
            }
        };

        manager
            .on_shortcut(shortcut, handler)
            .map_err(|e| format!("Failed to register '{}': {}", shortcut_key_for_error, e))?;
    }

    tracing::info!("Registered {} global shortcuts", enabled_count);
    Ok(())
}
