use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Vec<u8>,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: Option<OffsetDateTime>,
}

impl User {
    pub fn id_as_uuid(&self) -> Option<Uuid> {
        if self.id.len() == 16 {
            let bytes: [u8; 16] = self.id.as_slice().try_into().ok()?;
            Some(Uuid::from_bytes(bytes))
        } else {
            Uuid::parse_str(std::str::from_utf8(&self.id).ok()?).ok()
        }
    }
}
