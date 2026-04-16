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
        .route("/", post(handler::create).get(handler::list))
        .route("/:id", get(handler::get).delete(handler::delete))
        .route("/:id/availability", put(handler::set_available))
        .route("/:id/price", put(handler::update_price))
}