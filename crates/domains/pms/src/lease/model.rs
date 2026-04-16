use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LeaseStatus {
    Active,
    Terminated,
    Expired,
}

impl LeaseStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            LeaseStatus::Active     => "active",
            LeaseStatus::Terminated => "terminated",
            LeaseStatus::Expired    => "expired",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lease {
    pub id:               Uuid,
    pub organization_id:  Uuid,
    pub unit_id:          Uuid,
    pub tenant_id:        Uuid,
    pub rent:             Decimal,
    pub security_deposit: Decimal,
    pub late_fee:         Decimal,
    pub billing_day:      i32,
    pub start_date:       NaiveDate,
    pub end_date:         NaiveDate,
    pub status:           LeaseStatus,
    pub notes:            Option<String>,
    pub created_at:       DateTime<Utc>,
    pub updated_at:       DateTime<Utc>,
    pub deleted_at:       Option<DateTime<Utc>>,
}
