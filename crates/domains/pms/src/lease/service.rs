use std::sync::Arc;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;
use cores::{AppError, AppResult};
use shared::pagination::{PaginationParams, PaginatedResponse};
use super::{model::{Lease, LeaseStatus}, repository::LeaseRepository};

pub struct LeaseService {
    repo: Arc<dyn LeaseRepository>,
}

impl LeaseService {
    pub fn new(repo: Arc<dyn LeaseRepository>) -> Self { Self { repo } }

    pub async fn create(
        &self, org_id: Uuid, unit_id: Uuid, tenant_id: Uuid,
        rent: Decimal, security_deposit: Decimal, late_fee: Decimal,
        billing_day: i32, start_date: NaiveDate, end_date: NaiveDate,
        notes: Option<&str>,
    ) -> AppResult<Lease> {
        if end_date <= start_date {
            return Err(AppError::Validation("end_date must be after start_date".into()));
        }
        if rent <= Decimal::ZERO {
            return Err(AppError::Validation("Rent must be positive".into()));
        }
        // Prevent double-leasing a unit
        if self.repo.find_active_by_unit(unit_id).await?.is_some() {
            return Err(AppError::Conflict("Unit already has an active lease".into()));
        }
        self.repo.create(org_id, unit_id, tenant_id, rent, security_deposit, late_fee, billing_day, start_date, end_date, notes).await
    }

    pub async fn get(&self, id: Uuid, org_id: Uuid) -> AppResult<Lease> {
        self.repo.find_by_id(id, org_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Lease {id} not found")))
    }

    pub async fn list(&self, org_id: Uuid, params: &PaginationParams) -> AppResult<PaginatedResponse<Lease>> {
        let (leases, total) = self.repo.find_all(org_id, params.limit(), params.offset()).await?;
        Ok(PaginatedResponse::new(leases, total, params))
    }

    pub async fn terminate(&self, id: Uuid, org_id: Uuid) -> AppResult<Lease> {
        let lease = self.get(id, org_id).await?;
        if lease.status != LeaseStatus::Active {
            return Err(AppError::UnprocessableEntity("Only active leases can be terminated".into()));
        }
        self.repo.update_status(id, "terminated").await
    }

    pub async fn expiring_soon(&self, org_id: Uuid, within_days: i32) -> AppResult<Vec<Lease>> {
        self.repo.find_expiring_soon(org_id, within_days).await
    }
}
