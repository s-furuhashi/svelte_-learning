use axum::{
    extract::{Path, State},
    Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::{json, Value};
use uuid::Uuid;
use crate::AppState;
use crate::models::article::{Article, AdminArticleResponse, CreateArticleRequest, UpdateArticleRequest};
use crate::utils::markdown::markdown_to_html;
use crate::utils::slug::{generate_slug, make_unique_slug};

fn error_response(status: StatusCode, message: &str) -> (StatusCode, Json<Value>) {
    (status, Json(json!({ "error": message })))
}

pub async fn list_articles(
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let articles = sqlx::query_as::<_, Article>(
        "SELECT id, title, slug, markdown, html, published, created_at, updated_at FROM articles ORDER BY created_at DESC"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        error_response(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })?;

    let responses: Vec<AdminArticleResponse> = articles.into_iter().map(|a| a.into()).collect();
    Ok(Json(json!(responses)))
}

pub async fn get_article_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let uuid = Uuid::parse_str(&id)
        .map_err(|_| error_response(StatusCode::BAD_REQUEST, "Invalid article ID"))?;
    let id_bytes = uuid.as_bytes().to_vec();

    let article = sqlx::query_as::<_, Article>(
        "SELECT id, title, slug, markdown, html, published, created_at, updated_at FROM articles WHERE id = ?"
    )
    .bind(&id_bytes)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        error_response(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })?
    .ok_or_else(|| error_response(StatusCode::NOT_FOUND, "Article not found"))?;

    let response: AdminArticleResponse = article.into();
    Ok(Json(json!(response)))
}

pub async fn create_article(
    State(state): State<AppState>,
    Json(payload): Json<CreateArticleRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    // Validate title
    if payload.title.is_empty() || payload.title.len() > 200 {
        return Err(error_response(StatusCode::BAD_REQUEST, "Title must be between 1 and 200 characters"));
    }
    // Validate markdown
    if payload.markdown.is_empty() {
        return Err(error_response(StatusCode::BAD_REQUEST, "Markdown must not be empty"));
    }

    let id = Uuid::new_v4();
    let id_bytes = id.as_bytes().to_vec();
    let html = markdown_to_html(&payload.markdown);
    let published = payload.published.unwrap_or(false);

    // Generate slug from title
    let base_slug = generate_slug(&payload.title);

    // Check for slug uniqueness; append short uuid suffix if collision
    let slug = {
        let exists: bool = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM articles WHERE slug = ?"
        )
        .bind(&base_slug)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("DB error: {}", e);
            error_response(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        })? > 0;

        if exists {
            make_unique_slug(&base_slug)
        } else {
            base_slug
        }
    };

    sqlx::query(
        "INSERT INTO articles (id, title, slug, markdown, html, published) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&id_bytes)
    .bind(&payload.title)
    .bind(&slug)
    .bind(&payload.markdown)
    .bind(&html)
    .bind(published)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        error_response(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })?;

    let article = sqlx::query_as::<_, Article>(
        "SELECT id, title, slug, markdown, html, published, created_at, updated_at FROM articles WHERE id = ?"
    )
    .bind(&id_bytes)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        error_response(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })?;

    let response: AdminArticleResponse = article.into();
    Ok((StatusCode::CREATED, Json(json!(response))))
}

pub async fn update_article(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateArticleRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let uuid = Uuid::parse_str(&id)
        .map_err(|_| error_response(StatusCode::BAD_REQUEST, "Invalid article ID"))?;
    let id_bytes = uuid.as_bytes().to_vec();

    // Validate title if provided
    if let Some(ref title) = payload.title {
        if title.is_empty() || title.len() > 200 {
            return Err(error_response(StatusCode::BAD_REQUEST, "Title must be between 1 and 200 characters"));
        }
    }
    // Validate markdown if provided
    if let Some(ref md) = payload.markdown {
        if md.is_empty() {
            return Err(error_response(StatusCode::BAD_REQUEST, "Markdown must not be empty"));
        }
    }

    let article = sqlx::query_as::<_, Article>(
        "SELECT id, title, slug, markdown, html, published, created_at, updated_at FROM articles WHERE id = ?"
    )
    .bind(&id_bytes)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        error_response(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })?
    .ok_or_else(|| error_response(StatusCode::NOT_FOUND, "Article not found"))?;

    let new_title = payload.title.unwrap_or(article.title);
    let has_new_markdown = payload.markdown.is_some();
    let new_markdown = payload.markdown.unwrap_or_else(|| article.markdown.clone());
    let new_html = if has_new_markdown {
        markdown_to_html(&new_markdown)
    } else {
        article.html
    };
    let new_published = payload.published.unwrap_or(article.published);

    sqlx::query(
        "UPDATE articles SET title = ?, markdown = ?, html = ?, published = ?, updated_at = NOW() WHERE id = ?"
    )
    .bind(&new_title)
    .bind(&new_markdown)
    .bind(&new_html)
    .bind(new_published)
    .bind(&id_bytes)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        error_response(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })?;

    let updated = sqlx::query_as::<_, Article>(
        "SELECT id, title, slug, markdown, html, published, created_at, updated_at FROM articles WHERE id = ?"
    )
    .bind(&id_bytes)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        error_response(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })?;

    let response: AdminArticleResponse = updated.into();
    Ok(Json(json!(response)))
}

pub async fn delete_article(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    let uuid = Uuid::parse_str(&id)
        .map_err(|_| error_response(StatusCode::BAD_REQUEST, "Invalid article ID"))?;
    let id_bytes = uuid.as_bytes().to_vec();

    sqlx::query("DELETE FROM articles WHERE id = ?")
        .bind(&id_bytes)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("DB error: {}", e);
            error_response(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        })?;

    Ok(StatusCode::NO_CONTENT)
}

