use axum::{extract::{State, Path, Query}, Json};
use uuid::Uuid;
use crate::AppState;
use core_lib::AppResult;
use shared_lib::pagination::PaginationParams;
use super::dto::*;

pub async fn create(
    State(state): State<AppState>,
    Json(dto): Json<CreateUnitDto>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.unit_service.create(
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
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.unit_service.get(id, q.org_id).await?
    )))
}

pub async fn list_by_property(
    State(state): State<AppState>,
    Query(q): Query<PropertyQuery>,
    Query(p): Query<PaginationParams>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.unit_service.list_by_property(q.property_id, &p).await?
    )))
}

pub async fn mark_occupied(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.unit_service.mark_occupied(id, q.org_id).await?
    )))
}

pub async fn mark_vacant(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.unit_service.mark_vacant(id, q.org_id).await?
    )))
}

pub async fn vacancy_count(
    State(state): State<AppState>,
    Query(q): Query<PropertyQuery>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.unit_service.vacancy_count(q.property_id).await?
    )))
}