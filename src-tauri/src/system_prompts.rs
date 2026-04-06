use crate::db::system_prompt as repo;
use crate::error::log_err;
use crate::models::SystemPrompt;

#[tauri::command]
pub async fn get_system_prompts() -> Result<Vec<SystemPrompt>, String> {
    tracing::trace!("get_system_prompts called");

    let prompts = tokio::task::spawn_blocking(|| repo::get_all())
        .await
        .map_err(|e| log_err(e, "get_system_prompts"))?
        .map_err(|e| log_err(e, "get_system_prompts"))?;

    Ok(prompts)
}

#[tauri::command]
pub async fn get_system_prompt(id: String) -> Result<Option<SystemPrompt>, String> {
    tracing::trace!(prompt_id = %id, "get_system_prompt called");

    let id_clone = id.clone();
    let prompt = tokio::task::spawn_blocking(move || repo::get_by_id(&id_clone))
        .await
        .map_err(|e| log_err(e, "get_system_prompt"))?
        .map_err(|e| log_err(e, "get_system_prompt"))?;

    Ok(prompt)
}

#[tauri::command]
pub async fn create_system_prompt(name: String, prompt: String) -> Result<SystemPrompt, String> {
    tracing::trace!(name = %name, "create_system_prompt called");

    let now = chrono::Utc::now().timestamp();
    let new_prompt = SystemPrompt {
        id: uuid::Uuid::new_v4().to_string(),
        name: name.clone(),
        prompt: prompt.clone(),
        created_at: now,
        updated_at: now,
    };

    let prompt_to_save = new_prompt.clone();

    tokio::task::spawn_blocking(move || repo::create(&prompt_to_save))
        .await
        .map_err(|e| log_err(e, "create_system_prompt"))?
        .map_err(|e| log_err(e, "create_system_prompt"))?;

    Ok(new_prompt)
}

#[tauri::command]
pub async fn update_system_prompt(id: String, name: String, prompt: String) -> Result<(), String> {
    tracing::trace!(prompt_id = %id, "update_system_prompt called");

    tokio::task::spawn_blocking(move || repo::update(&id, &name, &prompt))
        .await
        .map_err(|e| log_err(e, "update_system_prompt"))?
        .map_err(|e| log_err(e, "update_system_prompt"))?;

    Ok(())
}

#[tauri::command]
pub async fn delete_system_prompt(id: String) -> Result<(), String> {
    tracing::trace!(prompt_id = %id, "delete_system_prompt called");

    tokio::task::spawn_blocking(move || repo::delete(&id))
        .await
        .map_err(|e| log_err(e, "delete_system_prompt"))?
        .map_err(|e| log_err(e, "delete_system_prompt"))?;

    Ok(())
}
