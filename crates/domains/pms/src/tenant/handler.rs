use axum::extract::Query;
use axum::{extract::{State, Path, }, Json};
use uuid::Uuid;
use std::sync::Arc;
use super::service::TenantService;
use cores::AppResult;
use shared::pagination::PaginationParams;
use super::dto::*;

pub async fn create(
    State(svc): State<Arc<TenantService>>,
    Json(dto): Json<CreateTenantDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.create(
            dto.org_id,
            &dto.name,
            dto.email.as_deref(),
            dto.phone.as_deref(),
            dto.national_id.as_deref(),
        ).await?
    )))
}

pub async fn get(
    State(svc): State<Arc<TenantService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.get(id, q.org_id).await?
    )))
}

pub async fn list(
    State(svc): State<Arc<TenantService>>,
    Query(q): Query<OrgQuery>,
    Query(p): Query<PaginationParams>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.list(q.org_id, &p).await?
    )))
}

pub async fn delete(
    State(svc): State<Arc<TenantService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<()> {
    svc.delete(id, q.org_id).await?;
    Ok(())
}


