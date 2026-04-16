use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ListingStatus { Draft, Active, Sold, Cancelled }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ListingType { Sale, Lease }

impl ListingStatus {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Draft => "draft", Self::Active => "active", Self::Sold => "sold", Self::Cancelled => "cancelled" }
    }
    pub fn can_transition_to(&self, next: &ListingStatus) -> bool {
        matches!((self, next),
            (Self::Draft, Self::Active) | (Self::Active, Self::Sold) |
            (Self::Draft, Self::Cancelled) | (Self::Active, Self::Cancelled))
    }
}

impl ListingType {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Sale => "sale", Self::Lease => "lease" }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    pub id:              Uuid,
    pub organization_id: Uuid,
    pub asset_id:        Uuid,
    pub title:           String,
    pub description:     Option<String>,
    pub price:           Decimal,
    pub listing_type:    ListingType,
    pub status:          ListingStatus,
    pub listed_at:       Option<NaiveDate>,
    pub created_at:      DateTime<Utc>,
    pub updated_at:      DateTime<Utc>,
    pub deleted_at:      Option<DateTime<Utc>>,
}
