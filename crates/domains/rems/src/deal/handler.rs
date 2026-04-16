use axum::{extract::{State, Path, Query}, Json};
use uuid::Uuid;
use crate::AppState;
use core_lib::AppResult;
use shared_lib::pagination::PaginationParams;
use super::dto::*;

pub async fn create(
    State(state): State<AppState>,
    Json(dto): Json<CreateDealDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        state.deal_service.create(
            dto.org_id,
            dto.listing_id,
            dto.client_id,
            dto.agent_id,
            dto.deal_value,
            dto.notes.as_deref(),
        ).await?
    )))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        state.deal_service.get(id, q.org_id).await?
    )))
}

pub async fn list(
    State(state): State<AppState>,
    Query(q): Query<OrgQuery>,
    Query(p): Query<PaginationParams>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        state.deal_service.list(q.org_id, &p).await?
    )))
}

pub async fn advance_stage(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(dto): Json<AdvanceStageDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        state.deal_service.advance_stage(id, dto.org_id, dto.next).await?
    )))
}

pub async fn agent_pipeline(
    State(state): State<AppState>,
    Path(agent_id): Path<Uuid>,
    Query(p): Query<PaginationParams>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        state.deal_service.agent_pipeline(agent_id, &p).await?
    )))
}