use serde::Deserialize;
use uuid::Uuid;
use rust_decimal::Decimal;
use super::model::ListingType;

#[derive(Debug, Deserialize)]
pub struct CreateListingDto {
    pub org_id:       Uuid,
    pub asset_id:     Uuid,
    pub title:        String,
    pub description:  Option<String>,
    pub price:        Decimal,
    pub listing_type: ListingType,
}

/// Re-export Listing as the response DTO.
pub use crate::listing::model::Listing as ListingResponseDto;
