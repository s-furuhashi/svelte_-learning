use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Article {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub markdown: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateArticleRequest {
    pub title: String,
    pub markdown: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArticleRequest {
    pub title: Option<String>,
    pub markdown: Option<String>,
}

/// Public list response: title, slug, created_at.
#[derive(Debug, Serialize)]
pub struct ArticleListItem {
    pub title: String,
    pub slug: String,
    pub created_at: Option<String>,
}

impl From<Article> for ArticleListItem {
    fn from(a: Article) -> Self {
        ArticleListItem {
            title: a.title,
            slug: a.slug,
            created_at: a.created_at,
        }
    }
}

/// Public detail response: title, slug, html (converted from markdown), created_at.
#[derive(Debug, Serialize)]
pub struct ArticleDetail {
    pub title: String,
    pub slug: String,
    pub html: String,
    pub created_at: Option<String>,
}

/// Admin response: includes id and markdown.
#[derive(Debug, Serialize)]
pub struct AdminArticleResponse {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub markdown: String,
    pub created_at: Option<String>,
}

impl From<Article> for AdminArticleResponse {
    fn from(a: Article) -> Self {
        AdminArticleResponse {
            id: a.id,
            title: a.title,
            slug: a.slug,
            markdown: a.markdown,
            created_at: a.created_at,
        }
    }
}

