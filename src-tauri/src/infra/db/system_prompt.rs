use sqlx::SqlitePool;
use uuid::Uuid;

use crate::domain::system_prompts::SystemPrompt;

pub async fn upsert(pool: &SqlitePool, prompt: &SystemPrompt) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO system_prompts (
            id,
            name,
            prompt,
            created_at,
            updated_at
        ) VALUES (?, ?, ?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET
            name = excluded.name,
            prompt = excluded.prompt,
            created_at = excluded.created_at,
            updated_at = excluded.updated_at",
    )
    .bind(&prompt.id)
    .bind(&prompt.name)
    .bind(&prompt.prompt)
    .bind(&prompt.created_at)
    .bind(&prompt.updated_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<SystemPrompt>, sqlx::Error> {
    sqlx::query_as::<_, SystemPrompt>(
        "SELECT
            id,
            name,
            prompt,
            created_at,
            updated_at
        FROM system_prompts
        ORDER BY updated_at DESC",
    )
    .fetch_all(pool)
    .await
}

pub async fn get_by_id(pool: &SqlitePool, id: &Uuid) -> Result<Option<SystemPrompt>, sqlx::Error> {
    sqlx::query_as::<_, SystemPrompt>(
        "SELECT id, name, prompt, created_at, updated_at
        FROM system_prompts
        WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn delete(pool: &SqlitePool, id: &Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM system_prompts WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
