use sqlx::SqlitePool;
use time::OffsetDateTime;
use uuid::Uuid;
use crate::config::SESSION_DURATION_DAYS;
use crate::models::user::User;

pub async fn create_session(pool: &SqlitePool, user_id: Uuid) -> Result<Uuid, sqlx::Error> {
    let session_id = Uuid::new_v4();
    let expires_at = OffsetDateTime::now_utc() + time::Duration::days(SESSION_DURATION_DAYS);
    let session_id_str = session_id.to_string();
    let user_id_str = user_id.to_string();
    sqlx::query("INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, ?)")
        .bind(session_id_str)
        .bind(user_id_str)
        .bind(expires_at)
        .execute(pool)
        .await?;
    Ok(session_id)
}

pub async fn get_user_by_session(pool: &SqlitePool, session_id: &str) -> Option<User> {
    let now = OffsetDateTime::now_utc();

    // Fetch user only for a valid, non-expired session
    let row = sqlx::query_as::<_, User>(
        "SELECT u.id, u.email, u.password_hash, u.created_at FROM users u \
         JOIN sessions s ON u.id = s.user_id WHERE s.id = ? AND s.expires_at > ?"
    )
    .bind(session_id)
    .bind(now)
    .fetch_optional(pool)
    .await
    .ok()?;

    if row.is_none() {
        // Session not found or expired: purge any expired record for this ID
        let _ = sqlx::query("DELETE FROM sessions WHERE id = ? AND expires_at <= ?")
            .bind(session_id)
            .bind(now)
            .execute(pool)
            .await;
    }

    row
}

pub async fn delete_session(pool: &SqlitePool, session_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM sessions WHERE id = ?")
        .bind(session_id)
        .execute(pool)
        .await?;
    Ok(())
}
