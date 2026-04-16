use axum::extract::Query;
use axum::{extract::{State, Path, }, Json};
use uuid::Uuid;
use std::sync::Arc;
use super::service::OrderService;
use cores::AppResult;
use shared::pagination::PaginationParams;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct OpenOrderDto { pub org_id: Uuid, pub restaurant_id: Uuid, pub table_number: Option<String>, pub notes: Option<String>, pub opened_by: Option<Uuid>, pub items: Vec<crate::order::model::NewOrderItem> }
#[derive(Deserialize)]
pub struct OrgQuery { pub org_id: Uuid }
#[derive(Deserialize)]
pub struct ListOrderQuery { pub org_id: Uuid, pub restaurant_id: Option<Uuid>, pub limit: Option<i64>, pub offset: Option<i64> }
#[derive(Deserialize)]
pub struct RevenueQuery { pub org_id: Uuid, pub restaurant_id: Uuid, pub date: chrono::NaiveDate }
#[derive(Deserialize)]
pub struct RestaurantQuery { pub restaurant_id: Uuid }
#[derive(Deserialize)]
pub struct CreateOrderDto { pub org_id: Uuid, pub restaurant_id: Uuid, pub table_number: Option<String>, pub notes: Option<String> }
#[derive(Deserialize)]
pub struct CloseOrderDto { pub tax_rate: rust_decimal::Decimal, pub service_charge_rate: rust_decimal::Decimal, pub discount: rust_decimal::Decimal, pub payment_method: String, pub org_id: Uuid, pub closed_by: Option<Uuid> }

pub async fn open_order(
    State(svc): State<Arc<OrderService>>,
    Json(dto): Json<OpenOrderDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.open_order(
            dto.org_id,
            dto.restaurant_id,
            dto.table_number.as_deref(),
            dto.opened_by,
            dto.items,
            dto.notes.as_deref(),
        ).await?
    )))
}

pub async fn close_order(
    State(svc): State<Arc<OrderService>>,
    Path(id): Path<Uuid>,
    Json(dto): Json<CloseOrderDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.close_order(
            id,
            dto.org_id,
            dto.tax_rate,
            dto.service_charge_rate,
            dto.discount,
            &dto.payment_method,
            dto.closed_by,
        ).await?
    )))
}

pub async fn cancel(
    State(svc): State<Arc<OrderService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.cancel(id, q.org_id).await?
    )))
}

pub async fn get(
    State(svc): State<Arc<OrderService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.get(id, q.org_id).await?
    )))
}

pub async fn list(
    State(svc): State<Arc<OrderService>>,
    Query(q): Query<ListOrderQuery>,
    Query(p): Query<PaginationParams>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.list(q.org_id, q.restaurant_id, &p).await?
    )))
}

pub async fn daily_revenue(
    State(svc): State<Arc<OrderService>>,
    Query(q): Query<RevenueQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.daily_revenue(q.org_id, q.restaurant_id, q.date).await?
    )))
}


