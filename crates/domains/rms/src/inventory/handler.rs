use axum::extract::Query;
use axum::{extract::{State, Path, }, Json};
use uuid::Uuid;
use std::sync::Arc;
use super::service::InventoryService;
use cores::AppResult;
use serde::Deserialize;
use crate::menu::handler::RestaurantQuery;

#[derive(Deserialize)]
pub struct CreateInventoryDto { pub restaurant_id: Uuid, pub name: String, pub unit: String, pub reorder_level: rust_decimal::Decimal, pub cost_per_unit: rust_decimal::Decimal }

#[derive(Deserialize)]
pub struct AdjustDto { pub delta: rust_decimal::Decimal, pub reason: Option<String> }

pub async fn create(
    State(svc): State<Arc<InventoryService>>,
    Json(dto): Json<CreateInventoryDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.create(
            dto.restaurant_id,
            &dto.name,
            &dto.unit,
            dto.reorder_level,
            dto.cost_per_unit,
        ).await?
    )))
}

pub async fn get(
    State(svc): State<Arc<InventoryService>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.get(id).await?
    )))
}

pub async fn list_by_restaurant(
    State(svc): State<Arc<InventoryService>>,
    Query(q): Query<RestaurantQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.list_by_restaurant(q.restaurant_id).await?
    )))
}

pub async fn low_stock(
    State(svc): State<Arc<InventoryService>>,
    Query(q): Query<RestaurantQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.low_stock_alerts(q.restaurant_id).await?
    )))
}

pub async fn adjust(
    State(svc): State<Arc<InventoryService>>,
    Path(id): Path<Uuid>,
    Json(dto): Json<AdjustDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.adjust(id, dto.delta).await?
    )))
}


