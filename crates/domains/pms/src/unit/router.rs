use axum::{Router, routing::{get, post, put}};
use axum::extract::FromRef;
use std::sync::Arc;
use super::service::UnitService;
use super::handler;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<UnitService>: FromRef<S>,
{
    Router::new()
        .route("/", post(handler::create).get(handler::list_by_property))
        .route("/:id", get(handler::get))
        .route("/:id/occupied", put(handler::mark_occupied))
        .route("/:id/vacant", put(handler::mark_vacant))
        .route("/vacancy-count", get(handler::vacancy_count))
}