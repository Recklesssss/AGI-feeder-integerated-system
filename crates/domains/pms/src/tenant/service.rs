use std::sync::Arc;
use uuid::Uuid;
use core_lib::{AppError, AppResult};
use shared_lib::pagination::{PaginationParams, PaginatedResponse};
use super::{model::Tenant, repository::TenantRepository};

pub struct TenantService<R: TenantRepository> {
    repo: Arc<R>,
}

impl<R: TenantRepository> TenantService<R> {
    pub fn new(repo: Arc<R>) -> Self { Self { repo } }

    pub async fn create(&self, org_id: Uuid, name: &str, email: Option<&str>, phone: Option<&str>, national_id: Option<&str>) -> AppResult<Tenant> {
        if name.trim().is_empty() {
            return Err(AppError::Validation("Tenant name is required".into()));
        }
        self.repo.create(org_id, name, email, phone, national_id).await
    }

    pub async fn get(&self, id: Uuid, org_id: Uuid) -> AppResult<Tenant> {
        self.repo.find_by_id(id, org_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Tenant {id} not found")))
    }

    pub async fn list(&self, org_id: Uuid, params: &PaginationParams) -> AppResult<PaginatedResponse<Tenant>> {
        let (tenants, total) = self.repo.find_all(org_id, params.limit(), params.offset()).await?;
        Ok(PaginatedResponse::new(tenants, total, params))
    }

    pub async fn delete(&self, id: Uuid, org_id: Uuid) -> AppResult<()> {
        let _ = self.get(id, org_id).await?;
        self.repo.soft_delete(id, org_id).await
    }
}
