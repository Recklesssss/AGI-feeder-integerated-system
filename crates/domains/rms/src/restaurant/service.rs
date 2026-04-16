use std::sync::Arc;
use uuid::Uuid;
use core_lib::{AppError, AppResult};
use shared_lib::pagination::{PaginationParams, PaginatedResponse};
use super::{model::Restaurant, repository::RestaurantRepository};

pub struct RestaurantService<R: RestaurantRepository> {
    repo: Arc<R>,
}

impl<R: RestaurantRepository> RestaurantService<R> {
    pub fn new(repo: Arc<R>) -> Self { Self { repo } }

    pub async fn create(&self, org_id: Uuid, asset_id: Uuid, name: &str, address: Option<&str>) -> AppResult<Restaurant> {
        if name.trim().is_empty() { return Err(AppError::Validation("Restaurant name is required".into())); }
        self.repo.create(org_id, asset_id, name, address).await
    }

    pub async fn get(&self, id: Uuid, org_id: Uuid) -> AppResult<Restaurant> {
        self.repo.find_by_id(id, org_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Restaurant {id} not found")))
    }

    pub async fn list(&self, org_id: Uuid, params: &PaginationParams) -> AppResult<PaginatedResponse<Restaurant>> {
        let (items, total) = self.repo.find_all(org_id, params.limit(), params.offset()).await?;
        Ok(PaginatedResponse::new(items, total, params))
    }

    pub async fn delete(&self, id: Uuid, org_id: Uuid) -> AppResult<()> {
        let _ = self.get(id, org_id).await?;
        self.repo.soft_delete(id, org_id).await
    }
}
