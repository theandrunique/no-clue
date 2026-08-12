use sqlx::SqlitePool;

use crate::domain::shortcuts::ShortcutOverride;

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<ShortcutOverride>, sqlx::Error> {
    sqlx::query_as::<_, ShortcutOverride>(
        "SELECT id, key_override, enabled FROM shortcut_overrides",
    )
    .fetch_all(pool)
    .await
}

pub async fn get_by_id(
    pool: &SqlitePool,
    shortcut_id: &str,
) -> Result<Option<ShortcutOverride>, sqlx::Error> {
    sqlx::query_as::<_, ShortcutOverride>(
        "SELECT id, key_override, enabled FROM shortcut_overrides WHERE id = ?",
    )
    .bind(shortcut_id)
    .fetch_optional(pool)
    .await
}

pub async fn save(
    pool: &SqlitePool,
    shortcut_override: &ShortcutOverride,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO shortcut_overrides (id, key_override, enabled)
        VALUES (?, ?, ?)
        ON CONFLICT(id) DO UPDATE
            SET key_override = excluded.key_override,
                enabled = excluded.enabled",
    )
    .bind(&shortcut_override.id)
    .bind(&shortcut_override.key_override)
    .bind(&shortcut_override.enabled)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete(pool: &SqlitePool, id: &str) -> Result<bool, sqlx::Error> {
    let res = sqlx::query("DELETE FROM shortcut_overrides WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(res.rows_affected() > 0)
}
