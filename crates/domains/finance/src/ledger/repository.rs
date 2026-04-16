use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;
use cores::AppResult;
use super::model::{Account, LedgerEntry, LedgerDirection, AccountType};

#[async_trait]
pub trait LedgerRepository: Send + Sync + 'static {
    async fn create_account(
        &self, id: Uuid, org_id: Uuid, name: &str, account_type: AccountType,
    ) -> AppResult<Account>;
    async fn find_accounts(&self, org_id: Uuid) -> AppResult<Vec<Account>>;
    async fn post_entry(
        &self, org_id: Uuid, account_id: Uuid, amount: Decimal,
        direction: LedgerDirection, ref_type: Option<String>, ref_id: Option<Uuid>,
    ) -> AppResult<LedgerEntry>;
    async fn get_account_balance(&self, account_id: Uuid) -> AppResult<Decimal>;
    async fn get_entries(
        &self, org_id: Uuid, account_id: Option<Uuid>, limit: i64, offset: i64,
    ) -> AppResult<Vec<LedgerEntry>>;
}
