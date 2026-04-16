use axum::{Router, routing::{get, post}};
use axum::extract::FromRef;
use std::sync::Arc;
use super::handler;
use super::service::ClientService;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<ClientService>: FromRef<S>,
{
    Router::new()
        .route("/", post(handler::create).get(handler::list))
        .route("/:id", get(handler::get).delete(handler::delete))
}