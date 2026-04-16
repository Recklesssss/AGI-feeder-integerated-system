use axum::{Router, routing::{get, post, put}};
use axum::extract::FromRef;
use std::sync::Arc;
use super::service::DealService;
use super::handler;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<DealService>: FromRef<S>,
{
    Router::new()
    .route("/", post(handler::create).get(handler::list))
    .route("/:id", get(handler::get))
    .route("/:id/advance", put(handler::advance_stage))
    .route("/agent/:agent_id", get(handler::agent_pipeline))
}