use axum::extract::Query;
use axum::{extract::{State, Path, }, Json};
use uuid::Uuid;
use std::sync::Arc;
use super::service::DealService;
use cores::AppResult;
use shared::pagination::PaginationParams;
use super::dto::*;

pub async fn create(
    State(svc): State<Arc<DealService>>,
    Json(dto): Json<CreateDealDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.create(
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
    State(svc): State<Arc<DealService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.get(id, q.org_id).await?
    )))
}

pub async fn list(
    State(svc): State<Arc<DealService>>,
    Query(q): Query<OrgQuery>,
    Query(p): Query<PaginationParams>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.list(q.org_id, &p).await?
    )))
}

pub async fn advance_stage(
    State(svc): State<Arc<DealService>>,
    Path(id): Path<Uuid>,
    Json(dto): Json<AdvanceStageDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.advance_stage(id, dto.org_id, dto.next).await?
    )))
}

pub async fn agent_pipeline(
    State(svc): State<Arc<DealService>>,
    Path(agent_id): Path<Uuid>,
    Query(p): Query<PaginationParams>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.agent_pipeline(agent_id, &p).await?
    )))
}


