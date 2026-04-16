use axum::extract::Query;
use axum::{extract::{State, Path, }, Json};
use uuid::Uuid;
use std::sync::Arc;
use super::service::MenuService;
use cores::AppResult;
use shared::pagination::PaginationParams;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct RestaurantQuery { pub restaurant_id: Uuid }
#[derive(Deserialize)]
pub struct CreateMenuDto { pub restaurant_id: Uuid, pub name: String, pub description: Option<String>, pub category: Option<String>, pub price: rust_decimal::Decimal, pub cost: rust_decimal::Decimal }
#[derive(Deserialize)]
pub struct SetAvailableDto { pub available: bool }
#[derive(Deserialize)]
pub struct UpdatePriceDto { pub price: rust_decimal::Decimal }

pub async fn create(
    State(svc): State<Arc<MenuService>>,
    Json(dto): Json<CreateMenuDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.create(
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
    State(svc): State<Arc<MenuService>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(svc.get(id).await?)))
}

pub async fn list(
    State(svc): State<Arc<MenuService>>,
    Query(q): Query<RestaurantQuery>,
    Query(p): Query<PaginationParams>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.list_by_restaurant(q.restaurant_id, &p).await?
    )))
}

pub async fn set_available(
    State(svc): State<Arc<MenuService>>,
    Path(id): Path<Uuid>,
    Json(dto): Json<SetAvailableDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.set_available(id, dto.available).await?
    )))
}

pub async fn update_price(
    State(svc): State<Arc<MenuService>>,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdatePriceDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.update_price(id, dto.price).await?
    )))
}

pub async fn delete(
    State(svc): State<Arc<MenuService>>,
    Path(id): Path<Uuid>,
) -> AppResult<()> {
    svc.delete(id).await?;
    Ok(())
}


