use sqlx::SqlitePool;

use crate::domain::shortcuts::ShortcutOverride;

pub async fn get_all_overrides(pool: &SqlitePool) -> Result<Vec<ShortcutOverride>, sqlx::Error> {
    sqlx::query_as::<_, ShortcutOverride>(
        "SELECT id, key_override, enabled FROM shortcut_overrides",
    )
    .fetch_all(pool)
    .await
}

pub async fn get_override(
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

pub async fn save_override(
    pool: &SqlitePool,
    shortcut_id: &str,
    key_override: Option<String>,
    enabled: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO shortcut_overrides (id, key_override, enabled)
        VALUES (?, ?, ?)
        ON CONFLICT(id) DO UPDATE
            SET key_override = excluded.key_override,
                enabled = excluded.enabled",
    )
    .bind(shortcut_id)
    .bind(key_override)
    .bind(enabled)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_override(pool: &SqlitePool, shortcut_id: &str) -> Result<bool, sqlx::Error> {
    let res = sqlx::query("DELETE FROM shortcut_overrides WHERE id = ?")
        .bind(shortcut_id)
        .execute(pool)
        .await?;

    Ok(res.rows_affected() > 0)
}
