use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MovementType { In, Out, Adjustment, Waste }

impl MovementType {
    pub fn as_str(&self) -> &'static str {
        match self { Self::In => "in", Self::Out => "out", Self::Adjustment => "adjustment", Self::Waste => "waste" }
    }

    /// Returns the signed delta to apply to inventory quantity.
    pub fn signed_delta(&self, qty: Decimal) -> Decimal {
        match self {
            Self::In | Self::Adjustment => qty,
            Self::Out | Self::Waste     => -qty,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockMovement {
    pub id:                Uuid,
    pub inventory_item_id: Uuid,
    pub quantity:          Decimal,
    pub movement_type:     MovementType,
    pub reference_type:    Option<String>,
    pub reference_id:      Option<Uuid>,
    pub notes:             Option<String>,
    pub recorded_by:       Option<Uuid>,
    pub created_at:        DateTime<Utc>,
}
