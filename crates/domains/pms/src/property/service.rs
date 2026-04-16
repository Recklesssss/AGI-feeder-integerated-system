use std::sync::Arc;
use uuid::Uuid;
use cores::{AppError, AppResult};
use shared::pagination::{PaginationParams, PaginatedResponse};
use super::{model::Property, repository::PropertyRepository};

pub struct PropertyService {
    repo: Arc<dyn PropertyRepository>,
}

impl PropertyService {
    pub fn new(repo: Arc<dyn PropertyRepository>) -> Self { Self { repo } }

    pub async fn create(&self, org_id: Uuid, asset_id: Uuid, address: &str, city: Option<&str>, country: Option<&str>) -> AppResult<Property> {
        if address.trim().is_empty() {
            return Err(AppError::Validation("Address is required".into()));
        }
        self.repo.create(org_id, asset_id, address, city, country).await
    }

    pub async fn get(&self, id: Uuid, org_id: Uuid) -> AppResult<Property> {
        self.repo.find_by_id(id, org_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Property {id} not found")))
    }

    pub async fn list(&self, org_id: Uuid, params: &PaginationParams) -> AppResult<PaginatedResponse<Property>> {
        let (props, total) = self.repo.find_all(org_id, params.limit(), params.offset()).await?;
        Ok(PaginatedResponse::new(props, total, params))
    }

    pub async fn delete(&self, id: Uuid, org_id: Uuid) -> AppResult<()> {
        let _ = self.get(id, org_id).await?;
        self.repo.soft_delete(id, org_id).await
    }
}
