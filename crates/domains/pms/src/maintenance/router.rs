use axum::{
    Router,
    routing::{get, post, delete},
};
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
    .route("/:id", get(handler::get))
    .route("/:id/assign", put(handler::assign))
    .route("/:id/resolve", put(handler::resolve))
    .route("/:id/close", put(handler::close))
        
}