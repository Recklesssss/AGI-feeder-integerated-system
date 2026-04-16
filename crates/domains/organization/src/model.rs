use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrgStatus {
    Active,
    Suspended,
}

impl OrgStatus {
    pub fn from_str(s: &str) -> Self {
        match s {
            "suspended" => OrgStatus::Suspended,
            _           => OrgStatus::Active,
        }
    }
}

impl std::fmt::Display for OrgStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrgStatus::Active    => write!(f, "active"),
            OrgStatus::Suspended => write!(f, "suspended"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id:         Uuid,
    pub name:       String,
    pub status:     OrgStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
