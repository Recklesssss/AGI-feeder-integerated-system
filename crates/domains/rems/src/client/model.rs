use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ClientType { Buyer, Seller, Lessee, Lessor }

impl ClientType {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Buyer => "buyer", Self::Seller => "seller", Self::Lessee => "lessee", Self::Lessor => "lessor" }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub id:              Uuid,
    pub organization_id: Uuid,
    pub name:            String,
    pub email:           Option<String>,
    pub phone:           Option<String>,
    pub client_type:     ClientType,
    pub source:          Option<String>,
    pub created_at:      DateTime<Utc>,
    pub updated_at:      DateTime<Utc>,
}
