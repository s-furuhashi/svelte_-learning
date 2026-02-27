use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: Vec<u8>,
    pub user_id: Vec<u8>,
    pub expires_at: OffsetDateTime,
    pub created_at: Option<OffsetDateTime>,
}

impl Session {
    pub fn id_as_str(&self) -> String {
        if let Ok(s) = std::str::from_utf8(&self.id) {
            s.to_string()
        } else {
            Uuid::from_slice(&self.id).map(|u| u.to_string()).unwrap_or_default()
        }
    }
}
