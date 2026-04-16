use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;
use cores::AppResult;
use super::model::MenuItem;

#[async_trait]
pub trait MenuRepository: Send + Sync + 'static {
    async fn create(&self, restaurant_id: Uuid, name: &str, description: Option<&str>, category: Option<&str>, price: Decimal, cost: Decimal) -> AppResult<MenuItem>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<MenuItem>>;
    async fn find_by_restaurant(&self, restaurant_id: Uuid, limit: i64, offset: i64) -> AppResult<(Vec<MenuItem>, i64)>;
    async fn update_availability(&self, id: Uuid, available: bool) -> AppResult<MenuItem>;
    async fn update_price(&self, id: Uuid, price: Decimal) -> AppResult<MenuItem>;
    async fn soft_delete(&self, id: Uuid) -> AppResult<()>;
}
