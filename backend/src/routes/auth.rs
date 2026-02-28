use axum::{
    extract::State,
    Json,
    http::StatusCode,
};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use time::Duration;
use crate::AppState;
use crate::auth::session::{create_session, delete_session, get_user_by_session};
use crate::config::SESSION_DURATION_DAYS;
use crate::models::user::User;

type AuthError = (StatusCode, Json<Value>);

fn err(code: StatusCode, msg: &str) -> AuthError {
    (code, Json(json!({ "error": msg })))
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<Value>, AuthError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, created_at FROM users WHERE email = ?"
    )
    .bind(&payload.email)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| err(StatusCode::INTERNAL_SERVER_ERROR, "internal server error"))?
    .ok_or_else(|| err(StatusCode::UNAUTHORIZED, "invalid credentials"))?;

    let valid = bcrypt::verify(&payload.password, &user.password_hash)
        .map_err(|_| err(StatusCode::INTERNAL_SERVER_ERROR, "internal server error"))?;

    if !valid {
        return Err(err(StatusCode::UNAUTHORIZED, "invalid credentials"));
    }

    let user_uuid = user.id_as_uuid()
        .ok_or_else(|| err(StatusCode::INTERNAL_SERVER_ERROR, "internal server error"))?;
    let session_id = create_session(&state.pool, user_uuid)
        .await
        .map_err(|_| err(StatusCode::INTERNAL_SERVER_ERROR, "internal server error"))?;

    let mut cookie = Cookie::new("session_id", session_id.to_string());
    cookie.set_http_only(true);
    cookie.set_same_site(tower_cookies::cookie::SameSite::Lax);
    cookie.set_path("/");
    cookie.set_max_age(Duration::days(SESSION_DURATION_DAYS));
    if state.config.is_production {
        cookie.set_secure(true);
    }
    cookies.add(cookie);

    Ok(Json(json!({ "success": true })))
}

pub async fn me(
    State(state): State<AppState>,
    cookies: Cookies,
) -> Result<Json<Value>, AuthError> {
    let session_id = cookies
        .get("session_id")
        .map(|c| c.value().to_string())
        .ok_or_else(|| err(StatusCode::UNAUTHORIZED, "unauthorized"))?;
    let user = get_user_by_session(&state.pool, &session_id)
        .await
        .ok_or_else(|| err(StatusCode::UNAUTHORIZED, "unauthorized"))?;
    let user_id = user.id_as_uuid()
        .map(|u| u.to_string())
        .ok_or_else(|| err(StatusCode::INTERNAL_SERVER_ERROR, "internal server error"))?;
    Ok(Json(json!({ "user_id": user_id, "email": user.email })))
}

pub async fn logout(
    State(state): State<AppState>,
    cookies: Cookies,
) -> Json<Value> {
    if let Some(session_cookie) = cookies.get("session_id") {
        let session_id = session_cookie.value().to_string();
        let _ = delete_session(&state.pool, &session_id).await;
    }

    let mut removal = Cookie::new("session_id", "");
    removal.set_path("/");
    removal.set_max_age(Duration::seconds(0));
    cookies.add(removal);

    Json(json!({ "success": true }))
}
