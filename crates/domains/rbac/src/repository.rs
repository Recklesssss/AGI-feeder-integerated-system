use async_trait::async_trait;
use uuid::Uuid;
use cores::AppResult;

#[async_trait]
pub trait RbacRepository: Send + Sync {
    async fn get_user_permission_keys(
        &self,
        user_id: Uuid,
    ) -> AppResult<Vec<String>>;
}