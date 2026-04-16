use sqlx::{PgPool, Row};
use uuid::Uuid;
use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::Decimal;

use cores::AppResult;
use finance::invoice::{
    model::{Invoice, InvoiceStatus},
    repository::InvoiceRepository,
};

pub struct PgInvoiceRepository {
    pub db: PgPool,
}

fn invoice_status_from_str(s: &str) -> InvoiceStatus {
    match s {
        "issued"    => InvoiceStatus::Issued,
        "paid"      => InvoiceStatus::Paid,
        "cancelled" => InvoiceStatus::Cancelled,
        _           => InvoiceStatus::Draft,
    }
}

fn map_invoice(row: &sqlx::postgres::PgRow) -> Result<Invoice, sqlx::Error> {
    let status_str: String = row.try_get("status")?;
    Ok(Invoice {
        id:              row.try_get("id")?,
        organization_id: row.try_get("organization_id")?,
        asset_id:        row.try_get("asset_id")?,
        total:           row.try_get("total")?,
        status:          invoice_status_from_str(&status_str),
        issued_at:       row.try_get("issued_at")?,
        created_at:      row.try_get("created_at")?,
        deleted_at:      row.try_get("deleted_at")?,
    })
}

const SELECT_COLS: &str =
    "id, organization_id, asset_id, total, status, issued_at, created_at, deleted_at";

#[async_trait]
impl InvoiceRepository for PgInvoiceRepository {
    async fn create(
        &self, org_id: Uuid, asset_id: Option<Uuid>,
        total: Decimal, issued_at: Option<NaiveDate>,
    ) -> AppResult<Invoice> {
        let row = sqlx::query(&format!(
            "INSERT INTO invoices (organization_id, asset_id, total, issued_at)
             VALUES ($1, $2, $3, $4)
             RETURNING {SELECT_COLS}",
        ))
        .bind(org_id)
        .bind(asset_id)
        .bind(total)
        .bind(issued_at)
        .fetch_one(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        map_invoice(&row).map_err(cores::AppError::from)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Invoice>> {
        let row = sqlx::query(&format!(
            "SELECT {SELECT_COLS} FROM invoices WHERE id = $1 AND deleted_at IS NULL",
        ))
        .bind(id)
        .fetch_optional(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        match row {
            Some(r) => Ok(Some(map_invoice(&r).map_err(cores::AppError::from)?)),
            None    => Ok(None),
        }
    }

    async fn find_all(
        &self, org_id: Uuid, limit: i64, offset: i64,
    ) -> AppResult<(Vec<Invoice>, i64)> {
        let rows = sqlx::query(&format!(
            "SELECT {SELECT_COLS} FROM invoices
             WHERE organization_id = $1 AND deleted_at IS NULL
             ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        ))
        .bind(org_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        let total: i64 = sqlx::query(
            "SELECT COUNT(*) AS count FROM invoices WHERE organization_id = $1 AND deleted_at IS NULL",
        )
        .bind(org_id)
        .fetch_one(&self.db)
        .await
        .map_err(cores::AppError::from)?
        .get("count");

        let invoices: AppResult<Vec<Invoice>> = rows
            .iter()
            .map(|r| map_invoice(r).map_err(cores::AppError::from))
            .collect();

        Ok((invoices?, total))
    }

    async fn update_status(&self, id: Uuid, status: &str) -> AppResult<Invoice> {
        let row = sqlx::query(&format!(
            "UPDATE invoices SET status = $1
             WHERE id = $2 AND deleted_at IS NULL
             RETURNING {SELECT_COLS}",
        ))
        .bind(status)
        .bind(id)
        .fetch_one(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        map_invoice(&row).map_err(cores::AppError::from)
    }
}
