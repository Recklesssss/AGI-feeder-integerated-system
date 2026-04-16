use serde::Deserialize;
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Deserialize)]
pub struct CreateMaintenanceDto {
    pub org_id: Uuid,
    pub unit_id: Uuid,
    pub description: String,
    pub priority: String,
    pub reported_by: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct AssignDto {
    pub org_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Deserialize)]
pub struct ResolveDto {
    pub org_id: Uuid,
    pub actual_cost: Option<Decimal>,
}

#[derive(Deserialize)]
pub struct OrgQuery {
    pub org_id: Uuid,
}