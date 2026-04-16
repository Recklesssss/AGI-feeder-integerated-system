use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub id:              Uuid,
    pub organization_id: Uuid,
    pub account_id:      Uuid,
    pub amount:          Decimal,
    pub direction:       LedgerDirection,
    pub reference_type:  Option<String>,
    pub reference_id:    Option<Uuid>,
    pub created_at:      DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LedgerDirection {
    Debit,
    Credit,
}

impl LedgerDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            LedgerDirection::Debit  => "debit",
            LedgerDirection::Credit => "credit",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id:              Uuid,
    pub organization_id: Uuid,
    pub name:            String,
    pub account_type:    AccountType,
    pub created_at:      DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

impl AccountType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccountType::Asset     => "asset",
            AccountType::Liability => "liability",
            AccountType::Equity    => "equity",
            AccountType::Revenue   => "revenue",
            AccountType::Expense   => "expense",
        }
    }
}
