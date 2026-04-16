use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus { Pending, Paid, Cancelled }

impl OrderStatus {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Pending => "pending", Self::Paid => "paid", Self::Cancelled => "cancelled" }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub order_id:     Uuid,
    pub menu_item_id: Uuid,
    pub quantity:     i32,
    pub unit_price:   Decimal,
    pub line_total:   Decimal,
    pub notes:        Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id:              Uuid,
    pub organization_id: Uuid,
    pub restaurant_id:   Uuid,
    pub table_number:    Option<String>,
    pub subtotal:        Decimal,
    pub tax:             Decimal,
    pub service_charge:  Decimal,
    pub discount:        Decimal,
    pub total:           Decimal,
    pub payment_method:  Option<String>,
    pub status:          OrderStatus,
    pub opened_by:       Option<Uuid>,
    pub closed_by:       Option<Uuid>,
    pub closed_at:       Option<DateTime<Utc>>,
    pub notes:           Option<String>,
    pub items:           Vec<OrderItem>,
    pub created_at:      DateTime<Utc>,
    pub updated_at:      DateTime<Utc>,
}

/// POS new order input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewOrderItem {
    pub menu_item_id: Uuid,
    pub quantity:     i32,
    pub notes:        Option<String>,
}
