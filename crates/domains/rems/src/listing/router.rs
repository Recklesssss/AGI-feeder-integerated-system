use axum::{Router, routing::{get, post, put}};
use axum::extract::FromRef;
use std::sync::Arc;
use super::handler;
use super::service::ListingService;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<ListingService>: FromRef<S>,
{
    Router::new()
        .route("/",             post(handler::create).get(handler::list))
        .route("/:id",          get(handler::get))
        .route("/:id/activate", put(handler::activate))
        .route("/:id/sold",     put(handler::mark_sold))
        .route("/:id/cancel",   put(handler::cancel))
}