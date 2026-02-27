use axum::{
    extract::{Path, State},
    Json,
    http::StatusCode,
};
use serde_json::{json, Value};
use crate::AppState;
use crate::models::book::{Book, BookResponse};

pub async fn list_books(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    let books = sqlx::query_as::<_, Book>(
        "SELECT id, title, slug, markdown, html, image_url, published, created_at, updated_at FROM books WHERE published = true ORDER BY created_at DESC"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let responses: Vec<BookResponse> = books.into_iter().map(|b| b.into()).collect();
    Ok(Json(json!({ "books": responses })))
}

pub async fn get_book(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let book = sqlx::query_as::<_, Book>(
        "SELECT id, title, slug, markdown, html, image_url, published, created_at, updated_at FROM books WHERE slug = ? AND published = true"
    )
    .bind(&slug)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let response: BookResponse = book.into();
    Ok(Json(json!({ "book": response })))
}
