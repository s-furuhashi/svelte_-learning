use axum::{
    extract::{Path, State},
    Json,
    http::StatusCode,
};
use serde_json::{json, Value};
use crate::AppState;
use crate::models::article::{Article, ArticleResponse};

pub async fn list_articles(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    let articles = sqlx::query_as::<_, Article>(
        "SELECT id, title, slug, markdown, html, published, created_at, updated_at FROM articles WHERE published = true ORDER BY created_at DESC"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let responses: Vec<ArticleResponse> = articles.into_iter().map(|a| a.into()).collect();
    Ok(Json(json!({ "articles": responses })))
}

pub async fn get_article(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let article = sqlx::query_as::<_, Article>(
        "SELECT id, title, slug, markdown, html, published, created_at, updated_at FROM articles WHERE slug = ? AND published = true"
    )
    .bind(&slug)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let response: ArticleResponse = article.into();
    Ok(Json(json!({ "article": response })))
}
