use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;
use cores::AppResult;
use super::model::InventoryItem;

#[async_trait]
pub trait InventoryRepository: Send + Sync + 'static {
    async fn create(&self, restaurant_id: Uuid, name: &str, unit: &str, reorder_level: Decimal, cost_per_unit: Decimal) -> AppResult<InventoryItem>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<InventoryItem>>;
    async fn find_by_restaurant(&self, restaurant_id: Uuid) -> AppResult<Vec<InventoryItem>>;
    async fn adjust_quantity(&self, id: Uuid, delta: Decimal) -> AppResult<InventoryItem>;
    async fn find_low_stock(&self, restaurant_id: Uuid) -> AppResult<Vec<InventoryItem>>;
}
