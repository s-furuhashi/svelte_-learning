use axum::{
    extract::{Path, State},
    Json,
    http::StatusCode,
};
use serde_json::{json, Value};
use crate::AppState;
use crate::models::article::{Article, PublicArticleListResponse, PublicArticleDetailResponse};

fn error_response(status: StatusCode, message: &str) -> (StatusCode, Json<Value>) {
    (status, Json(json!({ "error": message })))
}

pub async fn list_articles(State(state): State<AppState>) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let articles = sqlx::query_as::<_, Article>(
        "SELECT id, title, slug, markdown, html, published, created_at, updated_at FROM articles WHERE published = true ORDER BY created_at DESC"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        error_response(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })?;

    let responses: Vec<PublicArticleListResponse> = articles.into_iter().map(|a| a.into()).collect();
    Ok(Json(json!(responses)))
}

pub async fn get_article(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let article = sqlx::query_as::<_, Article>(
        "SELECT id, title, slug, markdown, html, published, created_at, updated_at FROM articles WHERE slug = ? AND published = true"
    )
    .bind(&slug)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        error_response(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })?
    .ok_or_else(|| error_response(StatusCode::NOT_FOUND, "Article not found"))?;

    let response: PublicArticleDetailResponse = article.into();
    Ok(Json(json!(response)))
}

