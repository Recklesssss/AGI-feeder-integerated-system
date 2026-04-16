use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;
use core_lib::AppResult;
use super::model::{StockMovement, MovementType};

#[async_trait]
pub trait StockRepository: Send + Sync + 'static {
    async fn record(
        &self,
        inventory_item_id: Uuid,
        quantity: Decimal,
        movement_type: MovementType,
        reference_type: Option<String>,
        reference_id: Option<Uuid>,
        notes: Option<String>,
        recorded_by: Option<Uuid>,
    ) -> AppResult<StockMovement>;

    async fn find_by_item(&self, inventory_item_id: Uuid, limit: i64, offset: i64) -> AppResult<Vec<StockMovement>>;
    async fn waste_total(&self, restaurant_id: Uuid, from: chrono::NaiveDate, to: chrono::NaiveDate) -> AppResult<Decimal>;
}
