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
    pub markdown: String,
    pub published: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArticleRequest {
    pub title: Option<String>,
    pub markdown: Option<String>,
    pub published: Option<bool>,
}

/// Full article response (admin), includes markdown and published flag.
#[derive(Debug, Serialize)]
pub struct AdminArticleResponse {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub markdown: String,
    pub html: String,
    pub published: bool,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

impl From<Article> for AdminArticleResponse {
    fn from(a: Article) -> Self {
        let id = uuid::Uuid::from_slice(&a.id)
            .map(|u| u.to_string())
            .unwrap_or_default();
        AdminArticleResponse {
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

/// Public list response: id, title, slug, created_at, updated_at only.
#[derive(Debug, Serialize)]
pub struct PublicArticleListResponse {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

impl From<Article> for PublicArticleListResponse {
    fn from(a: Article) -> Self {
        let id = uuid::Uuid::from_slice(&a.id)
            .map(|u| u.to_string())
            .unwrap_or_default();
        PublicArticleListResponse {
            id,
            title: a.title,
            slug: a.slug,
            created_at: a.created_at,
            updated_at: a.updated_at,
        }
    }
}

/// Public detail response: id, title, slug, html, created_at, updated_at (no markdown).
#[derive(Debug, Serialize)]
pub struct PublicArticleDetailResponse {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub html: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

impl From<Article> for PublicArticleDetailResponse {
    fn from(a: Article) -> Self {
        let id = uuid::Uuid::from_slice(&a.id)
            .map(|u| u.to_string())
            .unwrap_or_default();
        PublicArticleDetailResponse {
            id,
            title: a.title,
            slug: a.slug,
            html: a.html,
            created_at: a.created_at,
            updated_at: a.updated_at,
        }
    }
}

