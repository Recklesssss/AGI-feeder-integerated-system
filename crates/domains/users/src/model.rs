use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id:            Uuid,
    pub email:         String,
    pub full_name:     String,
    pub password_hash: String,
    pub status:        UserStatus,
    pub created_at:    DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Locked,
    Suspended,
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Active    => write!(f, "active"),
            UserStatus::Locked    => write!(f, "locked"),
            UserStatus::Suspended => write!(f, "suspended"),
        }
    }
}