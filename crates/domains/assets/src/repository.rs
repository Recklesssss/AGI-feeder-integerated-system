use async_trait::async_trait;
use uuid::Uuid;
use cores::AppResult;
use super::model::Asset;

#[async_trait]
pub trait AssetRepository: Send + Sync {
    async fn create_asset(&self, asset: Asset) -> AppResult<Asset>;
    async fn get_asset_by_id(&self, id: Uuid) -> AppResult<Option<Asset>>;
    async fn list_assets(&self, org_id: Uuid) -> AppResult<Vec<Asset>>;
    async fn update_status(&self, id: Uuid, status: String) -> AppResult<()>;
}