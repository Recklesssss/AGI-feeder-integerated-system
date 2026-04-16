use serde::Deserialize;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::NaiveDate;

#[derive(Debug, Deserialize)]
pub struct RecordPaymentDto {
    pub invoice_id: Uuid,
    pub amount:     Decimal,
    pub method:     String,
    pub paid_at:    Option<NaiveDate>,
}