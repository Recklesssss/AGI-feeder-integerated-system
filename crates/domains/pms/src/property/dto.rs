use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreatePropertyDto {
    pub org_id: Uuid,
    pub asset_id: Uuid,
    pub address: String,
    pub city: Option<String>,
    pub country: Option<String>,
}

#[derive(Deserialize)]
pub struct OrgQuery {
    pub org_id: Uuid,
}