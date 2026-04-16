use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;
use core_lib::AppResult;
use super::model::{Order, OrderItem};

#[async_trait]
pub trait OrderRepository: Send + Sync + 'static {
    async fn create_order(&self,
        org_id: Uuid, restaurant_id: Uuid, table_number: Option<&str>,
        opened_by: Option<Uuid>, notes: Option<&str>,
    ) -> AppResult<Order>;

    async fn add_items(&self, order_id: Uuid, items: Vec<(Uuid, i32, Decimal, Decimal, Option<String>)>) -> AppResult<Vec<OrderItem>>;

    async fn find_by_id(&self, id: Uuid, org_id: Uuid) -> AppResult<Option<Order>>;
    async fn find_all(&self, org_id: Uuid, restaurant_id: Option<Uuid>, limit: i64, offset: i64) -> AppResult<(Vec<Order>, i64)>;

    async fn close_order(
        &self, id: Uuid,
        subtotal: Decimal, tax: Decimal, service_charge: Decimal,
        discount: Decimal, total: Decimal,
        payment_method: &str, closed_by: Option<Uuid>,
    ) -> AppResult<Order>;

    async fn cancel_order(&self, id: Uuid) -> AppResult<Order>;

    async fn daily_revenue(&self, org_id: Uuid, restaurant_id: Uuid, date: chrono::NaiveDate) -> AppResult<Decimal>;
}
