use axum::{
    extract::{Path, State},
    Json,
    http::StatusCode,
};
use serde_json::{json, Value};
use uuid::Uuid;
use crate::AppState;
use crate::models::article::{Article, CreateArticleRequest, UpdateArticleRequest};
use crate::utils::markdown::markdown_to_html;

pub async fn create_article(
    State(state): State<AppState>,
    Json(payload): Json<CreateArticleRequest>,
) -> Result<Json<Value>, StatusCode> {
    let id = Uuid::new_v4();
    let id_bytes = id.as_bytes().to_vec();
    let html = markdown_to_html(&payload.markdown);
    let published = payload.published.unwrap_or(false);

    sqlx::query("INSERT INTO articles (id, title, slug, markdown, html, published) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(&id_bytes)
        .bind(&payload.title)
        .bind(&payload.slug)
        .bind(&payload.markdown)
        .bind(&html)
        .bind(published)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("DB error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(json!({ "id": id.to_string(), "message": "created" })))
}

pub async fn update_article(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateArticleRequest>,
) -> Result<Json<Value>, StatusCode> {
    let uuid = Uuid::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let id_bytes = uuid.as_bytes().to_vec();

    let article = sqlx::query_as::<_, Article>(
        "SELECT id, title, slug, markdown, html, published, created_at, updated_at FROM articles WHERE id = ?"
    )
    .bind(&id_bytes)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let new_title = payload.title.unwrap_or(article.title);
    let new_slug = payload.slug.unwrap_or(article.slug);
    let has_new_markdown = payload.markdown.is_some();
    let new_markdown = payload.markdown.unwrap_or_else(|| article.markdown.clone());
    let new_html = if has_new_markdown {
        markdown_to_html(&new_markdown)
    } else {
        article.html
    };
    let new_published = payload.published.unwrap_or(article.published);

    sqlx::query("UPDATE articles SET title = ?, slug = ?, markdown = ?, html = ?, published = ?, updated_at = NOW() WHERE id = ?")
        .bind(&new_title)
        .bind(&new_slug)
        .bind(&new_markdown)
        .bind(&new_html)
        .bind(new_published)
        .bind(&id_bytes)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({ "message": "updated" })))
}

pub async fn delete_article(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let uuid = Uuid::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let id_bytes = uuid.as_bytes().to_vec();

    sqlx::query("DELETE FROM articles WHERE id = ?")
        .bind(&id_bytes)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({ "message": "deleted" })))
}
