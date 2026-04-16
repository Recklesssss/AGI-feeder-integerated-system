use axum::{Router, routing::{get, post}};
use axum::extract::FromRef;
use std::sync::Arc;
use super::service::StockService;
use super::handler;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<StockService>: FromRef<S>,
{
    Router::new()
    .route("/", post(handler::record))
    .route("/:inventory_item_id/history", get(handler::history))
    .route("/waste-report", get(handler::waste_report))
}