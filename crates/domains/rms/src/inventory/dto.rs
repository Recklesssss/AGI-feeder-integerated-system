use serde::Deserialize;
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Deserialize)]
pub struct CreateInventoryDto {
    pub restaurant_id: Uuid,
    pub name: String,
    pub unit: String,
    pub reorder_level: Decimal,
    pub cost_per_unit: Decimal,
}

#[derive(Deserialize)]
pub struct AdjustDto {
    pub delta: Decimal,
}