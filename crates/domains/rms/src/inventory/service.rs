use std::sync::Arc;
use rust_decimal::Decimal;
use uuid::Uuid;
use cores::{AppError, AppResult};
use super::{model::InventoryItem, repository::InventoryRepository};

pub struct InventoryService {
    repo: Arc<dyn InventoryRepository>,
}

impl InventoryService {
    pub fn new(repo: Arc<dyn InventoryRepository>) -> Self { Self { repo } }

    pub async fn create(&self, restaurant_id: Uuid, name: &str, unit: &str, reorder_level: Decimal, cost_per_unit: Decimal) -> AppResult<InventoryItem> {
        if name.trim().is_empty()  { return Err(AppError::Validation("Item name required".into())); }
        if cost_per_unit < Decimal::ZERO { return Err(AppError::Validation("Cost cannot be negative".into())); }
        self.repo.create(restaurant_id, name, unit, reorder_level, cost_per_unit).await
    }

    pub async fn get(&self, id: Uuid) -> AppResult<InventoryItem> {
        self.repo.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound(format!("Inventory item {id} not found")))
    }

    pub async fn list_by_restaurant(&self, restaurant_id: Uuid) -> AppResult<Vec<InventoryItem>> {
        self.repo.find_by_restaurant(restaurant_id).await
    }

    pub async fn low_stock_alerts(&self, restaurant_id: Uuid) -> AppResult<Vec<InventoryItem>> {
        self.repo.find_low_stock(restaurant_id).await
    }

    /// Receive stock (+delta) or consume stock (negative delta).
    pub async fn adjust(&self, id: Uuid, delta: Decimal) -> AppResult<InventoryItem> {
        let item = self.get(id).await?;
        let new_qty = item.quantity + delta;
        if new_qty < Decimal::ZERO {
            return Err(AppError::UnprocessableEntity(
                format!("Insufficient stock: have {}, need {}", item.quantity, delta.abs())
            ));
        }
        self.repo.adjust_quantity(id, delta).await
    }
}
