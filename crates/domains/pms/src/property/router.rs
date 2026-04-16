use axum::{
    Router,
    routing::{get, post},
};
use axum::extract::FromRef;

use std::sync::Arc;
use super::service::PropertyService;
use super::handler;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<PropertyService>: FromRef<S>,
{
    Router::new()
    .route("/", post(handler::create).get(handler::list))
    .route("/:id", get(handler::get).delete(handler::delete))
        
}