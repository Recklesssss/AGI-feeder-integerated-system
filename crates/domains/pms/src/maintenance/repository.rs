use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;
use core_lib::AppResult;
use super::model::MaintenanceRequest;

#[async_trait]
pub trait MaintenanceRepository: Send + Sync + 'static {
    async fn create(&self, org_id: Uuid, unit_id: Uuid, description: &str, priority: &str, reported_by: Option<Uuid>) -> AppResult<MaintenanceRequest>;
    async fn find_by_id(&self, id: Uuid, org_id: Uuid) -> AppResult<Option<MaintenanceRequest>>;
    async fn find_all(&self, org_id: Uuid, limit: i64, offset: i64) -> AppResult<(Vec<MaintenanceRequest>, i64)>;
    async fn assign(&self, id: Uuid, user_id: Uuid) -> AppResult<MaintenanceRequest>;
    async fn update_status(&self, id: Uuid, status: &str, actual_cost: Option<Decimal>) -> AppResult<MaintenanceRequest>;
}
