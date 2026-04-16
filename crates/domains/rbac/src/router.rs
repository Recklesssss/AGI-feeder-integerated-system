use axum::{Router, routing::get};
use axum::extract::FromRef;
use std::sync::Arc;
use super::handler;
use super::service::RbacService;

/// RBAC admin routes — mount under /api/v1/admin/rbac.
/// These endpoints should be protected by superadmin permission check (T-02).
pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<RbacService>: FromRef<S>,
{
    Router::new()
        .route("/users/:id/permissions", get(handler::get_user_permissions))
}
