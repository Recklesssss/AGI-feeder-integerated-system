use std::sync::Arc;
use rust_decimal::Decimal;
use uuid::Uuid;
use chrono::NaiveDate;
use cores::{AppError, AppResult};
use super::model::{StockMovement, MovementType};
use super::repository::StockRepository;
use crate::inventory::repository::InventoryRepository;

pub struct StockService {
    stock_repo:     Arc<dyn StockRepository>,
    inventory_repo: Arc<dyn InventoryRepository>,
}

impl StockService {
    pub fn new(stock_repo: Arc<dyn StockRepository>, inventory_repo: Arc<dyn InventoryRepository>) -> Self {
        Self { stock_repo, inventory_repo }
    }

    /// Record any stock movement and update inventory quantity atomically.
    pub async fn record_movement(
        &self,
        inventory_item_id: Uuid,
        quantity: Decimal,
        movement_type: MovementType,
        reference_type: Option<String>,
        reference_id: Option<Uuid>,
        notes: Option<String>,
        recorded_by: Option<Uuid>,
    ) -> AppResult<StockMovement> {
        if quantity <= Decimal::ZERO {
            return Err(AppError::Validation("Movement quantity must be positive".into()));
        }

        let delta = movement_type.signed_delta(quantity);

        // Guard against negative stock for Out/Waste movements
        if delta < Decimal::ZERO {
            let item = self.inventory_repo.find_by_id(inventory_item_id).await?
                .ok_or_else(|| AppError::NotFound(format!("Item {inventory_item_id} not found")))?;
            if item.quantity + delta < Decimal::ZERO {
                return Err(AppError::UnprocessableEntity(
                    format!("Insufficient stock: have {}, movement would result in negative", item.quantity)
                ));
            }
        }

        // Post the stock movement (immutable)
        let movement = self.stock_repo.record(
            inventory_item_id, quantity, movement_type,
            reference_type, reference_id, notes, recorded_by,
        ).await?;

        // Adjust inventory quantity
        self.inventory_repo.adjust_quantity(inventory_item_id, delta).await?;

        Ok(movement)
    }

    pub async fn history(&self, inventory_item_id: Uuid, limit: i64, offset: i64) -> AppResult<Vec<StockMovement>> {
        self.stock_repo.find_by_item(inventory_item_id, limit, offset).await
    }

    pub async fn waste_report(&self, restaurant_id: Uuid, from: NaiveDate, to: NaiveDate) -> AppResult<Decimal> {
        self.stock_repo.waste_total(restaurant_id, from, to).await
    }
}
