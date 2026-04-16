use axum::extract::Query;
use axum::{extract::{State, Path, }, Json};
use std::sync::Arc;
use serde::Deserialize;
use uuid::Uuid;

use cores::AppResult;
use super::dto::CreateInvoiceDto;
use super::service::InvoiceService;

#[derive(Debug, Deserialize)]
pub struct OrgQuery { pub org_id: Uuid }

#[derive(Debug, Deserialize)]
pub struct ListInvoicesQuery {
    pub org_id:   Uuid,
    #[serde(default = "default_limit")]
    pub limit:    i64,
    #[serde(default = "default_offset")]
    pub offset:   i64,
}

fn default_limit()  -> i64 { 20 }
fn default_offset() -> i64 { 0 }

pub async fn create(
    State(svc): State<Arc<InvoiceService>>,
    Json(dto): Json<CreateInvoiceDto>,
) -> AppResult<Json<serde_json::Value>> {
    let inv = svc.create(dto.org_id, dto.asset_id, dto.total, dto.issued_at).await?;
    Ok(Json(serde_json::json!(inv)))
}

pub async fn get(
    State(svc): State<Arc<InvoiceService>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let inv = svc.get(id).await?;
    Ok(Json(serde_json::json!(inv)))
}

pub async fn list(
    State(svc): State<Arc<InvoiceService>>,
    Query(q): Query<ListInvoicesQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let (items, total) = svc.list(q.org_id, q.limit, q.offset).await?;
    Ok(Json(serde_json::json!({ "items": items, "total": total })))
}

pub async fn issue(
    State(svc): State<Arc<InvoiceService>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let inv = svc.issue(id).await?;
    Ok(Json(serde_json::json!(inv)))
}

pub async fn mark_paid(
    State(svc): State<Arc<InvoiceService>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let inv = svc.mark_paid(id).await?;
    Ok(Json(serde_json::json!(inv)))
}

pub async fn cancel(
    State(svc): State<Arc<InvoiceService>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let inv = svc.cancel(id).await?;
    Ok(Json(serde_json::json!(inv)))
}