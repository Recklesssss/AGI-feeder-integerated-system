use async_trait::async_trait;
use uuid::Uuid;
use core_lib::AppResult;
use super::model::Restaurant;

#[async_trait]
pub trait RestaurantRepository: Send + Sync + 'static {
    async fn create(&self, org_id: Uuid, asset_id: Uuid, name: &str, address: Option<&str>) -> AppResult<Restaurant>;
    async fn find_by_id(&self, id: Uuid, org_id: Uuid) -> AppResult<Option<Restaurant>>;
    async fn find_all(&self, org_id: Uuid, limit: i64, offset: i64) -> AppResult<(Vec<Restaurant>, i64)>;
    async fn soft_delete(&self, id: Uuid, org_id: Uuid) -> AppResult<()>;
}
