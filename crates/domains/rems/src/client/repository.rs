use async_trait::async_trait;
use uuid::Uuid;
use cores::AppResult;
use super::model::{Client, ClientType};

#[async_trait]
pub trait ClientRepository: Send + Sync + 'static {
    async fn create(
        &self, org_id: Uuid, name: &str, email: Option<&str>,
        phone: Option<&str>, client_type: ClientType, source: Option<&str>,
    ) -> AppResult<Client>;
    async fn find_by_id(&self, id: Uuid, org_id: Uuid) -> AppResult<Option<Client>>;
    async fn find_all(&self, org_id: Uuid, limit: i64, offset: i64) -> AppResult<(Vec<Client>, i64)>;
    async fn soft_delete(&self, id: Uuid, org_id: Uuid) -> AppResult<()>;
}
