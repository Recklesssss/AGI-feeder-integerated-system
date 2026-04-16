use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Restaurant {
    pub id:              Uuid,
    pub organization_id: Uuid,
    pub asset_id:        Uuid,
    pub name:            String,
    pub address:         Option<String>,
    pub created_at:      DateTime<Utc>,
    pub updated_at:      DateTime<Utc>,
}
