use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateTenantDto {
    pub org_id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub national_id: Option<String>,
}

#[derive(Deserialize)]
pub struct OrgQuery {
    pub org_id: Uuid,
}