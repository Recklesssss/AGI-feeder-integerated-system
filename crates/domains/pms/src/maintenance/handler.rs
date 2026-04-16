use axum::extract::Query;
use axum::{extract::{State, Path, }, Json};
use uuid::Uuid;
use std::sync::Arc;
use super::service::MaintenanceService;
use cores::AppResult;
use shared::pagination::PaginationParams;
use super::dto::*;

pub async fn create(
    State(svc): State<Arc<MaintenanceService>>,
    Json(dto): Json<CreateMaintenanceDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.create(
            dto.org_id,
            dto.unit_id,
            &dto.description,
            &dto.priority,
            dto.reported_by,
        ).await?
    )))
}

pub async fn get(
    State(svc): State<Arc<MaintenanceService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.get(id, q.org_id).await?
    )))
}

pub async fn list(
    State(svc): State<Arc<MaintenanceService>>,
    Query(q): Query<OrgQuery>,
    Query(p): Query<PaginationParams>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.list(q.org_id, &p).await?
    )))
}

pub async fn assign(
    State(svc): State<Arc<MaintenanceService>>,
    Path(id): Path<Uuid>,
    Json(dto): Json<AssignDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.assign(id, dto.org_id, dto.user_id).await?
    )))
}

pub async fn resolve(
    State(svc): State<Arc<MaintenanceService>>,
    Path(id): Path<Uuid>,
    Json(dto): Json<ResolveDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.resolve(id, dto.org_id, dto.actual_cost).await?
    )))
}

pub async fn close(
    State(svc): State<Arc<MaintenanceService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.close(id, q.org_id).await?
    )))
}


