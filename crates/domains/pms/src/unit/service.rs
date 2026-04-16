use std::sync::Arc;
use rust_decimal::Decimal;
use uuid::Uuid;
use cores::{AppError, AppResult};
use shared::pagination::{PaginationParams, PaginatedResponse};
use super::{model::{Unit, UnitStatus}, repository::UnitRepository};

pub struct UnitService {
    repo: Arc<dyn UnitRepository>,
}

impl UnitService {
    pub fn new(repo: Arc<dyn UnitRepository>) -> Self { Self { repo } }

    pub async fn create(&self, org_id: Uuid, property_id: Uuid, asset_id: Uuid, unit_number: &str, floor: Option<i32>, bedrooms: Option<i32>, bathrooms: Option<i32>, area_sqm: Option<Decimal>) -> AppResult<Unit> {
        self.repo.create(org_id, property_id, asset_id, unit_number, floor, bedrooms, bathrooms, area_sqm).await
    }

    pub async fn get(&self, id: Uuid, org_id: Uuid) -> AppResult<Unit> {
        self.repo.find_by_id(id, org_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Unit {id} not found")))
    }

    pub async fn list_by_property(&self, property_id: Uuid, params: &PaginationParams) -> AppResult<PaginatedResponse<Unit>> {
        let (units, total) = self.repo.find_by_property(property_id, params.limit(), params.offset()).await?;
        Ok(PaginatedResponse::new(units, total, params))
    }

    pub async fn mark_occupied(&self, id: Uuid, org_id: Uuid) -> AppResult<Unit> {
        let _ = self.get(id, org_id).await?;
        self.repo.update_status(id, UnitStatus::Occupied.as_str()).await
    }

    pub async fn mark_vacant(&self, id: Uuid, org_id: Uuid) -> AppResult<Unit> {
        let _ = self.get(id, org_id).await?;
        self.repo.update_status(id, UnitStatus::Vacant.as_str()).await
    }

    pub async fn vacancy_count(&self, property_id: Uuid) -> AppResult<i64> {
        self.repo.count_vacant(property_id).await
    }
}
