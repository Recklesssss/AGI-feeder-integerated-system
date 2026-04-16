use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DealStatus { Prospect, Negotiation, Contracted, Closed, Failed }

impl DealStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Prospect    => "prospect",
            Self::Negotiation => "negotiation",
            Self::Contracted  => "contracted",
            Self::Closed      => "closed",
            Self::Failed      => "failed",
        }
    }

    pub fn can_transition_to(&self, next: &DealStatus) -> bool {
        matches!((self, next),
            (Self::Prospect,    Self::Negotiation) |
            (Self::Negotiation, Self::Contracted)  |
            (Self::Contracted,  Self::Closed)      |
            (Self::Prospect,    Self::Failed)      |
            (Self::Negotiation, Self::Failed)      |
            (Self::Contracted,  Self::Failed))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deal {
    pub id:              Uuid,
    pub organization_id: Uuid,
    pub listing_id:      Uuid,
    pub client_id:       Uuid,
    pub agent_id:        Option<Uuid>,
    pub deal_value:      Decimal,
    pub status:          DealStatus,
    pub closed_at:       Option<NaiveDate>,
    pub notes:           Option<String>,
    pub created_at:      DateTime<Utc>,
    pub updated_at:      DateTime<Utc>,
    pub deleted_at:      Option<DateTime<Utc>>,
}
