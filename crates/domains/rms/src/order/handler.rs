use axum::{extract::{State, Path, Query}, Json};
use uuid::Uuid;
use crate::AppState;
use core_lib::AppResult;
use shared_lib::pagination::PaginationParams;
use super::dto::*;

pub async fn open_order(
    State(state): State<AppState>,
    Json(dto): Json<OpenOrderDto>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.order_service.open_order(
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
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(dto): Json<CloseOrderDto>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.order_service.close_order(
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
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.order_service.cancel(id, q.org_id).await?
    )))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.order_service.get(id, q.org_id).await?
    )))
}

pub async fn list(
    State(state): State<AppState>,
    Query(q): Query<ListOrderQuery>,
    Query(p): Query<PaginationParams>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.order_service.list(q.org_id, q.restaurant_id, &p).await?
    )))
}

pub async fn daily_revenue(
    State(state): State<AppState>,
    Query(q): Query<RevenueQuery>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.order_service.daily_revenue(q.org_id, q.restaurant_id, q.date).await?
    )))
}