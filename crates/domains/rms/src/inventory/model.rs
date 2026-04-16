use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id:            Uuid,
    pub restaurant_id: Uuid,
    pub name:          String,
    pub unit:          String,      // "kg" | "liter" | "unit"
    pub quantity:      Decimal,
    pub reorder_level: Decimal,
    pub cost_per_unit: Decimal,
    pub created_at:    DateTime<Utc>,
    pub updated_at:    DateTime<Utc>,
    pub deleted_at:    Option<DateTime<Utc>>,
}

impl InventoryItem {
    /// Returns true if stock is at or below reorder level.
    pub fn needs_reorder(&self) -> bool {
        self.quantity <= self.reorder_level
    }
}
