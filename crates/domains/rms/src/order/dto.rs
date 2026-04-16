use serde::Deserialize;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::NaiveDate;
use super::model::NewOrderItem;

#[derive(Deserialize)]
pub struct OpenOrderDto {
    pub org_id: Uuid,
    pub restaurant_id: Uuid,
    pub table_number: Option<String>,
    pub opened_by: Option<Uuid>,
    pub items: Vec<NewOrderItem>,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct CloseOrderDto {
    pub org_id: Uuid,
    pub tax_rate: Decimal,
    pub service_charge_rate: Decimal,
    pub discount: Decimal,
    pub payment_method: String,
    pub closed_by: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct OrgQuery {
    pub org_id: Uuid,
}

#[derive(Deserialize)]
pub struct ListOrderQuery {
    pub org_id: Uuid,
    pub restaurant_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct RevenueQuery {
    pub org_id: Uuid,
    pub restaurant_id: Uuid,
    pub date: NaiveDate,
}