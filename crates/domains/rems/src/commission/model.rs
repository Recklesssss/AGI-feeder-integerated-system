use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CommissionStatus { Pending, Approved, Paid }

impl CommissionStatus {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Pending => "pending", Self::Approved => "approved", Self::Paid => "paid" }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commission {
    pub id:         Uuid,
    pub deal_id:    Uuid,
    pub agent_id:   Uuid,
    pub amount:     Decimal,
    pub percentage: Option<Decimal>,
    pub status:     CommissionStatus,
    pub paid_at:    Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}
