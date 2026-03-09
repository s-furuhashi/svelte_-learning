use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: Option<OffsetDateTime>,
}

impl User {
    pub fn id_as_uuid(&self) -> Option<Uuid> {
        Uuid::parse_str(&self.id).ok()
    }
}
