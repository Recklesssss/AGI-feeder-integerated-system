use axum::extract::Query;
use axum::{extract::{State, Path, }, Json};
use uuid::Uuid;
use std::sync::Arc;
use super::service::StockService;
use cores::AppResult;
use super::dto::*;

pub async fn record(
    State(svc): State<Arc<StockService>>,
    Json(dto): Json<RecordStockDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.record_movement(
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
    State(svc): State<Arc<StockService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<HistoryQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.history(id, q.limit, q.offset).await?
    )))
}

pub async fn waste_report(
    State(svc): State<Arc<StockService>>,
    Query(q): Query<WasteQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.waste_report(q.restaurant_id, q.from, q.to).await?
    )))
}


