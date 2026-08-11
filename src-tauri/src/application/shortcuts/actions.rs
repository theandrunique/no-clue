use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

use crate::application::shortcuts::definitions::get_shortcut_definitions;
use crate::db::shortcut_overrides as db_shortcut;
use crate::utils::{move_overlay, toggle_overlay, Direction};

struct RepeatState {
    running: AtomicBool,
}

static REPEAT_STATES: Mutex<Option<HashMap<String, Arc<RepeatState>>>> = Mutex::new(None);

fn get_repeat_states() -> std::sync::MutexGuard<'static, Option<HashMap<String, Arc<RepeatState>>>>
{
    let mut guard = REPEAT_STATES.lock().unwrap();
    if guard.is_none() {
        *guard = Some(HashMap::new());
    }
    guard
}

pub async fn execute_action(app: &AppHandle, shortcut_id: &str) {
    let pool = app.state::<SqlitePool>();
    if let Ok(Some(override_)) = db_shortcut::get_override(&pool, shortcut_id).await {
        if !override_.enabled {
            tracing::trace!("Shortcut {} is disabled", shortcut_id);
            return;
        }
    }

    let def = get_shortcut_definitions()
        .into_iter()
        .find(|d| d.id == shortcut_id);

    let Some(_def) = def else {
        tracing::warn!("Unknown shortcut triggered: {}", shortcut_id);
        return;
    };

    let _ = app.emit("shortcut-triggered", shortcut_id);

    run_backend_action(app, shortcut_id);
}

pub async fn on_shortcut_pressed(app: &AppHandle, shortcut_id: &str) {
    execute_action(app, shortcut_id).await;
    start_repeat(app.clone(), shortcut_id.to_string());
}

pub fn on_shortcut_released(shortcut_id: &str) {
    stop_repeat(shortcut_id);
}

fn start_repeat(app: AppHandle, shortcut_id: String) {
    let state = Arc::new(RepeatState {
        running: AtomicBool::new(true),
    });

    {
        let mut states = get_repeat_states();
        states
            .as_mut()
            .unwrap()
            .insert(shortcut_id.clone(), state.clone());
    }

    let app_clone = app.clone();
    let shortcut_id_clone = shortcut_id.clone();

    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(200));

        if !state.running.load(Ordering::SeqCst) {
            return;
        }

        while state.running.load(Ordering::SeqCst) {
            if state.running.load(Ordering::SeqCst) {
                run_backend_action(&app_clone, &shortcut_id_clone);
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    });
}

fn stop_repeat(shortcut_id: &str) {
    let mut states = get_repeat_states();
    if let Some(map) = states.as_mut() {
        if let Some(state) = map.remove(shortcut_id) {
            state.running.store(false, Ordering::SeqCst);
        }
    }
}

fn run_backend_action(app: &AppHandle, shortcut_id: &str) {
    match shortcut_id {
        "move_window_up" => {
            if let Err(e) = move_overlay(app.clone(), Direction::Up, 10) {
                tracing::warn!("Failed to move window up: {}", e);
            }
        }
        "move_window_down" => {
            if let Err(e) = move_overlay(app.clone(), Direction::Down, 10) {
                tracing::warn!("Failed to move window down: {}", e);
            }
        }
        "move_window_left" => {
            if let Err(e) = move_overlay(app.clone(), Direction::Left, 10) {
                tracing::warn!("Failed to move window left: {}", e);
            }
        }
        "move_window_right" => {
            if let Err(e) = move_overlay(app.clone(), Direction::Right, 10) {
                tracing::warn!("Failed to move window right: {}", e);
            }
        }
        "toggle_overlay_visibility" => {
            if let Err(e) = toggle_overlay(app) {
                tracing::warn!("Failed to toggle overlay: {}", e);
            }
        }
        _ => {}
    }
}
