use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthCredentials {
    pub id:              Uuid,
    pub organization_id: Uuid,
    pub email:           String,
    pub password_hash:   String,
    pub status:          UserAuthStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UserAuthStatus {
    Active,
    Inactive,
}

impl UserAuthStatus {
    pub fn from_str(s: &str) -> Self {
        match s {
            "active" => Self::Active,
            _        => Self::Inactive,
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
}
