use axum::{extract::{State, Path, Query}, Json};
use uuid::Uuid;
use crate::AppState;
use core_lib::AppResult;
use super::dto::*;

pub async fn record(
    State(state): State<AppState>,
    Json(dto): Json<RecordStockDto>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.stock_service.record_movement(
            dto.inventory_item_id,
            dto.quantity,
            dto.movement_type,
            dto.reference_type,
            dto.reference_id,
            dto.notes,
            dto.recorded_by,
        ).await?
    )))
}

pub async fn history(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(q): Query<HistoryQuery>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.stock_service.history(id, q.limit, q.offset).await?
    )))
}

pub async fn waste_report(
    State(state): State<AppState>,
    Query(q): Query<WasteQuery>,
) -> AppResult<Json<_>> {
    Ok(Json(serde_json::json!(
        state.stock_service.waste_report(q.restaurant_id, q.from, q.to).await?
    )))
}