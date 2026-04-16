use axum::{Router, routing::{get, patch}};
use axum::extract::FromRef;
use std::sync::Arc;
use super::handler;
use crate::service::OrgService;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<OrgService>: FromRef<S>,
{
    Router::new()
        .route("/",             get(handler::list_orgs).post(handler::create_org))
        .route("/:id",          get(handler::get_org))
        .route("/:id/suspend",  patch(handler::suspend_org))
        .route("/:id/activate", patch(handler::activate_org))
}