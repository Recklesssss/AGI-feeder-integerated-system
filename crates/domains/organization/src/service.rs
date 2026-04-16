use std::sync::Arc;
use uuid::Uuid;

use cores::{AppError, AppResult};
use super::{
    model::{Organization, OrgStatus},
    repository::OrgRepository,
};

pub struct OrgService {
    repo: Arc<dyn OrgRepository>,
}

impl OrgService {
    pub fn new(repo: Arc<dyn OrgRepository>) -> Self {
        Self { repo }
    }

    pub async fn create(&self, name: String) -> AppResult<Organization> {
        if name.trim().is_empty() {
            return Err(AppError::Validation("Organization name is required".into()));
        }
        let id = Uuid::new_v4();
        self.repo.create(id, &name).await
    }

    pub async fn get(&self, id: Uuid) -> AppResult<Organization> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Organization {id} not found")))
    }

    pub async fn list(&self, limit: i64, offset: i64) -> AppResult<(Vec<Organization>, i64)> {
        self.repo.find_all(limit, offset).await
    }

    pub async fn suspend(&self, id: Uuid) -> AppResult<Organization> {
        let org = self.get(id).await?;
        if org.status == OrgStatus::Suspended {
            return Err(AppError::Conflict("Organization is already suspended".into()));
        }
        self.repo.update_status(id, "suspended").await
    }

    pub async fn activate(&self, id: Uuid) -> AppResult<Organization> {
        self.repo.update_status(id, "active").await
    }
}
