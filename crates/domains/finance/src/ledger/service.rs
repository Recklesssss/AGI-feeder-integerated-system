use std::sync::Arc;
use rust_decimal::Decimal;
use uuid::Uuid;
use cores::AppResult;
use super::model::{Account, AccountType, LedgerDirection, LedgerEntry};
use super::repository::LedgerRepository;

/// Non-generic LedgerService — uses Arc<dyn LedgerRepository>.
pub struct LedgerService {
    repo: Arc<dyn LedgerRepository>,
}

impl LedgerService {
    pub fn new(repo: Arc<dyn LedgerRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_account(
        &self, org_id: Uuid, name: &str, account_type: AccountType,
    ) -> AppResult<Account> {
        let id = Uuid::new_v4();
        self.repo.create_account(id, org_id, name, account_type).await
    }

    pub async fn list_accounts(&self, org_id: Uuid) -> AppResult<Vec<Account>> {
        self.repo.find_accounts(org_id).await
    }

    /// Post a debit (amount leaves the account).
    pub async fn debit(
        &self, org_id: Uuid, account_id: Uuid, amount: Decimal,
        ref_type: Option<String>, ref_id: Option<Uuid>,
    ) -> AppResult<LedgerEntry> {
        self.repo
            .post_entry(org_id, account_id, amount, LedgerDirection::Debit, ref_type, ref_id)
            .await
    }

    /// Post a credit (amount enters the account).
    pub async fn credit(
        &self, org_id: Uuid, account_id: Uuid, amount: Decimal,
        ref_type: Option<String>, ref_id: Option<Uuid>,
    ) -> AppResult<LedgerEntry> {
        self.repo
            .post_entry(org_id, account_id, amount, LedgerDirection::Credit, ref_type, ref_id)
            .await
    }

    pub async fn get_balance(&self, account_id: Uuid) -> AppResult<Decimal> {
        self.repo.get_account_balance(account_id).await
    }

    pub async fn get_entries(
        &self, org_id: Uuid, account_id: Option<Uuid>, limit: i64, offset: i64,
    ) -> AppResult<Vec<LedgerEntry>> {
        self.repo.get_entries(org_id, account_id, limit, offset).await
    }
}
