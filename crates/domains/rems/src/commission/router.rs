use axum::{Router, routing::{post, put}};
use axum::extract::FromRef;
use crate::AppState;
use super::handler;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    AppState: FromRef<S>,
{
    Router::new()
    .route("/", post(handler::create))
    .route("/deal/:deal_id", get(handler::for_deal))
    .route("/agent/:agent_id", get(handler::for_agent))
    .route("/:id/approve", put(handler::approve))
    .route("/:id/pay", put(handler::pay))
}