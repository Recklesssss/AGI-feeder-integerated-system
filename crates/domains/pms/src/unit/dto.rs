use serde::Deserialize;
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Deserialize)]
pub struct CreateUnitDto {
    pub org_id: Uuid,
    pub property_id: Uuid,
    pub asset_id: Uuid,
    pub unit_number: String,
    pub floor: Option<i32>,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub area_sqm: Option<Decimal>,
}

#[derive(Deserialize)]
pub struct OrgQuery {
    pub org_id: Uuid,
}

#[derive(Deserialize)]
pub struct PropertyQuery {
    pub property_id: Uuid,
}