//! In-memory mock for AuditRepository — verifies audit logging without Postgres.

use std::sync::Mutex;
use async_trait::async_trait;
use uuid::Uuid;
use chrono::Utc;

use cores::AppResult;
use audit::model::{AuditLog, AuditAction};
use audit::repository::AuditRepository;

/// Captures all audit log entries in memory for assertion.
pub struct MockAuditRepository {
    pub entries: Mutex<Vec<AuditLog>>,
}

impl Default for MockAuditRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl MockAuditRepository {
    pub fn new() -> Self {
        Self { entries: Mutex::new(Vec::new()) }
    }

    pub fn recorded_count(&self) -> usize {
        self.entries.lock().unwrap().len()
    }

    pub fn last_entry(&self) -> Option<AuditLog> {
        self.entries.lock().unwrap().last().cloned()
    }
}

#[async_trait]
impl AuditRepository for MockAuditRepository {
    async fn record(
        &self,
        organization_id: Uuid,
        user_id: Uuid,
        action: AuditAction,
        entity: &str,
        entity_id: Uuid,
        metadata: serde_json::Value,
    ) -> AppResult<AuditLog> {
        let log = AuditLog {
            id: Uuid::new_v4(),
            organization_id,
            user_id,
            action,
            entity: entity.to_owned(),
            entity_id,
            metadata,
            created_at: Utc::now(),
        };
        self.entries.lock().unwrap().push(log.clone());
        Ok(log)
    }

    async fn find_by_org(
        &self,
        org_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<AuditLog>> {
        let entries = self.entries.lock().unwrap();
        let result: Vec<AuditLog> = entries
            .iter()
            .filter(|e| e.organization_id == org_id)
            .skip(offset as usize)
            .take(limit as usize)
            .cloned()
            .collect();
        Ok(result)
    }

    async fn find_by_entity(
        &self,
        entity: &str,
        entity_id: Uuid,
    ) -> AppResult<Vec<AuditLog>> {
        let entries = self.entries.lock().unwrap();
        let result: Vec<AuditLog> = entries
            .iter()
            .filter(|e| e.entity == entity && e.entity_id == entity_id)
            .cloned()
            .collect();
        Ok(result)
    }
}
