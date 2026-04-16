use axum::{Router, routing::{get, post}};
use axum::extract::FromRef;
use std::sync::Arc;
use super::handler;
use super::service::PaymentService;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<PaymentService>: FromRef<S>,
{
    Router::new()
        .route("/",                      post(handler::record).get(handler::list_for_invoice))
        .route("/:invoice_id/total",     get(handler::total_paid))
}