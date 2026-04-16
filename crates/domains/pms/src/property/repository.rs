use async_trait::async_trait;
use uuid::Uuid;
use core_lib::AppResult;
use super::model::Property;

#[async_trait]
pub trait PropertyRepository: Send + Sync + 'static {
    async fn create(&self, org_id: Uuid, asset_id: Uuid, address: &str, city: Option<&str>, country: Option<&str>) -> AppResult<Property>;
    async fn find_by_id(&self, id: Uuid, org_id: Uuid) -> AppResult<Option<Property>>;
    async fn find_all(&self, org_id: Uuid, limit: i64, offset: i64) -> AppResult<(Vec<Property>, i64)>;
    async fn soft_delete(&self, id: Uuid, org_id: Uuid) -> AppResult<()>;
}
