use serde::Deserialize;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::NaiveDate;

#[derive(Deserialize)]
pub struct CreateInvoiceDto {
    pub org_id: Uuid,
    pub asset_id: Option<Uuid>,
    pub total: Decimal,
    pub issued_at: Option<NaiveDate>,
}