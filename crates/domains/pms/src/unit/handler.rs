use axum::extract::Query;
use axum::{extract::{State, Path, }, Json};
use uuid::Uuid;
use std::sync::Arc;
use super::service::UnitService;
use cores::AppResult;
use shared::pagination::PaginationParams;
use super::dto::*;

pub async fn create(
    State(svc): State<Arc<UnitService>>,
    Json(dto): Json<CreateUnitDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.create(
            dto.org_id,
            dto.property_id,
            dto.asset_id,
            &dto.unit_number,
            dto.floor,
            dto.bedrooms,
            dto.bathrooms,
            dto.area_sqm,
        ).await?
    )))
}

pub async fn get(
    State(svc): State<Arc<UnitService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.get(id, q.org_id).await?
    )))
}

pub async fn list_by_property(
    State(svc): State<Arc<UnitService>>,
    Query(q): Query<PropertyQuery>,
    Query(p): Query<PaginationParams>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.list_by_property(q.property_id, &p).await?
    )))
}

pub async fn mark_occupied(
    State(svc): State<Arc<UnitService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.mark_occupied(id, q.org_id).await?
    )))
}

pub async fn mark_vacant(
    State(svc): State<Arc<UnitService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.mark_vacant(id, q.org_id).await?
    )))
}

pub async fn vacancy_count(
    State(svc): State<Arc<UnitService>>,
    Query(q): Query<PropertyQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.vacancy_count(q.property_id).await?
    )))
}


