use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateRestaurantDto {
    pub org_id: Uuid,
    pub asset_id: Uuid,
    pub name: String,
    pub address: Option<String>,
}

#[derive(Deserialize)]
pub struct OrgQuery {
    pub org_id: Uuid,
}