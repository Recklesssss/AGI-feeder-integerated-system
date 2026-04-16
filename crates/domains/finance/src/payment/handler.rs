use axum::{extract::{State, Path, Query}, Json};
use std::sync::Arc;
use serde::Deserialize;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::NaiveDate;

use cores::AppResult;
use super::model::PaymentMethod;
use super::service::PaymentService;

// ── Request DTOs ───────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct RecordPaymentDto {
    pub invoice_id: Uuid,
    pub amount:     Decimal,
    pub method:     String,  // "cash" | "bank_transfer" | "card" | "mobile"
    pub paid_at:    Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct InvoiceQuery { pub invoice_id: Uuid }

// ── Handlers ───────────────────────────────────────────────────────────────

pub async fn record(
    State(svc): State<Arc<PaymentService>>,
    Json(dto): Json<RecordPaymentDto>,
) -> AppResult<Json<serde_json::Value>> {
    let method = PaymentMethod::from_str(&dto.method);
    let payment = svc.record(dto.invoice_id, dto.amount, method, dto.paid_at).await?;
    Ok(Json(serde_json::json!(payment)))
}

pub async fn list_for_invoice(
    State(svc): State<Arc<PaymentService>>,
    Query(q): Query<InvoiceQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let payments = svc.payments_for_invoice(q.invoice_id).await?;
    Ok(Json(serde_json::json!(payments)))
}

pub async fn total_paid(
    State(svc): State<Arc<PaymentService>>,
    Path(invoice_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let total = svc.total_paid(invoice_id).await?;
    Ok(Json(serde_json::json!({ "invoice_id": invoice_id, "total_paid": total })))
}