use std::sync::Arc;
use rust_decimal::Decimal;
use uuid::Uuid;
use core_lib::{AppError, AppResult};
use shared_lib::pagination::{PaginationParams, PaginatedResponse};
use super::{model::{MaintenanceRequest, MaintenanceStatus}, repository::MaintenanceRepository};

pub struct MaintenanceService<R: MaintenanceRepository> {
    repo: Arc<R>,
}

impl<R: MaintenanceRepository> MaintenanceService<R> {
    pub fn new(repo: Arc<R>) -> Self { Self { repo } }

    pub async fn create(&self, org_id: Uuid, unit_id: Uuid, description: &str, priority: &str, reported_by: Option<Uuid>) -> AppResult<MaintenanceRequest> {
        if description.trim().is_empty() {
            return Err(AppError::Validation("Description is required".into()));
        }
        self.repo.create(org_id, unit_id, description, priority, reported_by).await
    }

    pub async fn get(&self, id: Uuid, org_id: Uuid) -> AppResult<MaintenanceRequest> {
        self.repo.find_by_id(id, org_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Maintenance request {id} not found")))
    }

    pub async fn list(&self, org_id: Uuid, params: &PaginationParams) -> AppResult<PaginatedResponse<MaintenanceRequest>> {
        let (items, total) = self.repo.find_all(org_id, params.limit(), params.offset()).await?;
        Ok(PaginatedResponse::new(items, total, params))
    }

    pub async fn assign(&self, id: Uuid, org_id: Uuid, user_id: Uuid) -> AppResult<MaintenanceRequest> {
        let _ = self.get(id, org_id).await?;
        self.repo.assign(id, user_id).await
    }

    pub async fn resolve(&self, id: Uuid, org_id: Uuid, actual_cost: Option<Decimal>) -> AppResult<MaintenanceRequest> {
        let _ = self.get(id, org_id).await?;
        self.repo.update_status(id, MaintenanceStatus::Resolved.as_str(), actual_cost).await
    }

    pub async fn close(&self, id: Uuid, org_id: Uuid) -> AppResult<MaintenanceRequest> {
        let _ = self.get(id, org_id).await?;
        self.repo.update_status(id, MaintenanceStatus::Closed.as_str(), None).await
    }
}
