use sqlx::SqlitePool;
use uuid::Uuid;

use crate::domain::conversations::Conversation;

pub async fn save(pool: &SqlitePool, conversation: &Conversation) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO conversations (id, title, created_at, updated_at) VALUES (?, ?, ?, ?)",
    )
    .bind(&conversation.id)
    .bind(&conversation.title)
    .bind(&conversation.created_at)
    .bind(&conversation.updated_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Conversation>, sqlx::Error> {
    sqlx::query_as::<_, Conversation>(
        "SELECT id, title, created_at, updated_at
        FROM conversations
        ORDER BY updated_at DESC",
    )
    .fetch_all(pool)
    .await
}

pub async fn get_by_id(pool: &SqlitePool, id: &Uuid) -> Result<Option<Conversation>, sqlx::Error> {
    sqlx::query_as::<_, Conversation>(
        "SELECT id, title, created_at, updated_at FROM conversations WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn delete(pool: &SqlitePool, id: &Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM conversations WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
