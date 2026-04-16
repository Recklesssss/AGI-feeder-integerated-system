use serde::Deserialize;
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Deserialize)]
pub struct CreateMenuDto {
    pub restaurant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub price: Decimal,
    pub cost: Decimal,
}

#[derive(Deserialize)]
pub struct SetAvailableDto {
    pub available: bool,
}

#[derive(Deserialize)]
pub struct UpdatePriceDto {
    pub price: Decimal,
}

#[derive(Deserialize)]
pub struct RestaurantQuery {
    pub restaurant_id: Uuid,
}