use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;
use cores::AppResult;
use super::model::Deal;

#[async_trait]
pub trait DealRepository: Send + Sync + 'static {
    async fn create(&self, org_id: Uuid, listing_id: Uuid, client_id: Uuid, agent_id: Option<Uuid>, deal_value: Decimal, notes: Option<&str>) -> AppResult<Deal>;
    async fn find_by_id(&self, id: Uuid, org_id: Uuid) -> AppResult<Option<Deal>>;
    async fn find_all(&self, org_id: Uuid, limit: i64, offset: i64) -> AppResult<(Vec<Deal>, i64)>;
    async fn find_by_agent(&self, agent_id: Uuid, limit: i64, offset: i64) -> AppResult<(Vec<Deal>, i64)>;
    async fn update_status(&self, id: Uuid, status: &str, closed_at: Option<NaiveDate>) -> AppResult<Deal>;
}
