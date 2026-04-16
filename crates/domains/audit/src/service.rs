use std::sync::Arc;
use uuid::Uuid;

use cores::AppResult;
use super::model::{AuditAction, AuditLog};
use super::repository::AuditRepository;

pub struct AuditService {
    repo: Arc<dyn AuditRepository>,
}

impl AuditService {
    pub fn new(repo: Arc<dyn AuditRepository>) -> Self {
        Self { repo }
    }

    /// Record an audit event. Called by domain handlers after mutations.
    pub async fn record(
        &self,
        organization_id: Uuid,
        user_id: Uuid,
        action: AuditAction,
        entity: &str,
        entity_id: Uuid,
        metadata: serde_json::Value,
    ) -> AppResult<AuditLog> {
        self.repo.record(organization_id, user_id, action, entity, entity_id, metadata).await
    }

    /// Fetch audit logs for an organization (paginated).
    pub async fn find_by_org(
        &self,
        org_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<AuditLog>> {
        self.repo.find_by_org(org_id, limit, offset).await
    }

    /// Fetch the full audit trail for a specific entity instance.
    pub async fn find_by_entity(
        &self,
        entity: &str,
        entity_id: Uuid,
    ) -> AppResult<Vec<AuditLog>> {
        self.repo.find_by_entity(entity, entity_id).await
    }
}
