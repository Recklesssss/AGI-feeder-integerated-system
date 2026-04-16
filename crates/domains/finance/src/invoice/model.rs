use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id:              Uuid,
    pub organization_id: Uuid,
    pub asset_id:        Option<Uuid>,
    pub total:           Decimal,
    pub status:          InvoiceStatus,
    pub issued_at:       Option<NaiveDate>,
    pub created_at:      DateTime<Utc>,
    pub deleted_at:      Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Draft,
    Issued,
    Paid,
    Cancelled,
}

impl InvoiceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            InvoiceStatus::Draft     => "draft",
            InvoiceStatus::Issued    => "issued",
            InvoiceStatus::Paid      => "paid",
            InvoiceStatus::Cancelled => "cancelled",
        }
    }

    pub fn can_transition_to(&self, next: &InvoiceStatus) -> bool {
        matches!(
            (self, next),
            (InvoiceStatus::Draft,   InvoiceStatus::Issued)    |
            (InvoiceStatus::Issued,  InvoiceStatus::Paid)      |
            (InvoiceStatus::Draft,   InvoiceStatus::Cancelled) |
            (InvoiceStatus::Issued,  InvoiceStatus::Cancelled)
        )
    }
}
