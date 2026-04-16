use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;
use cores::AppResult;
use super::model::{Listing, ListingType};

#[async_trait]
pub trait ListingRepository: Send + Sync + 'static {
    async fn create(
        &self, org_id: Uuid, asset_id: Uuid, title: &str,
        description: Option<&str>, price: Decimal, listing_type: ListingType,
    ) -> AppResult<Listing>;
    async fn find_by_id(&self, id: Uuid, org_id: Uuid) -> AppResult<Option<Listing>>;
    async fn find_all(&self, org_id: Uuid, limit: i64, offset: i64) -> AppResult<(Vec<Listing>, i64)>;
    async fn update_status(&self, id: Uuid, status: &str, listed_at: Option<NaiveDate>) -> AppResult<Listing>;
}
