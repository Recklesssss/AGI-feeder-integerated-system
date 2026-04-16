use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a single immutable audit log entry.
/// These records are strictly INSERT-only — no updates or deletes are ever permitted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id:              Uuid,
    pub organization_id: Uuid,
    pub user_id:         Uuid,
    pub action:          AuditAction,
    pub entity:          String,   // e.g. "asset", "invoice", "user"
    pub entity_id:       Uuid,
    pub metadata:        serde_json::Value,
    pub created_at:      DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    Create,
    Update,
    Delete,
}

impl AuditAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditAction::Create => "create",
            AuditAction::Update => "update",
            AuditAction::Delete => "delete",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "update" => AuditAction::Update,
            "delete" => AuditAction::Delete,
            _        => AuditAction::Create,
        }
    }
}
