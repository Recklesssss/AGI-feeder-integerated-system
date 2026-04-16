use axum::{routing::get, Router};
use axum::extract::FromRef;
use std::sync::Arc;
use super::handler;
use crate::service::UserService;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<UserService>: FromRef<S>,
{
    Router::new()
        // Collection + registration
        .route("/",             get(handler::list_users_handler).post(handler::register_handler))
        // Email lookup (static segment — matched before /:id)
        .route("/by-email",     get(handler::get_by_email_handler))
        // Single resource
        .route("/:id",          get(handler::get_by_id_handler).delete(handler::delete_user_handler))
        // Sub-resource mutations
        .route("/:id/email",    axum::routing::put(handler::update_email_handler))
        .route("/:id/password", axum::routing::put(handler::change_password_handler))
        .route("/:id/lock",     axum::routing::put(handler::lock_user_handler))
        .route("/:id/unlock",   axum::routing::put(handler::unlock_user_handler))
        .route("/:id/suspend",  axum::routing::put(handler::suspend_user_handler))
}
