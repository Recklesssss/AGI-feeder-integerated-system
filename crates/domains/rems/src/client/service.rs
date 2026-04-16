use std::sync::Arc;
use uuid::Uuid;

use cores::{AppError, AppResult};
use shared::pagination::{PaginationParams, PaginatedResponse};
use super::model::{Client, ClientType};
use super::repository::ClientRepository;

/// Non-generic ClientService — uses a boxed trait object for the repository
/// so it can be stored directly in AppState without type parameters.
pub struct ClientService {
    repo: Arc<dyn ClientRepository>,
}

impl ClientService {
    pub fn new(repo: Arc<dyn ClientRepository>) -> Self {
        Self { repo }
    }

    pub async fn create(
        &self, org_id: Uuid, name: &str,
        email: Option<&str>, phone: Option<&str>,
        client_type: ClientType, source: Option<&str>,
    ) -> AppResult<Client> {
        if name.trim().is_empty() {
            return Err(AppError::Validation("Client name is required".into()));
        }
        self.repo.create(org_id, name, email, phone, client_type, source).await
    }

    pub async fn get(&self, id: Uuid, org_id: Uuid) -> AppResult<Client> {
        self.repo.find_by_id(id, org_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Client {id} not found")))
    }

    pub async fn list(
        &self, org_id: Uuid, params: &PaginationParams,
    ) -> AppResult<PaginatedResponse<Client>> {
        let (items, total) = self.repo
            .find_all(org_id, params.limit(), params.offset())
            .await?;
        Ok(PaginatedResponse::new(items, total, params))
    }

    pub async fn delete(&self, id: Uuid, org_id: Uuid) -> AppResult<()> {
        let _ = self.get(id, org_id).await?;
        self.repo.soft_delete(id, org_id).await
    }
}
