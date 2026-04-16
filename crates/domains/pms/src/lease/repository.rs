use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;
use cores::AppResult;
use super::model::Lease;

#[async_trait]
pub trait LeaseRepository: Send + Sync + 'static {
    async fn create(&self, org_id: Uuid, unit_id: Uuid, tenant_id: Uuid, rent: Decimal, security_deposit: Decimal, late_fee: Decimal, billing_day: i32, start_date: NaiveDate, end_date: NaiveDate, notes: Option<&str>) -> AppResult<Lease>;
    async fn find_by_id(&self, id: Uuid, org_id: Uuid) -> AppResult<Option<Lease>>;
    async fn find_active_by_unit(&self, unit_id: Uuid) -> AppResult<Option<Lease>>;
    async fn find_all(&self, org_id: Uuid, limit: i64, offset: i64) -> AppResult<(Vec<Lease>, i64)>;
    async fn update_status(&self, id: Uuid, status: &str) -> AppResult<Lease>;
    async fn find_expiring_soon(&self, org_id: Uuid, within_days: i32) -> AppResult<Vec<Lease>>;
}
