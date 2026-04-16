use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id:         Uuid,
    pub invoice_id: Uuid,
    pub amount:     Decimal,
    pub method:     PaymentMethod,
    pub paid_at:    Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    Cash,
    BankTransfer,
    Card,
    Mobile,
}

impl PaymentMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            PaymentMethod::Cash         => "cash",
            PaymentMethod::BankTransfer => "bank_transfer",
            PaymentMethod::Card         => "card",
            PaymentMethod::Mobile       => "mobile",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "cash"          => Self::Cash,
            "bank_transfer" => Self::BankTransfer,
            "card"          => Self::Card,
            "mobile"        => Self::Mobile,
            _               => Self::Cash,
        }
    }
}
