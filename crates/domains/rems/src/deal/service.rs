use std::sync::Arc;
use chrono::Local;
use rust_decimal::Decimal;
use uuid::Uuid;
use cores::{AppError, AppResult};
use shared::pagination::{PaginationParams, PaginatedResponse};
use super::model::{Deal, DealStatus};
use super::repository::DealRepository;

pub struct DealService {
    repo: Arc<dyn DealRepository>,
}

impl DealService {
    pub fn new(repo: Arc<dyn DealRepository>) -> Self { Self { repo } }

    pub async fn create(&self, org_id: Uuid, listing_id: Uuid, client_id: Uuid, agent_id: Option<Uuid>, deal_value: Decimal, notes: Option<&str>) -> AppResult<Deal> {
        if deal_value <= Decimal::ZERO { return Err(AppError::Validation("Deal value must be positive".into())); }
        self.repo.create(org_id, listing_id, client_id, agent_id, deal_value, notes).await
    }

    pub async fn get(&self, id: Uuid, org_id: Uuid) -> AppResult<Deal> {
        self.repo.find_by_id(id, org_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Deal {id} not found")))
    }

    pub async fn list(&self, org_id: Uuid, params: &PaginationParams) -> AppResult<PaginatedResponse<Deal>> {
        let (items, total) = self.repo.find_all(org_id, params.limit(), params.offset()).await?;
        Ok(PaginatedResponse::new(items, total, params))
    }

    pub async fn advance_stage(&self, id: Uuid, org_id: Uuid, next: DealStatus) -> AppResult<Deal> {
        let deal = self.get(id, org_id).await?;
        if !deal.status.can_transition_to(&next) {
            return Err(AppError::UnprocessableEntity(
                format!("Cannot move deal from '{}' to '{}'", deal.status.as_str(), next.as_str())
            ));
        }
        let closed_at = if next == DealStatus::Closed { Some(Local::now().date_naive()) } else { None };
        self.repo.update_status(id, next.as_str(), closed_at).await
    }

    pub async fn agent_pipeline(&self, agent_id: Uuid, params: &PaginationParams) -> AppResult<PaginatedResponse<Deal>> {
        let (items, total) = self.repo.find_by_agent(agent_id, params.limit(), params.offset()).await?;
        Ok(PaginatedResponse::new(items, total, params))
    }
}
