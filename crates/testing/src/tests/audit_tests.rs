use std::sync::Arc;

use audit::model::AuditAction;
use audit::service::AuditService;
use crate::mocks::audit::MockAuditRepository;
use crate::mocks::{test_org_id, test_user_id, random_id};

#[tokio::test]
async fn audit_record_creates_entry() {
    let repo = Arc::new(MockAuditRepository::new());
    let svc = AuditService::new(repo.clone());

    let entity_id = random_id();
    let result = svc.record(
        test_org_id(),
        test_user_id(),
        AuditAction::Create,
        "asset",
        entity_id,
        serde_json::json!({"name": "Test Property"}),
    ).await;

    assert!(result.is_ok());
    let log = result.unwrap();
    assert_eq!(log.entity, "asset");
    assert_eq!(log.entity_id, entity_id);
    assert_eq!(log.action, AuditAction::Create);
    assert_eq!(repo.recorded_count(), 1);
}

#[tokio::test]
async fn audit_find_by_org_filters_correctly() {
    let repo = Arc::new(MockAuditRepository::new());
    let svc = AuditService::new(repo.clone());

    let org = test_org_id();
    let other_org = random_id();

    // Record 2 entries for our org, 1 for a different org
    svc.record(org, test_user_id(), AuditAction::Create, "user", random_id(), serde_json::json!({})).await.unwrap();
    svc.record(org, test_user_id(), AuditAction::Update, "user", random_id(), serde_json::json!({})).await.unwrap();
    svc.record(other_org, test_user_id(), AuditAction::Delete, "user", random_id(), serde_json::json!({})).await.unwrap();

    let results = svc.find_by_org(org, 100, 0).await.unwrap();
    assert_eq!(results.len(), 2);
}

#[tokio::test]
async fn audit_find_by_entity_returns_history() {
    let repo = Arc::new(MockAuditRepository::new());
    let svc = AuditService::new(repo.clone());

    let entity_id = random_id();

    svc.record(test_org_id(), test_user_id(), AuditAction::Create, "invoice", entity_id, serde_json::json!({"total": 500})).await.unwrap();
    svc.record(test_org_id(), test_user_id(), AuditAction::Update, "invoice", entity_id, serde_json::json!({"status": "paid"})).await.unwrap();
    svc.record(test_org_id(), test_user_id(), AuditAction::Create, "invoice", random_id(), serde_json::json!({})).await.unwrap();

    let history = svc.find_by_entity("invoice", entity_id).await.unwrap();
    assert_eq!(history.len(), 2);
}
