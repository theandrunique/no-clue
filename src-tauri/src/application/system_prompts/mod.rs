use sqlx::SqlitePool;
use tauri::{AppHandle, Manager};

use crate::db::system_prompt as repo;
use crate::domain::system_prompts::SystemPrompt;
use crate::errors::AppError;

#[tauri::command]
pub async fn get_system_prompts(app: AppHandle) -> Result<Vec<SystemPrompt>, AppError> {
    tracing::trace!("get_system_prompts called");
    let pool = app.state::<SqlitePool>();
    Ok(repo::get_all(&pool).await?)
}

#[tauri::command]
pub async fn get_system_prompt(app: AppHandle, id: &str) -> Result<Option<SystemPrompt>, AppError> {
    tracing::trace!(prompt_id = %id, "get_system_prompt called");
    let pool = app.state::<SqlitePool>();
    Ok(repo::get_by_id(&pool, id).await?)
}

#[tauri::command]
pub async fn create_system_prompt(
    app: AppHandle,
    name: &str,
    prompt: &str,
) -> Result<SystemPrompt, AppError> {
    tracing::trace!(name = %name, "create_system_prompt called");

    let now = chrono::Utc::now();
    let new_prompt = SystemPrompt {
        id: uuid::Uuid::new_v4(),
        name: name.to_string(),
        prompt: prompt.to_string(),
        created_at: now,
        updated_at: now,
    };

    let pool = app.state::<SqlitePool>();
    repo::upsert(&pool, &new_prompt).await?;

    Ok(new_prompt)
}

#[tauri::command]
pub async fn update_system_prompt(
    app: AppHandle,
    id: String,
    name: String,
    prompt: String,
) -> Result<(), AppError> {
    tracing::trace!(prompt_id = %id, "update_system_prompt called");
    let pool = app.state::<SqlitePool>();

    let mut system_prompt = repo::get_by_id(&pool, &id)
        .await?
        .ok_or(AppError::SystemMessageNotFound)?;

    system_prompt.name = name;
    system_prompt.prompt = prompt;

    Ok(repo::upsert(&pool, &system_prompt).await?)
}

#[tauri::command]
pub async fn delete_system_prompt(app: AppHandle, id: &str) -> Result<(), AppError> {
    tracing::trace!(prompt_id = %id, "delete_system_prompt called");
    let pool = app.state::<SqlitePool>();
    let deleted = repo::delete(&pool, &id).await?;
    if !deleted {
        return Err(AppError::SystemMessageNotFound);
    }
    Ok(())
}
