use sqlx::{PgPool, Row};
use uuid::Uuid;
use async_trait::async_trait;

use cores::{AppError, AppResult};
use audit::{
    model::{AuditLog, AuditAction},
    repository::AuditRepository,
};

pub struct PgAuditRepository {
    pub db: PgPool,
}

fn map_audit(row: &sqlx::postgres::PgRow) -> Result<AuditLog, sqlx::Error> {
    let action_str: String = row.try_get("action")?;
    Ok(AuditLog {
        id:              row.try_get("id")?,
        organization_id: row.try_get("organization_id")?,
        user_id:         row.try_get("user_id")?,
        action:          AuditAction::from_str(&action_str),
        entity:          row.try_get("entity")?,
        entity_id:       row.try_get("entity_id")?,
        metadata:        row.try_get("metadata")?,
        created_at:      row.try_get("created_at")?,
    })
}

#[async_trait]
impl AuditRepository for PgAuditRepository {
    async fn record(
        &self,
        organization_id: Uuid,
        user_id: Uuid,
        action: AuditAction,
        entity: &str,
        entity_id: Uuid,
        metadata: serde_json::Value,
    ) -> AppResult<AuditLog> {
        let row = sqlx::query(
            "INSERT INTO audit_logs (organization_id, user_id, action, entity, entity_id, metadata)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id, organization_id, user_id, action, entity, entity_id, metadata, created_at",
        )
        .bind(organization_id)
        .bind(user_id)
        .bind(action.as_str())
        .bind(entity)
        .bind(entity_id)
        .bind(metadata)
        .fetch_one(&self.db)
        .await
        .map_err(AppError::from)?;

        map_audit(&row).map_err(AppError::from)
    }

    async fn find_by_org(
        &self,
        org_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<AuditLog>> {
        let rows = sqlx::query(
            "SELECT id, organization_id, user_id, action, entity, entity_id, metadata, created_at
             FROM audit_logs WHERE organization_id = $1
             ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(org_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db)
        .await
        .map_err(AppError::from)?;

        rows.iter()
            .map(|r| map_audit(r).map_err(AppError::from))
            .collect()
    }

    async fn find_by_entity(
        &self,
        entity: &str,
        entity_id: Uuid,
    ) -> AppResult<Vec<AuditLog>> {
        let rows = sqlx::query(
            "SELECT id, organization_id, user_id, action, entity, entity_id, metadata, created_at
             FROM audit_logs WHERE entity = $1 AND entity_id = $2
             ORDER BY created_at DESC",
        )
        .bind(entity)
        .bind(entity_id)
        .fetch_all(&self.db)
        .await
        .map_err(AppError::from)?;

        rows.iter()
            .map(|r| map_audit(r).map_err(AppError::from))
            .collect()
    }
}
