use axum::{extract::{Path, Query, State}, Json};
use std::sync::Arc;
use uuid::Uuid;
use serde::Deserialize;

use cores::AppResult;
use shared::pagination::PaginationParams;
use super::{model::Organization, service::OrgService};

#[derive(Debug, Deserialize)]
pub struct CreateOrgDto {
    pub name: String,
}

pub async fn list_orgs(
    State(svc): State<Arc<OrgService>>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<serde_json::Value>> {
    let (orgs, total) = svc.list(params.limit(), params.offset()).await?;
    Ok(Json(serde_json::json!({ "items": orgs, "total": total })))
}

pub async fn create_org(
    State(svc): State<Arc<OrgService>>,
    Json(dto): Json<CreateOrgDto>,
) -> AppResult<Json<Organization>> {
    let org = svc.create(dto.name).await?;
    Ok(Json(org))
}

pub async fn get_org(
    State(svc): State<Arc<OrgService>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Organization>> {
    let org = svc.get(id).await?;
    Ok(Json(org))
}

pub async fn suspend_org(
    State(svc): State<Arc<OrgService>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Organization>> {
    let org = svc.suspend(id).await?;
    Ok(Json(org))
}

pub async fn activate_org(
    State(svc): State<Arc<OrgService>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Organization>> {
    let org = svc.activate(id).await?;
    Ok(Json(org))
}
