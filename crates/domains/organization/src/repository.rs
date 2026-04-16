use async_trait::async_trait;
use uuid::Uuid;
use cores::AppResult;
use super::model::Organization;

#[async_trait]
pub trait OrgRepository: Send + Sync + 'static {
    async fn create(&self, id: Uuid, name: &str) -> AppResult<Organization>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Organization>>;
    async fn find_all(&self, limit: i64, offset: i64) -> AppResult<(Vec<Organization>, i64)>;
    async fn update_status(&self, id: Uuid, status: &str) -> AppResult<Organization>;
    async fn soft_delete(&self, id: Uuid) -> AppResult<()>;
}
