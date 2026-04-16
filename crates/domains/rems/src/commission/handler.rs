use axum::{extract::{State, Path}, Json};
use uuid::Uuid;
use std::sync::Arc;
use super::service::CommissionService;
use cores::AppResult;
use super::dto::*;

pub async fn create(
    State(svc): State<Arc<CommissionService>>,
    Json(dto): Json<CreateCommissionDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.calculate_and_create(
            dto.deal_id,
            dto.agent_id,
            dto.deal_value,
            dto.percentage,
            dto.fixed_amount,
        ).await?
    )))
}

pub async fn for_deal(
    State(svc): State<Arc<CommissionService>>,
    Path(deal_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.for_deal(deal_id).await?
    )))
}

pub async fn for_agent(
    State(svc): State<Arc<CommissionService>>,
    Path(agent_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.for_agent(agent_id).await?
    )))
}

pub async fn approve(
    State(svc): State<Arc<CommissionService>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.approve(id).await?
    )))
}

pub async fn pay(
    State(svc): State<Arc<CommissionService>>,
    Path(id): Path<Uuid>,
    Json(dto): Json<PayCommissionDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.pay(id, dto.paid_at).await?
    )))
}
