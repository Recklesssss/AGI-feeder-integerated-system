use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    Property,
    Unit,
    Restaurant,
    Listing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetStatus {
    Active,
    Inactive,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id:              Uuid,
    pub organization_id: Uuid,
    pub asset_type:      AssetType,
    pub name:            String,
    pub status:          AssetStatus,
    pub created_at:      DateTime<Utc>,
    pub updated_at:      DateTime<Utc>,
    pub deleted_at:      Option<DateTime<Utc>>,
}