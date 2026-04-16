use axum::{extract::{State, Path, Query}, Json};
use uuid::Uuid;
use crate::AppState;
use core_lib::AppResult;
use shared_lib::pagination::PaginationParams;
use super::dto::*;

pub async fn create(
    State(state): State<AppState>,
    Json(dto): Json<CreateMenuDto>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.menu_service.create(
            dto.restaurant_id,
            &dto.name,
            dto.description.as_deref(),
            dto.category.as_deref(),
            dto.price,
            dto.cost,
        ).await?
    )))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(state.menu_service.get(id).await?)))
}

pub async fn list(
    State(state): State<AppState>,
    Query(q): Query<RestaurantQuery>,
    Query(p): Query<PaginationParams>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.menu_service.list_by_restaurant(q.restaurant_id, &p).await?
    )))
}

pub async fn set_available(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(dto): Json<SetAvailableDto>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.menu_service.set_available(id, dto.available).await?
    )))
}

pub async fn update_price(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdatePriceDto>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.menu_service.update_price(id, dto.price).await?
    )))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<()> {
    state.menu_service.delete(id).await?;
    Ok(())
}