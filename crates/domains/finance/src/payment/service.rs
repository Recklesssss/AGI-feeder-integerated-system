use std::sync::Arc;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;
use cores::{AppError, AppResult};
use super::model::{Payment, PaymentMethod};
use super::repository::PaymentRepository;

/// Non-generic PaymentService — uses Arc<dyn PaymentRepository>.
pub struct PaymentService {
    repo: Arc<dyn PaymentRepository>,
}

impl PaymentService {
    pub fn new(repo: Arc<dyn PaymentRepository>) -> Self {
        Self { repo }
    }

    /// Record a payment against an invoice. Amount must be positive.
    pub async fn record(
        &self,
        invoice_id: Uuid,
        amount: Decimal,
        method: PaymentMethod,
        paid_at: Option<NaiveDate>,
    ) -> AppResult<Payment> {
        if amount <= Decimal::ZERO {
            return Err(AppError::Validation("Payment amount must be positive".into()));
        }
        self.repo.create(invoice_id, amount, method.as_str(), paid_at).await
    }

    pub async fn payments_for_invoice(&self, invoice_id: Uuid) -> AppResult<Vec<Payment>> {
        self.repo.find_by_invoice(invoice_id).await
    }

    pub async fn total_paid(&self, invoice_id: Uuid) -> AppResult<Decimal> {
        self.repo.total_paid(invoice_id).await
    }
}
