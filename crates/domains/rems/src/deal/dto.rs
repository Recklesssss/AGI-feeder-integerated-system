use serde::Deserialize;
use uuid::Uuid;
use rust_decimal::Decimal;
use super::model::DealStatus;

#[derive(Deserialize)]
pub struct CreateDealDto {
    pub org_id: Uuid,
    pub listing_id: Uuid,
    pub client_id: Uuid,
    pub agent_id: Option<Uuid>,
    pub deal_value: Decimal,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct AdvanceStageDto {
    pub org_id: Uuid,
    pub next: DealStatus,
}

#[derive(Deserialize)]
pub struct OrgQuery {
    pub org_id: Uuid,
}