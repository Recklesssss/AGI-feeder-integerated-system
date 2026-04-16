use axum::extract::Query;
use axum::{extract::{State, Path, }, Json};
use uuid::Uuid;
use std::sync::Arc;
use super::service::RestaurantService;
use cores::AppResult;
use shared::pagination::PaginationParams;
use super::dto::*;

pub async fn create(
    State(svc): State<Arc<RestaurantService>>,
    Json(dto): Json<CreateRestaurantDto>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.create(
            dto.org_id,
            dto.asset_id,
            &dto.name,
            dto.address.as_deref(),
        ).await?
    )))
}

pub async fn get(
    State(svc): State<Arc<RestaurantService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.get(id, q.org_id).await?
    )))
}

pub async fn list(
    State(svc): State<Arc<RestaurantService>>,
    Query(q): Query<OrgQuery>,
    Query(p): Query<PaginationParams>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!(
        svc.list(q.org_id, &p).await?
    )))
}

pub async fn delete(
    State(svc): State<Arc<RestaurantService>>,
    Path(id): Path<Uuid>,
    Query(q): Query<OrgQuery>,
) -> AppResult<()> {
    svc.delete(id, q.org_id).await?;
    Ok(())
}


