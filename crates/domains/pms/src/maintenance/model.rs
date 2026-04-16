use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MaintenancePriority { Low, Normal, High, Urgent }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MaintenanceStatus { Open, InProgress, Resolved, Closed }

impl MaintenancePriority {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Low => "low", Self::Normal => "normal", Self::High => "high", Self::Urgent => "urgent" }
    }
}

impl MaintenanceStatus {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Open => "open", Self::InProgress => "in_progress", Self::Resolved => "resolved", Self::Closed => "closed" }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceRequest {
    pub id:              Uuid,
    pub organization_id: Uuid,
    pub unit_id:         Uuid,
    pub reported_by:     Option<Uuid>,
    pub assigned_to:     Option<Uuid>,
    pub description:     String,
    pub priority:        MaintenancePriority,
    pub status:          MaintenanceStatus,
    pub estimated_cost:  Option<Decimal>,
    pub actual_cost:     Option<Decimal>,
    pub resolved_at:     Option<DateTime<Utc>>,
    pub created_at:      DateTime<Utc>,
    pub updated_at:      DateTime<Utc>,
}
