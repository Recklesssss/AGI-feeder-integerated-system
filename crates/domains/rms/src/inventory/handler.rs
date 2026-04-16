use axum::{extract::{State, Path, Query}, Json};
use uuid::Uuid;
use crate::AppState;
use core_lib::AppResult;
use super::dto::*;

pub async fn create(
    State(state): State<AppState>,
    Json(dto): Json<CreateInventoryDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        state.inventory_service.create(
            dto.restaurant_id,
            &dto.name,
            &dto.unit,
            dto.reorder_level,
            dto.cost_per_unit,
        ).await?
    )))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        state.inventory_service.get(id).await?
    )))
}

pub async fn list_by_restaurant(
    State(state): State<AppState>,
    Query(q): Query<RestaurantQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        state.inventory_service.list_by_restaurant(q.restaurant_id).await?
    )))
}

pub async fn low_stock(
    State(state): State<AppState>,
    Query(q): Query<RestaurantQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        state.inventory_service.low_stock_alerts(q.restaurant_id).await?
    )))
}

pub async fn adjust(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(dto): Json<AdjustDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        state.inventory_service.adjust(id, dto.delta).await?
    )))
}