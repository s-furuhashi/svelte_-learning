use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Book {
    pub id: Vec<u8>,
    pub title: String,
    pub slug: String,
    pub markdown: String,
    pub html: String,
    pub image_url: Option<String>,
    pub published: bool,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBookRequest {
    pub title: String,
    pub slug: String,
    pub markdown: String,
    pub image_url: Option<String>,
    pub published: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBookRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub markdown: Option<String>,
    pub image_url: Option<String>,
    pub published: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct BookResponse {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub markdown: String,
    pub html: String,
    pub image_url: Option<String>,
    pub published: bool,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

impl From<Book> for BookResponse {
    fn from(b: Book) -> Self {
        let id = uuid::Uuid::from_slice(&b.id)
            .map(|u| u.to_string())
            .unwrap_or_default();
        BookResponse {
            id,
            title: b.title,
            slug: b.slug,
            markdown: b.markdown,
            html: b.html,
            image_url: b.image_url,
            published: b.published,
            created_at: b.created_at,
            updated_at: b.updated_at,
        }
    }
}
