use axum::{
    extract::{Path, Query, State},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use serde::Deserialize;

use cores::{AppError, AppResult};
use crate::service::AuditService;
use crate::model::AuditLog;

// DTO for query parameters based securely on the organization schema.
#[derive(Deserialize)]
pub struct AuditOrgQuery {
    pub organization_id: Uuid,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Deserialize)]
pub struct AuditEntityQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Fetch all audit logs for an Organization.
pub async fn get_org_audits(
    State(service): State<Arc<AuditService>>,
    Query(q): Query<AuditOrgQuery>,
) -> AppResult<Json<Vec<AuditLog>>> {
    // Limits default strictly to 50 if missing 
    let limit = q.limit.unwrap_or(50);
    let offset = q.offset.unwrap_or(0);

    let logs = service.find_by_org(q.organization_id, limit, offset).await?;
    Ok(Json(logs))
}

/// Fetch specific entity lifecycle audits explicitly via its ID mappings.
pub async fn get_entity_audits(
    State(service): State<Arc<AuditService>>,
    Path((entity_type, entity_id)): Path<(String, Uuid)>,
) -> AppResult<Json<Vec<AuditLog>>> {
    let logs = service.find_by_entity(&entity_type, entity_id).await?;
    Ok(Json(logs))
}
