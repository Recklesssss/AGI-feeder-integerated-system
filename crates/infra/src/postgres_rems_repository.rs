use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{NaiveDate, Utc};

use cores::{AppError, AppResult};

use rems::{
    deal::{repository::DealRepository, model::Deal},
    commission::{repository::CommissionRepository, model::Commission},
};

// ── Deal Repository ──────────────────────────────────────────────────────
pub struct PgDealRepository { pub pool: PgPool }
impl PgDealRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[async_trait]
impl DealRepository for PgDealRepository {
    async fn create(&self, org_id: Uuid, listing_id: Uuid, client_id: Uuid, _agent_id: Option<Uuid>, deal_value: Decimal, _notes: Option<&str>) -> AppResult<Deal> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let status = "pending";
        let row = sqlx::query("INSERT INTO deals (id, organization_id, listing_id, client_id, deal_value, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *")
            .bind(id).bind(org_id).bind(listing_id).bind(client_id).bind(deal_value).bind(status).bind(now).bind(now)
            .fetch_one(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
            
        Ok(Deal {
            id: row.try_get("id").unwrap_or_default(),
            organization_id: row.try_get("organization_id").unwrap_or_default(),
            listing_id: row.try_get("listing_id").unwrap_or_default(),
            client_id: row.try_get("client_id").unwrap_or_default(),
            agent_id: None,
            deal_value: row.try_get("deal_value").unwrap_or_default(),
            status: rems::deal::model::DealStatus::Prospect,
            notes: None, closed_at: None,
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
            deleted_at: None,
        })
    }

    async fn find_by_id(&self, id: Uuid, org_id: Uuid) -> AppResult<Option<Deal>> {
        let row_res = sqlx::query("SELECT * FROM deals WHERE id = $1 AND organization_id = $2")
            .bind(id).bind(org_id).fetch_optional(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;

        match row_res {
            Some(row) => {
                let status_str: String = row.try_get("status").unwrap_or_default();
                let status = match status_str.as_str() {
                    "closed" => rems::deal::model::DealStatus::Closed,
                    "cancelled" => rems::deal::model::DealStatus::Failed,
                    _ => rems::deal::model::DealStatus::Prospect,
                };
                Ok(Some(Deal {
                    id: row.try_get("id").unwrap_or_default(),
                    organization_id: row.try_get("organization_id").unwrap_or_default(),
                    listing_id: row.try_get("listing_id").unwrap_or_default(),
                    client_id: row.try_get("client_id").unwrap_or_default(),
                    agent_id: None,
                    deal_value: row.try_get("deal_value").unwrap_or_default(),
                    status,
                    notes: None, closed_at: None,
                    created_at: row.try_get("created_at").unwrap_or_default(),
                    updated_at: row.try_get("updated_at").unwrap_or_default(),
                    deleted_at: None,
                }))
            },
            None => Ok(None)
        }
    }

    async fn find_all(&self, org_id: Uuid, limit: i64, offset: i64) -> AppResult<(Vec<Deal>, i64)> {
        let rows = sqlx::query("SELECT * FROM deals WHERE organization_id = $1 LIMIT $2 OFFSET $3")
            .bind(org_id).bind(limit).bind(offset).fetch_all(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
            
        let mut out = Vec::new();
        for row in rows {
            out.push(Deal {
                id: row.try_get("id").unwrap_or_default(),
                organization_id: row.try_get("organization_id").unwrap_or_default(),
                listing_id: row.try_get("listing_id").unwrap_or_default(),
                client_id: row.try_get("client_id").unwrap_or_default(),
                agent_id: None,
                deal_value: row.try_get("deal_value").unwrap_or_default(),
                status: rems::deal::model::DealStatus::Prospect,
                notes: None, closed_at: None,
                created_at: row.try_get("created_at").unwrap_or_default(),
                updated_at: row.try_get("updated_at").unwrap_or_default(),
                deleted_at: None,
            });
        }
        let count = sqlx::query("SELECT COUNT(*) FROM deals WHERE organization_id = $1").bind(org_id).fetch_one(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        Ok((out, count.try_get(0).unwrap_or(0)))
    }
    
    async fn find_by_agent(&self, _agent_id: Uuid, _limit: i64, _offset: i64) -> AppResult<(Vec<Deal>, i64)> {
        Ok((Vec::new(), 0))
    }

    async fn update_status(&self, id: Uuid, status: &str, _closed_at: Option<NaiveDate>) -> AppResult<Deal> {
        let now = Utc::now();
        sqlx::query("UPDATE deals SET status = $1, updated_at = $2 WHERE id = $3")
            .bind(status).bind(now).bind(id).execute(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        self.find_by_id(id, Uuid::new_v4()).await?.ok_or(AppError::NotFound("Missing".into()))
    }
}

// ── Commission Repository ────────────────────────────────────────────────
pub struct PgCommissionRepository { pub pool: PgPool }
impl PgCommissionRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[async_trait]
impl CommissionRepository for PgCommissionRepository {
    async fn create(&self, deal_id: Uuid, agent_id: Uuid, amount: Decimal, _percentage: Option<Decimal>) -> AppResult<Commission> {
        let id = Uuid::new_v4();
        let row = sqlx::query("INSERT INTO commissions (id, deal_id, agent_id, amount) VALUES ($1, $2, $3, $4) RETURNING *")
            .bind(id).bind(deal_id).bind(agent_id).bind(amount)
            .fetch_one(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
            
        Ok(Commission {
            id: row.try_get("id").unwrap_or_default(),
            deal_id: row.try_get("deal_id").unwrap_or_default(),
            agent_id: row.try_get("agent_id").unwrap_or_default(),
            amount: row.try_get("amount").unwrap_or_default(),
            percentage: None,
            status: rems::commission::model::CommissionStatus::Pending,
            paid_at: None,
            created_at: Utc::now(), // missing in schema
            
        })
    }

    async fn find_by_deal(&self, deal_id: Uuid) -> AppResult<Vec<Commission>> {
        let rows = sqlx::query("SELECT * FROM commissions WHERE deal_id = $1").bind(deal_id).fetch_all(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        let mut out = Vec::new();
        for row in rows {
            out.push(Commission {
                id: row.try_get("id").unwrap_or_default(),
                deal_id: row.try_get("deal_id").unwrap_or_default(),
                agent_id: row.try_get("agent_id").unwrap_or_default(),
                amount: row.try_get("amount").unwrap_or_default(),
                percentage: None,
                status: rems::commission::model::CommissionStatus::Pending,
                paid_at: None,
                created_at: Utc::now(), 
            });
        }
        Ok(out)
    }

    async fn find_by_agent(&self, agent_id: Uuid) -> AppResult<Vec<Commission>> {
        let rows = sqlx::query("SELECT * FROM commissions WHERE agent_id = $1").bind(agent_id).fetch_all(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        let mut out = Vec::new();
        for row in rows {
            out.push(Commission {
                id: row.try_get("id").unwrap_or_default(),
                deal_id: row.try_get("deal_id").unwrap_or_default(),
                agent_id: row.try_get("agent_id").unwrap_or_default(),
                amount: row.try_get("amount").unwrap_or_default(),
                percentage: None,
                status: rems::commission::model::CommissionStatus::Pending,
                paid_at: None,
                created_at: Utc::now(), 
            });
        }
        Ok(out)
    }

    async fn approve(&self, id: Uuid) -> AppResult<Commission> {
        // Schema doesn't track status, just return mock wrapper
        let row = sqlx::query("SELECT * FROM commissions WHERE id = $1").bind(id).fetch_one(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        Ok(Commission {
            id: row.try_get("id").unwrap_or_default(),
            deal_id: row.try_get("deal_id").unwrap_or_default(),
            agent_id: row.try_get("agent_id").unwrap_or_default(),
            amount: row.try_get("amount").unwrap_or_default(),
            percentage: None,
            status: rems::commission::model::CommissionStatus::Approved,
            paid_at: None,
            created_at: Utc::now(), 
        })
    }

    async fn mark_paid(&self, id: Uuid, _paid_at: NaiveDate) -> AppResult<Commission> {
        let row = sqlx::query("SELECT * FROM commissions WHERE id = $1").bind(id).fetch_one(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        Ok(Commission {
            id: row.try_get("id").unwrap_or_default(),
            deal_id: row.try_get("deal_id").unwrap_or_default(),
            agent_id: row.try_get("agent_id").unwrap_or_default(),
            amount: row.try_get("amount").unwrap_or_default(),
            percentage: None,
            status: rems::commission::model::CommissionStatus::Paid,
            paid_at: None,
            created_at: Utc::now(), 
        })
    }
}

