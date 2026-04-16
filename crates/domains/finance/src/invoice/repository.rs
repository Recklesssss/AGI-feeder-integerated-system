use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;
use cores::AppResult;
use super::model::Invoice;

#[async_trait]
pub trait InvoiceRepository: Send + Sync + 'static {
    async fn create(
        &self, org_id: Uuid, asset_id: Option<Uuid>,
        total: Decimal, issued_at: Option<NaiveDate>,
    ) -> AppResult<Invoice>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Invoice>>;
    async fn find_all(&self, org_id: Uuid, limit: i64, offset: i64) -> AppResult<(Vec<Invoice>, i64)>;
    async fn update_status(&self, id: Uuid, status: &str) -> AppResult<Invoice>;
}
