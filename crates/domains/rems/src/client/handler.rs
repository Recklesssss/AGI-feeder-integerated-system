use axum::{extract::{State, Path, Query}, Json};
use std::sync::Arc;
use uuid::Uuid;
use serde::Deserialize;

use cores::AppResult;
use shared::pagination::PaginationParams;
use super::dto::*;
use super::service::ClientService;

#[derive(Debug, Deserialize)]
pub struct ListClientsQuery {
    pub org_id:   Uuid,
    #[serde(default = "default_page")]
    pub page:     u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

#[derive(Debug, Deserialize)]
pub struct OrgQuery { pub org_id: Uuid }

fn default_page() -> u32 { 1 }
fn default_per_page() -> u32 { 20 }

pub async fn create(
    State(svc): State<Arc<ClientService>>,
    Json(dto): Json<CreateClientDto>,
) -> AppResult<Json<ClientResponseDto>> {
    let client = svc.create(
        dto.org_id, &dto.name,
        dto.email.as_deref(), dto.phone.as_deref(),
        dto.client_type, dto.source.as_deref(),
    ).await?;
    Ok(Json(client.into()))
}

pub async fn get(
    State(svc): State<Arc<ClientService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<ClientResponseDto>> {
    let c = svc.get(id, q.org_id).await?;
    Ok(Json(c.into()))
}

/// Merged org + pagination into one struct — Axum only allows one extractor of each kind.
pub async fn list(
    State(svc): State<Arc<ClientService>>,
    Query(q): Query<ListClientsQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let params = PaginationParams { page: q.page, per_page: q.per_page };
    let res = svc.list(q.org_id, &params).await?;
    Ok(Json(serde_json::json!(res)))
}

pub async fn delete(
    State(svc): State<Arc<ClientService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<()> {
    svc.delete(id, q.org_id).await
}