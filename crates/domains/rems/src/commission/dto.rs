use serde::Deserialize;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::NaiveDate;

#[derive(Deserialize)]
pub struct CreateCommissionDto {
    pub deal_id: Uuid,
    pub agent_id: Uuid,
    pub deal_value: Decimal,
    pub percentage: Option<Decimal>,
    pub fixed_amount: Option<Decimal>,
}

#[derive(Deserialize)]
pub struct PayCommissionDto {
    pub paid_at: NaiveDate,
}