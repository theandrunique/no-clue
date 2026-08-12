use sqlx::SqlitePool;
use uuid::Uuid;

use crate::domain::transcriptions::Transcript;

pub async fn save(pool: &SqlitePool, transcript: &Transcript) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO transcripts (
            id,
            conversation_id,
            source,
            text,
            confidence,
            created_at
        ) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&transcript.id)
    .bind(&transcript.conversation_id)
    .bind(&transcript.source)
    .bind(&transcript.text)
    .bind(&transcript.confidence)
    .bind(&transcript.created_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_by_conversation(
    pool: &SqlitePool,
    conversation_id: &Uuid,
) -> Result<Vec<Transcript>, sqlx::Error> {
    sqlx::query_as::<_, Transcript>(
        "SELECT
            id,
            conversation_id,
            source,
            text,
            confidence,
            created_at
        FROM transcripts
        WHERE conversation_id = ?
        ORDER BY created_at ASC",
    )
    .bind(conversation_id)
    .fetch_all(pool)
    .await
}
