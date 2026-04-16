use serde::Deserialize;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::NaiveDate;

#[derive(Deserialize)]
pub struct CreateLeaseDto {
    pub org_id: Uuid,
    pub unit_id: Uuid,
    pub tenant_id: Uuid,
    pub rent: Decimal,
    pub security_deposit: Decimal,
    pub late_fee: Decimal,
    pub billing_day: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct OrgQuery {
    pub org_id: Uuid,
}

#[derive(Deserialize)]
pub struct ExpiringQuery {
    pub org_id: Uuid,
    pub within_days: i32,
}