use axum::{extract::{State, Path, Query}, Json};
use std::sync::Arc;
use uuid::Uuid;
use serde::Deserialize;
use rust_decimal::Decimal;

use cores::AppResult;
use shared::pagination::PaginationParams;
use super::model::ListingType;
use super::service::ListingService;

// ── Shared query DTOs ──────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct OrgQuery { pub org_id: Uuid }

#[derive(Debug, Deserialize)]
pub struct ListListingsQuery {
    pub org_id:   Uuid,
    #[serde(default = "default_page")]
    pub page:     u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 { 1 }
fn default_per_page() -> u32 { 20 }

// ── Request DTO ────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreateListingDto {
    pub org_id:       Uuid,
    pub asset_id:     Uuid,
    pub title:        String,
    pub description:  Option<String>,
    pub price:        Decimal,
    pub listing_type: ListingType,
}

// ── Handlers ───────────────────────────────────────────────────────────────

pub async fn create(
    State(svc): State<Arc<ListingService>>,
    Json(dto): Json<CreateListingDto>,
) -> AppResult<Json<serde_json::Value>> {
    let listing = svc.create(
        dto.org_id, dto.asset_id, &dto.title,
        dto.description.as_deref(), dto.price, dto.listing_type,
    ).await?;
    Ok(Json(serde_json::json!(listing)))
}

pub async fn get(
    State(svc): State<Arc<ListingService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let l = svc.get(id, q.org_id).await?;
    Ok(Json(serde_json::json!(l)))
}

/// Merged org + pagination into one struct (Axum: one Query<T> per handler).
pub async fn list(
    State(svc): State<Arc<ListingService>>,
    Query(q): Query<ListListingsQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let params = PaginationParams { page: q.page, per_page: q.per_page };
    let res = svc.list(q.org_id, &params).await?;
    Ok(Json(serde_json::json!(res)))
}

pub async fn activate(
    State(svc): State<Arc<ListingService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let l = svc.activate(id, q.org_id).await?;
    Ok(Json(serde_json::json!(l)))
}

pub async fn mark_sold(
    State(svc): State<Arc<ListingService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let l = svc.mark_sold(id, q.org_id).await?;
    Ok(Json(serde_json::json!(l)))
}

pub async fn cancel(
    State(svc): State<Arc<ListingService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let l = svc.cancel(id, q.org_id).await?;
    Ok(Json(serde_json::json!(l)))
}