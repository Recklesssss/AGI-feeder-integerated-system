use std::sync::Arc;
use rust_decimal::Decimal;
use uuid::Uuid;
use core_lib::{AppError, AppResult};
use shared_lib::pagination::{PaginationParams, PaginatedResponse};
use super::{model::MenuItem, repository::MenuRepository};

pub struct MenuService<R: MenuRepository> {
    repo: Arc<R>,
}

impl<R: MenuRepository> MenuService<R> {
    pub fn new(repo: Arc<R>) -> Self { Self { repo } }

    pub async fn create(&self, restaurant_id: Uuid, name: &str, description: Option<&str>, category: Option<&str>, price: Decimal, cost: Decimal) -> AppResult<MenuItem> {
        if name.trim().is_empty() { return Err(AppError::Validation("Item name is required".into())); }
        if price < Decimal::ZERO  { return Err(AppError::Validation("Price cannot be negative".into())); }
        if cost  < Decimal::ZERO  { return Err(AppError::Validation("Cost cannot be negative".into())); }
        self.repo.create(restaurant_id, name, description, category, price, cost).await
    }

    pub async fn get(&self, id: Uuid) -> AppResult<MenuItem> {
        self.repo.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound(format!("Menu item {id} not found")))
    }

    pub async fn list_by_restaurant(&self, restaurant_id: Uuid, params: &PaginationParams) -> AppResult<PaginatedResponse<MenuItem>> {
        let (items, total) = self.repo.find_by_restaurant(restaurant_id, params.limit(), params.offset()).await?;
        Ok(PaginatedResponse::new(items, total, params))
    }

    pub async fn set_available(&self, id: Uuid, available: bool) -> AppResult<MenuItem> {
        let _ = self.get(id).await?;
        self.repo.update_availability(id, available).await
    }

    pub async fn update_price(&self, id: Uuid, price: Decimal) -> AppResult<MenuItem> {
        if price < Decimal::ZERO { return Err(AppError::Validation("Price cannot be negative".into())); }
        let _ = self.get(id).await?;
        self.repo.update_price(id, price).await
    }

    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        let _ = self.get(id).await?;
        self.repo.soft_delete(id).await
    }
}
