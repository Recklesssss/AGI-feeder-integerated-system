use axum::{Router, routing::{get, post, put, delete}};
use axum::extract::FromRef;
use crate::AppState;
use super::handler;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    AppState: FromRef<S>,
{
    Router::new()
    .route("/", post(handler::record))
    .route("/:inventory_item_id/history", get(handler::history))
    .route("/waste-report", get(handler::waste_report))
}