use axum::{Router, routing::post};
use axum::extract::FromRef;
use std::sync::Arc;
use super::handler;
use crate::service::AuthService;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<AuthService>: FromRef<S>,
{
    Router::new()
        .route("/login",    post(handler::login))
        .route("/register", post(handler::register))
        .route("/refresh",  post(handler::refresh))
}
