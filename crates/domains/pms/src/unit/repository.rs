use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;
use cores::AppResult;
use super::model::Unit;

#[async_trait]
pub trait UnitRepository: Send + Sync + 'static {
    async fn create(&self, org_id: Uuid, property_id: Uuid, asset_id: Uuid, unit_number: &str, floor: Option<i32>, bedrooms: Option<i32>, bathrooms: Option<i32>, area_sqm: Option<Decimal>) -> AppResult<Unit>;
    async fn find_by_id(&self, id: Uuid, org_id: Uuid) -> AppResult<Option<Unit>>;
    async fn find_by_property(&self, property_id: Uuid, limit: i64, offset: i64) -> AppResult<(Vec<Unit>, i64)>;
    async fn update_status(&self, id: Uuid, status: &str) -> AppResult<Unit>;
    async fn soft_delete(&self, id: Uuid, org_id: Uuid) -> AppResult<()>;
    async fn count_vacant(&self, property_id: Uuid) -> AppResult<i64>;
}
