use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;
use core_lib::AppResult;
use super::model::Commission;

#[async_trait]
pub trait CommissionRepository: Send + Sync + 'static {
    async fn create(&self, deal_id: Uuid, agent_id: Uuid, amount: Decimal, percentage: Option<Decimal>) -> AppResult<Commission>;
    async fn find_by_deal(&self, deal_id: Uuid) -> AppResult<Vec<Commission>>;
    async fn find_by_agent(&self, agent_id: Uuid) -> AppResult<Vec<Commission>>;
    async fn approve(&self, id: Uuid) -> AppResult<Commission>;
    async fn mark_paid(&self, id: Uuid, paid_at: NaiveDate) -> AppResult<Commission>;
}
