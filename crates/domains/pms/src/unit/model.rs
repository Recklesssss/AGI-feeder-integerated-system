use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UnitStatus {
    Vacant,
    Occupied,
    UnderMaintenance,
}

impl UnitStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            UnitStatus::Vacant           => "vacant",
            UnitStatus::Occupied         => "occupied",
            UnitStatus::UnderMaintenance => "under_maintenance",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub id:              Uuid,
    pub organization_id: Uuid,
    pub property_id:     Uuid,
    pub asset_id:        Uuid,
    pub unit_number:     String,
    pub floor:           Option<i32>,
    pub bedrooms:        Option<i32>,
    pub bathrooms:       Option<i32>,
    pub area_sqm:        Option<Decimal>,
    pub status:          UnitStatus,
    pub created_at:      DateTime<Utc>,
    pub updated_at:      DateTime<Utc>,
    pub deleted_at:      Option<DateTime<Utc>>,
}
