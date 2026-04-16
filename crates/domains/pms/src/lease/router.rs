use axum::{
    Router,
    routing::{get, post, put},
};
use axum::extract::FromRef;

use std::sync::Arc;
use super::service::LeaseService;
use super::handler;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<LeaseService>: FromRef<S>,
{
    Router::new()
    .route("/", post(handler::create).get(handler::list))
    .route("/:id", get(handler::get))
    .route("/:id/terminate", put(handler::terminate))
    .route("/expiring", get(handler::expiring))
        
}