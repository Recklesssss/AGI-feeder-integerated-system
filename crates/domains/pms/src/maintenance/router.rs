use axum::{
    Router,
    routing::{get, post, put},
};
use axum::extract::FromRef;

use std::sync::Arc;
use super::service::MaintenanceService;
use super::handler;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<MaintenanceService>: FromRef<S>,
{
    Router::new()
    .route("/", post(handler::create).get(handler::list))
    .route("/:id", get(handler::get))
    .route("/:id/assign", put(handler::assign))
    .route("/:id/resolve", put(handler::resolve))
    .route("/:id/close", put(handler::close))
        
}