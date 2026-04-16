use axum::{extract::{State, Path, Query}, Json};
use uuid::Uuid;
use crate::AppState;
use core_lib::AppResult;
use super::dto::*;

pub async fn create(
    State(state): State<AppState>,
    Json(dto): Json<CreateCommissionDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        state.commission_service.calculate_and_create(
            dto.deal_id,
            dto.agent_id,
            dto.deal_value,
            dto.percentage,
            dto.fixed_amount,
        ).await?
    )))
}

pub async fn for_deal(
    State(state): State<AppState>,
    Path(deal_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        state.commission_service.for_deal(deal_id).await?
    )))
}

pub async fn for_agent(
    State(state): State<AppState>,
    Path(agent_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        state.commission_service.for_agent(agent_id).await?
    )))
}

pub async fn approve(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        state.commission_service.approve(id).await?
    )))
}

pub async fn pay(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(dto): Json<PayCommissionDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        state.commission_service.pay(id, dto.paid_at).await?
    )))
}