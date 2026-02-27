use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Article {
    pub id: Vec<u8>,
    pub title: String,
    pub slug: String,
    pub markdown: String,
    pub html: String,
    pub published: bool,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateArticleRequest {
    pub title: String,
    pub slug: String,
    pub markdown: String,
    pub published: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArticleRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub markdown: Option<String>,
    pub published: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ArticleResponse {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub markdown: String,
    pub html: String,
    pub published: bool,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

impl From<Article> for ArticleResponse {
    fn from(a: Article) -> Self {
        let id = uuid::Uuid::from_slice(&a.id)
            .map(|u| u.to_string())
            .unwrap_or_default();
        ArticleResponse {
            id,
            title: a.title,
            slug: a.slug,
            markdown: a.markdown,
            html: a.html,
            published: a.published,
            created_at: a.created_at,
            updated_at: a.updated_at,
        }
    }
}
