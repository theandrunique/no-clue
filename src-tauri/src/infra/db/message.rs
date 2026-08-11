use sqlx::SqlitePool;

use crate::domain::messages::Message;

pub async fn create(pool: &SqlitePool, message: &Message) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO messages (
            id,
            conversation_id,
            role,
            content,
            screenshot_path,
            created_at
        ) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&message.id)
    .bind(&message.conversation_id)
    .bind(&message.role)
    .bind(&message.content)
    .bind(&message.screenshot_path)
    .bind(&message.created_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_by_conversation(
    pool: &SqlitePool,
    conversation_id: &str,
) -> Result<Vec<Message>, sqlx::Error> {
    sqlx::query_as::<_, Message>(
        "SELECT
            id,
            conversation_id,
            role,
            content,
            screenshot_path,
            created_at
        FROM messages
        WHERE conversation_id = ? ORDER BY created_at ASC",
    )
    .bind(conversation_id)
    .fetch_all(pool)
    .await
}
