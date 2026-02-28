use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;
use tower_cookies::Cookies;
use crate::auth::session::get_user_by_session;
use crate::AppState;

pub async fn require_auth(
    State(state): State<AppState>,
    cookies: Cookies,
    mut req: Request,
    next: Next,
) -> Response {
    let session_id = match cookies.get("session_id").map(|c| c.value().to_string()) {
        Some(id) => id,
        None => {
            return (StatusCode::UNAUTHORIZED, Json(json!({ "error": "unauthorized" }))).into_response();
        }
    };

    match get_user_by_session(&state.pool, &session_id).await {
        Some(user) => {
            req.extensions_mut().insert(user);
            next.run(req).await
        }
        None => (StatusCode::UNAUTHORIZED, Json(json!({ "error": "unauthorized" }))).into_response(),
    }
}
