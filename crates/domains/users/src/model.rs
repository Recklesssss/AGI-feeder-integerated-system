use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct User {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub password_hash: String,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
}
pub enum UserStatus {
    Active,
    Locked,
    Suspended,
}