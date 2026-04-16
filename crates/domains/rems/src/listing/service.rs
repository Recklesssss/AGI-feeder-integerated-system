use std::sync::Arc;
use chrono::Local;
use rust_decimal::Decimal;
use uuid::Uuid;

use cores::{AppError, AppResult};
use shared::pagination::{PaginationParams, PaginatedResponse};
use super::model::{Listing, ListingStatus, ListingType};
use super::repository::ListingRepository;

/// Non-generic ListingService — uses Arc<dyn ListingRepository>.
pub struct ListingService {
    repo: Arc<dyn ListingRepository>,
}

impl ListingService {
    pub fn new(repo: Arc<dyn ListingRepository>) -> Self {
        Self { repo }
    }

    pub async fn create(
        &self, org_id: Uuid, asset_id: Uuid, title: &str,
        description: Option<&str>, price: Decimal, listing_type: ListingType,
    ) -> AppResult<Listing> {
        if price <= Decimal::ZERO {
            return Err(AppError::Validation("Price must be positive".into()));
        }
        if title.trim().is_empty() {
            return Err(AppError::Validation("Title is required".into()));
        }
        self.repo.create(org_id, asset_id, title, description, price, listing_type).await
    }

    pub async fn get(&self, id: Uuid, org_id: Uuid) -> AppResult<Listing> {
        self.repo.find_by_id(id, org_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Listing {id} not found")))
    }

    pub async fn list(
        &self, org_id: Uuid, params: &PaginationParams,
    ) -> AppResult<PaginatedResponse<Listing>> {
        let (items, total) = self.repo
            .find_all(org_id, params.limit(), params.offset())
            .await?;
        Ok(PaginatedResponse::new(items, total, params))
    }

    pub async fn activate(&self, id: Uuid, org_id: Uuid) -> AppResult<Listing> {
        let listing = self.get(id, org_id).await?;
        if !listing.status.can_transition_to(&ListingStatus::Active) {
            return Err(AppError::UnprocessableEntity(
                "Cannot activate listing in current status".into(),
            ));
        }
        let today = Local::now().date_naive();
        self.repo.update_status(id, "active", Some(today)).await
    }

    pub async fn mark_sold(&self, id: Uuid, org_id: Uuid) -> AppResult<Listing> {
        let listing = self.get(id, org_id).await?;
        if !listing.status.can_transition_to(&ListingStatus::Sold) {
            return Err(AppError::UnprocessableEntity(
                "Cannot mark listing as sold in current status".into(),
            ));
        }
        self.repo.update_status(id, "sold", None).await
    }

    pub async fn cancel(&self, id: Uuid, org_id: Uuid) -> AppResult<Listing> {
        let listing = self.get(id, org_id).await?;
        if !listing.status.can_transition_to(&ListingStatus::Cancelled) {
            return Err(AppError::UnprocessableEntity(
                "Cannot cancel listing in current status".into(),
            ));
        }
        self.repo.update_status(id, "cancelled", None).await
    }
}
