use axum::{
    Router,
    routing::{get, post, put},
};
use axum::extract::FromRef;

use std::sync::Arc;
use super::service::InventoryService;
use super::handler;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<InventoryService>: FromRef<S>,
{
    Router::new()
    .route("/", post(handler::create).get(handler::list_by_restaurant))
    .route("/:id", get(handler::get))
    .route("/:id/adjust", put(handler::adjust))
    .route("/low-stock", get(handler::low_stock))
        
}
