use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;
use cores::AppResult;
use super::model::Payment;

#[async_trait]
pub trait PaymentRepository: Send + Sync + 'static {
    async fn create(
        &self, invoice_id: Uuid, amount: Decimal, method: &str, paid_at: Option<NaiveDate>,
    ) -> AppResult<Payment>;
    async fn find_by_invoice(&self, invoice_id: Uuid) -> AppResult<Vec<Payment>>;
    async fn total_paid(&self, invoice_id: Uuid) -> AppResult<Decimal>;
}
