use sqlx::SqlitePool;
use uuid::Uuid;

use crate::domain::messages::Message;

pub async fn save(pool: &SqlitePool, message: &Message) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO messages (
            id,
            conversation_id,
            role,
            content,
            screenshot_path,
            finish_reason,
            created_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&message.id)
    .bind(&message.conversation_id)
    .bind(&message.role)
    .bind(&message.content)
    .bind(&message.screenshot_path)
    .bind(&message.finish_reason)
    .bind(&message.created_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_by_conversation(
    pool: &SqlitePool,
    conversation_id: &Uuid,
) -> Result<Vec<Message>, sqlx::Error> {
    sqlx::query_as::<_, Message>(
        "SELECT
            id,
            conversation_id,
            role,
            content,
            screenshot_path,
            finish_reason,
            created_at
        FROM messages
        WHERE conversation_id = ?
        ORDER BY created_at ASC",
    )
    .bind(conversation_id)
    .fetch_all(pool)
    .await
}

pub async fn get_by_id(pool: &SqlitePool, id: &Uuid) -> Result<Option<Message>, sqlx::Error> {
    sqlx::query_as::<_, Message>(
        "SELECT
            id,
            conversation_id,
            role,
            content,
            screenshot_path,
            finish_reason,
            created_at
        FROM messages
        WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn delete_after(
    pool: &SqlitePool,
    conversation_id: &Uuid,
    user_message_id: &Uuid,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM messages
         WHERE conversation_id = ?
           AND created_at >= (SELECT created_at FROM messages WHERE id = ?)
           AND id != ?",
    )
    .bind(conversation_id)
    .bind(user_message_id)
    .bind(user_message_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
