use async_trait::async_trait;
use uuid::Uuid;
use cores::AppResult;
use super::model::{AuditLog, AuditAction};

/// Repository trait for audit logs.
#[async_trait]
pub trait AuditRepository: Send + Sync {
    /// Record a new immutable audit log entry.
    async fn record(
        &self,
        organization_id: Uuid,
        user_id: Uuid,
        action: AuditAction,
        entity: &str,
        entity_id: Uuid,
        metadata: serde_json::Value,
    ) -> AppResult<AuditLog>;

    /// Retrieve audit logs for a specific organization, ordered by newest first.
    async fn find_by_org(
        &self,
        org_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<AuditLog>>;

    /// Retrieve audit logs for a specific entity instance.
    async fn find_by_entity(
        &self,
        entity: &str,
        entity_id: Uuid,
    ) -> AppResult<Vec<AuditLog>>;
}
