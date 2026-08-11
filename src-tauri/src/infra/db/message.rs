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
            timestamp
        ) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&message.id)
    .bind(&message.conversation_id)
    .bind(&message.role)
    .bind(&message.content)
    .bind(&message.screenshot_path)
    .bind(&message.timestamp)
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
            timestamp
        FROM messages
        WHERE conversation_id = ?1 ORDER BY timestamp ASC",
    )
    .bind(conversation_id)
    .fetch_all(pool)
    .await
}
