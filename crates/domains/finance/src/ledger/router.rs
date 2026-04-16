use axum::{Router, routing::{get, post}};
use axum::extract::FromRef;
use std::sync::Arc;
use super::handler;
use super::service::LedgerService;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<LedgerService>: FromRef<S>,
{
    Router::new()
        .route("/accounts",           post(handler::create_account).get(handler::list_accounts))
        .route("/accounts/:id/balance", get(handler::get_balance))
        .route("/entries",            post(handler::post_entry).get(handler::get_entries))
}