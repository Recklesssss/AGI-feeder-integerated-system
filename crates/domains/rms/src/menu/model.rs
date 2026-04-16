use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id:            Uuid,
    pub restaurant_id: Uuid,
    pub name:          String,
    pub description:   Option<String>,
    pub category:      Option<String>,
    pub price:         Decimal,
    pub cost:          Decimal,      // COGS per unit
    pub is_available:  bool,
    pub created_at:    DateTime<Utc>,
    pub updated_at:    DateTime<Utc>,
    pub deleted_at:    Option<DateTime<Utc>>,
}

impl MenuItem {
    /// Margin percentage: (price - cost) / price * 100
    pub fn margin_pct(&self) -> Option<Decimal> {
        if self.price == Decimal::ZERO { return None; }
        Some((self.price - self.cost) / self.price * Decimal::from(100))
    }
}
