use async_trait::async_trait;
use uuid::Uuid;
use core_lib::AppResult;
use super::model::Tenant;

#[async_trait]
pub trait TenantRepository: Send + Sync + 'static {
    async fn create(&self, org_id: Uuid, name: &str, email: Option<&str>, phone: Option<&str>, national_id: Option<&str>) -> AppResult<Tenant>;
    async fn find_by_id(&self, id: Uuid, org_id: Uuid) -> AppResult<Option<Tenant>>;
    async fn find_all(&self, org_id: Uuid, limit: i64, offset: i64) -> AppResult<(Vec<Tenant>, i64)>;
    async fn soft_delete(&self, id: Uuid, org_id: Uuid) -> AppResult<()>;
}
