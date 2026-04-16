use axum::{extract::{State, Path, Query}, Json};
use uuid::Uuid;
use crate::AppState;
use core_lib::AppResult;
use shared_lib::pagination::PaginationParams;
use super::dto::*;

pub async fn create(
    State(state): State<AppState>,
    Json(dto): Json<CreateRestaurantDto>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.restaurant_service.create(
            dto.org_id,
            dto.asset_id,
            &dto.name,
            dto.address.as_deref(),
        ).await?
    )))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.restaurant_service.get(id, q.org_id).await?
    )))
}

pub async fn list(
    State(state): State<AppState>,
    Query(q): Query<OrgQuery>,
    Query(p): Query<PaginationParams>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.restaurant_service.list(q.org_id, &p).await?
    )))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<()> {
    state.restaurant_service.delete(id, q.org_id).await?;
    Ok(())
}