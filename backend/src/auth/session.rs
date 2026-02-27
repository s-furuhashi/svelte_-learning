use sqlx::MySqlPool;
use time::OffsetDateTime;
use uuid::Uuid;
use crate::models::user::User;

pub async fn create_session(pool: &MySqlPool, user_id: Uuid) -> Result<Uuid, sqlx::Error> {
    let session_id = Uuid::new_v4();
    let expires_at = OffsetDateTime::now_utc() + time::Duration::days(7);
    let session_id_bytes = session_id.as_bytes().to_vec();
    let user_id_bytes = user_id.as_bytes().to_vec();
    sqlx::query("INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, ?)")
        .bind(session_id_bytes)
        .bind(user_id_bytes)
        .bind(expires_at)
        .execute(pool)
        .await?;
    Ok(session_id)
}

pub async fn get_user_by_session(pool: &MySqlPool, session_id: &str) -> Option<User> {
    let session_uuid = Uuid::parse_str(session_id).ok()?;
    let session_id_bytes = session_uuid.as_bytes().to_vec();
    let now = OffsetDateTime::now_utc();
    let row = sqlx::query_as::<_, User>(
        "SELECT u.id, u.email, u.password_hash, u.created_at FROM users u JOIN sessions s ON u.id = s.user_id WHERE s.id = ? AND s.expires_at > ?"
    )
    .bind(session_id_bytes)
    .bind(now)
    .fetch_optional(pool)
    .await
    .ok()??;
    Some(row)
}

pub async fn delete_session(pool: &MySqlPool, session_id: &str) -> Result<(), sqlx::Error> {
    if let Ok(session_uuid) = Uuid::parse_str(session_id) {
        let session_id_bytes = session_uuid.as_bytes().to_vec();
        sqlx::query("DELETE FROM sessions WHERE id = ?")
            .bind(session_id_bytes)
            .execute(pool)
            .await?;
    }
    Ok(())
}
