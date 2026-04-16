use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use cores::AppResult;
use super::repository::AssetRepository;
use super::model::{Asset, AssetStatus, AssetType};

pub struct AssetService {
    repo: Arc<dyn AssetRepository>,
}

impl AssetService {
    pub fn new(repo: Arc<dyn AssetRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_asset(
        &self,
        org_id: Uuid,
        asset_type: AssetType,
        name: String,
    ) -> AppResult<Asset> {
        let asset = Asset {
            id:              Uuid::new_v4(),
            organization_id: org_id,
            asset_type,
            name,
            status:          AssetStatus::Active,
            created_at:      Utc::now(),
            updated_at:      Utc::now(),
            deleted_at:      None,
        };
        self.repo.create_asset(asset).await
    }

    pub async fn list_assets(&self, org_id: Uuid) -> AppResult<Vec<Asset>> {
        self.repo.list_assets(org_id).await
    }

    pub async fn update_status(&self, asset_id: Uuid, status: AssetStatus) -> AppResult<()> {
        let status_str = match status {
            AssetStatus::Active   => "ACTIVE",
            AssetStatus::Inactive => "INACTIVE",
            AssetStatus::Archived => "ARCHIVED",
        };
        self.repo.update_status(asset_id, status_str.to_owned()).await
    }
}