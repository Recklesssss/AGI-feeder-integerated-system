use sqlx::{PgPool, Row};
use uuid::Uuid;
use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::Decimal;

use cores::AppResult;
use finance::payment::{
    model::{Payment, PaymentMethod},
    repository::PaymentRepository,
};

pub struct PgPaymentRepository {
    pub db: PgPool,
}

fn map_payment(row: &sqlx::postgres::PgRow) -> Result<Payment, sqlx::Error> {
    let method_str: String = row.try_get("method")?;
    Ok(Payment {
        id:         row.try_get("id")?,
        invoice_id: row.try_get("invoice_id")?,
        amount:     row.try_get("amount")?,
        method:     PaymentMethod::from_str(&method_str),
        paid_at:    row.try_get("paid_at")?,
        created_at: row.try_get("created_at")?,
    })
}

const SELECT_COLS: &str = "id, invoice_id, amount, method, paid_at, created_at";

#[async_trait]
impl PaymentRepository for PgPaymentRepository {
    async fn create(
        &self, invoice_id: Uuid, amount: Decimal, method: &str, paid_at: Option<NaiveDate>,
    ) -> AppResult<Payment> {
        let row = sqlx::query(&format!(
            "INSERT INTO payments (invoice_id, amount, method, paid_at)
             VALUES ($1, $2, $3, $4)
             RETURNING {SELECT_COLS}",
        ))
        .bind(invoice_id)
        .bind(amount)
        .bind(method)
        .bind(paid_at)
        .fetch_one(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        map_payment(&row).map_err(cores::AppError::from)
    }

    async fn find_by_invoice(&self, invoice_id: Uuid) -> AppResult<Vec<Payment>> {
        let rows = sqlx::query(&format!(
            "SELECT {SELECT_COLS} FROM payments
             WHERE invoice_id = $1
             ORDER BY created_at DESC",
        ))
        .bind(invoice_id)
        .fetch_all(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        rows.iter()
            .map(|r| map_payment(r).map_err(cores::AppError::from))
            .collect()
    }

    async fn total_paid(&self, invoice_id: Uuid) -> AppResult<Decimal> {
        let total: Decimal = sqlx::query_scalar(
            "SELECT COALESCE(SUM(amount), 0) FROM payments WHERE invoice_id = $1",
        )
        .bind(invoice_id)
        .fetch_one(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        Ok(total)
    }
}
