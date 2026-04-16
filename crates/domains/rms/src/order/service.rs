use std::sync::Arc;
use rust_decimal::Decimal;
use uuid::Uuid;
use chrono::NaiveDate;
use cores::{AppError, AppResult};
use shared::pagination::{PaginationParams, PaginatedResponse};
use super::model::{Order, NewOrderItem, OrderStatus};
use super::repository::OrderRepository;
use crate::menu::repository::MenuRepository;

pub struct OrderService {
    order_repo: Arc<dyn OrderRepository>,
    menu_repo:  Arc<dyn MenuRepository>,
}

impl OrderService {
    pub fn new(order_repo: Arc<dyn OrderRepository>, menu_repo: Arc<dyn MenuRepository>) -> Self {
        Self { order_repo, menu_repo }
    }

    /// Open a new POS ticket.
    pub async fn open_order(
        &self,
        org_id: Uuid,
        restaurant_id: Uuid,
        table_number: Option<&str>,
        opened_by: Option<Uuid>,
        new_items: Vec<NewOrderItem>,
        notes: Option<&str>,
    ) -> AppResult<Order> {
        if new_items.is_empty() {
            return Err(AppError::Validation("Order must have at least one item".into()));
        }

        let order = self.order_repo.create_order(org_id, restaurant_id, table_number, opened_by, notes).await?;

        // Resolve menu prices
        let mut resolved: Vec<(Uuid, i32, Decimal, Decimal, Option<String>)> = Vec::new();
        for item in &new_items {
            let menu_item = self.menu_repo.find_by_id(item.menu_item_id).await?
                .ok_or_else(|| AppError::NotFound(format!("Menu item {} not found", item.menu_item_id)))?;
            if !menu_item.is_available {
                return Err(AppError::UnprocessableEntity(format!("'{}' is not available", menu_item.name)));
            }
            let unit_price = menu_item.price;
            let line_total = unit_price * Decimal::from(item.quantity);
            resolved.push((item.menu_item_id, item.quantity, unit_price, line_total, item.notes.clone()));
        }

        self.order_repo.add_items(order.id, resolved).await?;

        // Re-fetch with items
        self.order_repo.find_by_id(order.id, org_id).await?
            .ok_or_else(|| AppError::DbError("Order disappeared after creation".into()))
    }

    /// Close (pay) an order. Calculates totals, records payment.
    pub async fn close_order(
        &self,
        id: Uuid,
        org_id: Uuid,
        tax_rate: Decimal,
        service_charge_rate: Decimal,
        discount: Decimal,
        payment_method: &str,
        closed_by: Option<Uuid>,
    ) -> AppResult<Order> {
        let order = self.get(id, org_id).await?;
        if order.status != OrderStatus::Pending {
            return Err(AppError::UnprocessableEntity("Order is not open".into()));
        }

        let subtotal: Decimal = order.items.iter().map(|i| i.line_total).sum();
        let tax            = subtotal * tax_rate / Decimal::from(100);
        let service_charge = subtotal * service_charge_rate / Decimal::from(100);
        let total          = subtotal + tax + service_charge - discount;

        self.order_repo.close_order(id, subtotal, tax, service_charge, discount, total, payment_method, closed_by).await
    }

    pub async fn cancel(&self, id: Uuid, org_id: Uuid) -> AppResult<Order> {
        let order = self.get(id, org_id).await?;
        if order.status != OrderStatus::Pending {
            return Err(AppError::UnprocessableEntity("Only pending orders can be cancelled".into()));
        }
        self.order_repo.cancel_order(id).await
    }

    pub async fn get(&self, id: Uuid, org_id: Uuid) -> AppResult<Order> {
        self.order_repo.find_by_id(id, org_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Order {id} not found")))
    }

    pub async fn list(&self, org_id: Uuid, restaurant_id: Option<Uuid>, params: &PaginationParams) -> AppResult<PaginatedResponse<Order>> {
        let (items, total) = self.order_repo.find_all(org_id, restaurant_id, params.limit(), params.offset()).await?;
        Ok(PaginatedResponse::new(items, total, params))
    }

    pub async fn daily_revenue(&self, org_id: Uuid, restaurant_id: Uuid, date: NaiveDate) -> AppResult<Decimal> {
        self.order_repo.daily_revenue(org_id, restaurant_id, date).await
    }
}
