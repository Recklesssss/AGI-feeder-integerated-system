use async_trait::async_trait;
use uuid::Uuid;
use cores::AppResult;
use super::model::AuthCredentials;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> AppResult<Option<AuthCredentials>>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<AuthCredentials>>;
    async fn create_user(
        &self, id: Uuid, org_id: Uuid, email: &str, full_name: &str, password_hash: &str,
    ) -> AppResult<AuthCredentials>;
}
