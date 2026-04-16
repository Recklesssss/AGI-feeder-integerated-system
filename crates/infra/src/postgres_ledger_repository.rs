use sqlx::{PgPool, Row};
use uuid::Uuid;
use async_trait::async_trait;
use rust_decimal::Decimal;

use cores::AppResult;
use finance::ledger::{
    model::{Account, AccountType, LedgerDirection, LedgerEntry},
    repository::LedgerRepository,
};

pub struct PgLedgerRepository {
    pub db: PgPool,
}

// ── String converters (models only have as_str, not from_str) ─────────────

fn account_type_from_str(s: &str) -> AccountType {
    match s {
        "liability" => AccountType::Liability,
        "equity"    => AccountType::Equity,
        "revenue"   => AccountType::Revenue,
        "expense"   => AccountType::Expense,
        _           => AccountType::Asset,
    }
}

fn direction_from_str(s: &str) -> LedgerDirection {
    match s { "credit" => LedgerDirection::Credit, _ => LedgerDirection::Debit }
}

// ── Row mappers ────────────────────────────────────────────────────────────

fn map_account(row: &sqlx::postgres::PgRow) -> Result<Account, sqlx::Error> {
    let at_str: String = row.try_get("account_type")?;
    Ok(Account {
        id:              row.try_get("id")?,
        organization_id: row.try_get("organization_id")?,
        name:            row.try_get("name")?,
        account_type:    account_type_from_str(&at_str),
        created_at:      row.try_get("created_at")?,
        deleted_at:      row.try_get("deleted_at")?,
    })
}

fn map_entry(row: &sqlx::postgres::PgRow) -> Result<LedgerEntry, sqlx::Error> {
    let dir_str: String = row.try_get("direction")?;
    Ok(LedgerEntry {
        id:              row.try_get("id")?,
        organization_id: row.try_get("organization_id")?,
        account_id:      row.try_get("account_id")?,
        amount:          row.try_get("amount")?,
        direction:       direction_from_str(&dir_str),
        reference_type:  row.try_get("reference_type")?,
        reference_id:    row.try_get("reference_id")?,
        created_at:      row.try_get("created_at")?,
    })
}

#[async_trait]
impl LedgerRepository for PgLedgerRepository {
    async fn create_account(
        &self, id: Uuid, org_id: Uuid, name: &str, account_type: AccountType,
    ) -> AppResult<Account> {
        let row = sqlx::query(
            "INSERT INTO accounts (id, organization_id, name, account_type)
             VALUES ($1, $2, $3, $4)
             RETURNING id, organization_id, name, account_type, created_at, deleted_at",
        )
        .bind(id)
        .bind(org_id)
        .bind(name)
        .bind(account_type.as_str())
        .fetch_one(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        map_account(&row).map_err(cores::AppError::from)
    }

    async fn find_accounts(&self, org_id: Uuid) -> AppResult<Vec<Account>> {
        let rows = sqlx::query(
            "SELECT id, organization_id, name, account_type, created_at, deleted_at
             FROM accounts WHERE organization_id = $1 AND deleted_at IS NULL ORDER BY name",
        )
        .bind(org_id)
        .fetch_all(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        rows.iter()
            .map(|r| map_account(r).map_err(cores::AppError::from))
            .collect()
    }

    async fn post_entry(
        &self, org_id: Uuid, account_id: Uuid, amount: Decimal,
        direction: LedgerDirection, ref_type: Option<String>, ref_id: Option<Uuid>,
    ) -> AppResult<LedgerEntry> {
        let row = sqlx::query(
            "INSERT INTO ledger_entries \
             (organization_id, account_id, amount, direction, reference_type, reference_id)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id, organization_id, account_id, amount, direction, \
                       reference_type, reference_id, created_at",
        )
        .bind(org_id)
        .bind(account_id)
        .bind(amount)
        .bind(direction.as_str())
        .bind(ref_type)
        .bind(ref_id)
        .fetch_one(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        map_entry(&row).map_err(cores::AppError::from)
    }

    async fn get_account_balance(&self, account_id: Uuid) -> AppResult<Decimal> {
        // Double-entry: balance = SUM(debits) - SUM(credits)
        let debit: Decimal = sqlx::query_scalar(
            "SELECT COALESCE(SUM(amount), 0) FROM ledger_entries
             WHERE account_id = $1 AND direction = 'debit'",
        )
        .bind(account_id)
        .fetch_one(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        let credit: Decimal = sqlx::query_scalar(
            "SELECT COALESCE(SUM(amount), 0) FROM ledger_entries
             WHERE account_id = $1 AND direction = 'credit'",
        )
        .bind(account_id)
        .fetch_one(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        Ok(debit - credit)
    }

    async fn get_entries(
        &self, org_id: Uuid, account_id: Option<Uuid>, limit: i64, offset: i64,
    ) -> AppResult<Vec<LedgerEntry>> {
        let rows = if let Some(acc_id) = account_id {
            sqlx::query(
                "SELECT id, organization_id, account_id, amount, direction, \
                         reference_type, reference_id, created_at
                 FROM ledger_entries
                 WHERE organization_id = $1 AND account_id = $2
                 ORDER BY created_at DESC LIMIT $3 OFFSET $4",
            )
            .bind(org_id)
            .bind(acc_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.db)
            .await
            .map_err(cores::AppError::from)?
        } else {
            sqlx::query(
                "SELECT id, organization_id, account_id, amount, direction, \
                         reference_type, reference_id, created_at
                 FROM ledger_entries
                 WHERE organization_id = $1
                 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            )
            .bind(org_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.db)
            .await
            .map_err(cores::AppError::from)?
        };

        rows.iter()
            .map(|r| map_entry(r).map_err(cores::AppError::from))
            .collect()
    }
}
