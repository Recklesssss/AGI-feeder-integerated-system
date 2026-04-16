use serde::Deserialize;
use uuid::Uuid;
use rust_decimal::Decimal;
use super::model::MovementType;
use chrono::NaiveDate;

#[derive(Deserialize)]
pub struct RecordStockDto {
    pub inventory_item_id: Uuid,
    pub quantity: Decimal,
    pub movement_type: MovementType,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub notes: Option<String>,
    pub recorded_by: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct HistoryQuery {
    pub limit: i64,
    pub offset: i64,
}

#[derive(Deserialize)]
pub struct WasteQuery {
    pub restaurant_id: Uuid,
    pub from: NaiveDate,
    pub to: NaiveDate,
}