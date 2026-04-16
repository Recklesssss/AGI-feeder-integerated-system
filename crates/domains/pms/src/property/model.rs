use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub id:              Uuid,
    pub organization_id: Uuid,
    pub asset_id:        Uuid,
    pub address:         String,
    pub city:            Option<String>,
    pub country:         Option<String>,
    pub created_at:      DateTime<Utc>,
    pub updated_at:      DateTime<Utc>,
}
