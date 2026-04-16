use axum::extract::Query;
use axum::{extract::{State, Path, }, Json};
use std::sync::Arc;
use serde::Deserialize;
use uuid::Uuid;
use rust_decimal::Decimal;

use cores::AppResult;
use super::model::{AccountType, LedgerDirection};
use super::service::LedgerService;

//  Request DTOs 

#[derive(Debug, Deserialize)]
pub struct CreateAccountDto {
    pub org_id:       Uuid,
    pub name:         String,
    pub account_type: String, // "asset" | "liability" | "equity" | "revenue" | "expense"
}

#[derive(Debug, Deserialize)]
pub struct PostEntryDto {
    pub org_id:     Uuid,
    pub account_id: Uuid,
    pub amount:     Decimal,
    pub direction:  String, // "debit" | "credit"
    pub ref_type:   Option<String>,
    pub ref_id:     Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct OrgQuery { pub org_id: Uuid }

#[derive(Debug, Deserialize)]
pub struct EntriesQuery {
    pub org_id:      Uuid,
    pub account_id:  Option<Uuid>,
    #[serde(default = "default_limit")]
    pub limit:  i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 { 50 }

//  Helpers 

fn parse_account_type(s: &str) -> cores::AppResult<AccountType> {
    match s {
        "asset"     => Ok(AccountType::Asset),
        "liability" => Ok(AccountType::Liability),
        "equity"    => Ok(AccountType::Equity),
        "revenue"   => Ok(AccountType::Revenue),
        "expense"   => Ok(AccountType::Expense),
        other       => Err(cores::AppError::Validation(format!("Unknown account type: {other}"))),
    }
}

fn parse_direction(s: &str) -> cores::AppResult<LedgerDirection> {
    match s {
        "debit"  => Ok(LedgerDirection::Debit),
        "credit" => Ok(LedgerDirection::Credit),
        other    => Err(cores::AppError::Validation(format!("Unknown direction: {other}"))),
    }
}

//  Handlers 

pub async fn create_account(
    State(svc): State<Arc<LedgerService>>,
    Json(dto): Json<CreateAccountDto>,
) -> AppResult<Json<serde_json::Value>> {
    let account_type = parse_account_type(&dto.account_type)?;
    let acc = svc.create_account(dto.org_id, &dto.name, account_type).await?;
    Ok(Json(serde_json::json!(acc)))
}

pub async fn list_accounts(
    State(svc): State<Arc<LedgerService>>,
    Query(q): Query<OrgQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let accounts = svc.list_accounts(q.org_id).await?;
    Ok(Json(serde_json::json!(accounts)))
}

pub async fn post_entry(
    State(svc): State<Arc<LedgerService>>,
    Json(dto): Json<PostEntryDto>,
) -> AppResult<Json<serde_json::Value>> {
    let direction = parse_direction(&dto.direction)?;
    let entry = match direction {
        LedgerDirection::Debit  => svc.debit(dto.org_id, dto.account_id, dto.amount, dto.ref_type, dto.ref_id).await?,
        LedgerDirection::Credit => svc.credit(dto.org_id, dto.account_id, dto.amount, dto.ref_type, dto.ref_id).await?,
    };
    Ok(Json(serde_json::json!(entry)))
}

pub async fn get_balance(
    State(svc): State<Arc<LedgerService>>,
    Path(account_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let balance = svc.get_balance(account_id).await?;
    Ok(Json(serde_json::json!({ "account_id": account_id, "balance": balance })))
}

pub async fn get_entries(
    State(svc): State<Arc<LedgerService>>,
    Query(q): Query<EntriesQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let entries = svc.get_entries(q.org_id, q.account_id, q.limit, q.offset).await?;
    Ok(Json(serde_json::json!(entries)))
}