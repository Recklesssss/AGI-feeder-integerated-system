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
        .route("/", post(handler::open_order).get(handler::list))
        .route("/:id", get(handler::get))
        .route("/:id/close", put(handler::close_order))
        .route("/:id/cancel", put(handler::cancel))
        .route("/revenue/daily", get(handler::daily_revenue))
}