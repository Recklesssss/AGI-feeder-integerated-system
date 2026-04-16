use axum::{Router, routing::{get, post, put}};
use axum::extract::FromRef;
use std::sync::Arc;
use super::handler;
use super::service::InvoiceService;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<InvoiceService>: FromRef<S>,
{
    Router::new()
        .route("/",           post(handler::create).get(handler::list))
        .route("/:id",        get(handler::get))
        .route("/:id/issue",  put(handler::issue))
        .route("/:id/paid",   put(handler::mark_paid))
        .route("/:id/cancel", put(handler::cancel))
}