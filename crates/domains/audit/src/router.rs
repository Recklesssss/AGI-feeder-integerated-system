use axum::{
    routing::get,
    Router,
};
use crate::handler;
use crate::service::AuditService;
use std::sync::Arc;
use axum::extract::FromRef;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<AuditService>: FromRef<S>,
{
    Router::new()
        // Example: GET /api/v1/audit?organization_id=123...
        .route("/", get(handler::get_org_audits))
        // Example: GET /api/v1/audit/entities/invoice/456...
        .route("/entities/:entity_type/:entity_id", get(handler::get_entity_audits))
}
