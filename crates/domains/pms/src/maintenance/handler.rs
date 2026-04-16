use axum::{extract::{State, Path, Query}, Json};
use uuid::Uuid;
use crate::AppState;
use core_lib::AppResult;
use shared_lib::pagination::PaginationParams;
use super::dto::*;

pub async fn create(
    State(state): State<AppState>,
    Json(dto): Json<CreateMaintenanceDto>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.maintenance_service.create(
            dto.org_id,
            dto.unit_id,
            &dto.description,
            &dto.priority,
            dto.reported_by,
        ).await?
    )))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.maintenance_service.get(id, q.org_id).await?
    )))
}

pub async fn list(
    State(state): State<AppState>,
    Query(q): Query<OrgQuery>,
    Query(p): Query<PaginationParams>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.maintenance_service.list(q.org_id, &p).await?
    )))
}

pub async fn assign(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(dto): Json<AssignDto>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.maintenance_service.assign(id, dto.org_id, dto.user_id).await?
    )))
}

pub async fn resolve(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(dto): Json<ResolveDto>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.maintenance_service.resolve(id, dto.org_id, dto.actual_cost).await?
    )))
}

pub async fn close(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.maintenance_service.close(id, q.org_id).await?
    )))
}