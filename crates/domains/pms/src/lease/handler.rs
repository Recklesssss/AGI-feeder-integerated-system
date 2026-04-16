use axum::extract::Query;
use axum::{extract::{State, Path, }, Json};
use uuid::Uuid;
use std::sync::Arc;
use super::service::LeaseService;
use cores::AppResult;
use shared::pagination::PaginationParams;
use super::dto::*;

pub async fn create(
    State(svc): State<Arc<LeaseService>>,
    Json(dto): Json<CreateLeaseDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.create(
            dto.org_id,
            dto.unit_id,
            dto.tenant_id,
            dto.rent,
            dto.security_deposit,
            dto.late_fee,
            dto.billing_day,
            dto.start_date,
            dto.end_date,
            dto.notes.as_deref(),
        ).await?
    )))
}

pub async fn get(
    State(svc): State<Arc<LeaseService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.get(id, q.org_id).await?
    )))
}

pub async fn list(
    State(svc): State<Arc<LeaseService>>,
    Query(q): Query<OrgQuery>,
    Query(p): Query<PaginationParams>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.list(q.org_id, &p).await?
    )))
}

pub async fn terminate(
    State(svc): State<Arc<LeaseService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.terminate(id, q.org_id).await?
    )))
}

pub async fn expiring(
    State(svc): State<Arc<LeaseService>>,
    Query(q): Query<ExpiringQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.expiring_soon(q.org_id, q.within_days).await?
    )))
}


