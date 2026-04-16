use axum::{Router, routing::{get, post, put}};
use axum::extract::FromRef;
use std::sync::Arc;
use super::service::MenuService;
use super::handler;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<MenuService>: FromRef<S>,
{
    Router::new()
        .route("/", post(handler::create).get(handler::list))
        .route("/:id", get(handler::get).delete(handler::delete))
        .route("/:id/availability", put(handler::set_available))
        .route("/:id/price", put(handler::update_price))
}