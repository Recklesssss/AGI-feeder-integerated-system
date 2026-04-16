use std::sync::Arc;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;
use cores::{AppError, AppResult};
use super::model::{Invoice, InvoiceStatus};
use super::repository::InvoiceRepository;

/// Non-generic InvoiceService — uses Arc<dyn InvoiceRepository> so it can
/// be stored directly in AppState without exposing type parameters.
pub struct InvoiceService {
    repo: Arc<dyn InvoiceRepository>,
}

impl InvoiceService {
    pub fn new(repo: Arc<dyn InvoiceRepository>) -> Self {
        Self { repo }
    }

    pub async fn create(
        &self, org_id: Uuid, asset_id: Option<Uuid>,
        total: Decimal, issued_at: Option<NaiveDate>,
    ) -> AppResult<Invoice> {
        if total <= Decimal::ZERO {
            return Err(AppError::Validation("Invoice total must be positive".into()));
        }
        self.repo.create(org_id, asset_id, total, issued_at).await
    }

    pub async fn get(&self, id: Uuid) -> AppResult<Invoice> {
        self.repo.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound(format!("Invoice {id} not found")))
    }

    pub async fn list(&self, org_id: Uuid, limit: i64, offset: i64) -> AppResult<(Vec<Invoice>, i64)> {
        self.repo.find_all(org_id, limit, offset).await
    }

    pub async fn issue(&self, id: Uuid) -> AppResult<Invoice> {
        let invoice = self.get(id).await?;
        if !invoice.status.can_transition_to(&InvoiceStatus::Issued) {
            return Err(AppError::UnprocessableEntity(
                format!("Cannot issue invoice in status '{}'", invoice.status.as_str()),
            ));
        }
        self.repo.update_status(id, "issued").await
    }

    pub async fn mark_paid(&self, id: Uuid) -> AppResult<Invoice> {
        let invoice = self.get(id).await?;
        if !invoice.status.can_transition_to(&InvoiceStatus::Paid) {
            return Err(AppError::UnprocessableEntity(
                format!("Cannot mark invoice as paid in status '{}'", invoice.status.as_str()),
            ));
        }
        self.repo.update_status(id, "paid").await
    }

    pub async fn cancel(&self, id: Uuid) -> AppResult<Invoice> {
        let invoice = self.get(id).await?;
        if !invoice.status.can_transition_to(&InvoiceStatus::Cancelled) {
            return Err(AppError::UnprocessableEntity(
                format!("Cannot cancel invoice in status '{}'", invoice.status.as_str()),
            ));
        }
        self.repo.update_status(id, "cancelled").await
    }
}
