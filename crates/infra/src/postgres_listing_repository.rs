use sqlx::{PgPool, Row};
use uuid::Uuid;
use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::Decimal;

use cores::AppResult;
use rems::listing::{
    model::{Listing, ListingType, ListingStatus},
    repository::ListingRepository,
};

pub struct PgListingRepository {
    pub db: PgPool,
}

fn listing_type_from_str(s: &str) -> ListingType {
    match s { "lease" => ListingType::Lease, _ => ListingType::Sale }
}

fn listing_status_from_str(s: &str) -> ListingStatus {
    match s {
        "active"    => ListingStatus::Active,
        "sold"      => ListingStatus::Sold,
        "cancelled" => ListingStatus::Cancelled,
        _           => ListingStatus::Draft,
    }
}

fn map_listing(row: &sqlx::postgres::PgRow) -> Result<Listing, sqlx::Error> {
    let lt_str: String = row.try_get("listing_type")?;
    let ls_str: String = row.try_get("status")?;
    Ok(Listing {
        id:              row.try_get("id")?,
        organization_id: row.try_get("organization_id")?,
        asset_id:        row.try_get("asset_id")?,
        title:           row.try_get("title")?,
        description:     row.try_get("description")?,
        price:           row.try_get("price")?,
        listing_type:    listing_type_from_str(&lt_str),
        status:          listing_status_from_str(&ls_str),
        listed_at:       row.try_get("listed_at")?,
        created_at:      row.try_get("created_at")?,
        updated_at:      row.try_get("updated_at")?,
    })
}

const SELECT_COLS: &str =
    "id, organization_id, asset_id, title, description, price, \
     listing_type, status, listed_at, created_at, updated_at";

#[async_trait]
impl ListingRepository for PgListingRepository {
    async fn create(
        &self, org_id: Uuid, asset_id: Uuid, title: &str,
        description: Option<&str>, price: Decimal, listing_type: ListingType,
    ) -> AppResult<Listing> {
        let row = sqlx::query(&format!(
            "INSERT INTO listings \
             (organization_id, asset_id, title, description, price, listing_type)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING {SELECT_COLS}",
        ))
        .bind(org_id)
        .bind(asset_id)
        .bind(title)
        .bind(description)
        .bind(price)
        .bind(listing_type.as_str())
        .fetch_one(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        map_listing(&row).map_err(cores::AppError::from)
    }

    async fn find_by_id(&self, id: Uuid, org_id: Uuid) -> AppResult<Option<Listing>> {
        let row = sqlx::query(&format!(
            "SELECT {SELECT_COLS} FROM listings
             WHERE id = $1 AND organization_id = $2",
        ))
        .bind(id)
        .bind(org_id)
        .fetch_optional(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        match row {
            Some(r) => Ok(Some(map_listing(&r).map_err(cores::AppError::from)?)),
            None    => Ok(None),
        }
    }

    async fn find_all(
        &self, org_id: Uuid, limit: i64, offset: i64,
    ) -> AppResult<(Vec<Listing>, i64)> {
        let rows = sqlx::query(&format!(
            "SELECT {SELECT_COLS} FROM listings
             WHERE organization_id = $1
             ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        ))
        .bind(org_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        let total: i64 = sqlx::query(
            "SELECT COUNT(*) AS count FROM listings WHERE organization_id = $1",
        )
        .bind(org_id)
        .fetch_one(&self.db)
        .await
        .map_err(cores::AppError::from)?
        .get("count");

        let listings: AppResult<Vec<Listing>> = rows
            .iter()
            .map(|r| map_listing(r).map_err(cores::AppError::from))
            .collect();

        Ok((listings?, total))
    }

    async fn update_status(
        &self, id: Uuid, status: &str, listed_at: Option<NaiveDate>,
    ) -> AppResult<Listing> {
        let row = sqlx::query(&format!(
            "UPDATE listings
             SET status = $1, listed_at = $2, updated_at = NOW()
             WHERE id = $3
             RETURNING {SELECT_COLS}",
        ))
        .bind(status)
        .bind(listed_at)
        .bind(id)
        .fetch_one(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        map_listing(&row).map_err(cores::AppError::from)
    }
}
