use std::sync::Arc;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;
use core_lib::{AppError, AppResult};
use super::model::Commission;

use super::repository::CommissionRepository;

pub struct CommissionService<R: CommissionRepository> {
    repo: Arc<R>,
}

impl<R: CommissionRepository> CommissionService<R> {
    pub fn new(repo: Arc<R>) -> Self { Self { repo } }

    /// Calculate and record commission. If percentage given: amount = deal_value × pct.
    pub async fn calculate_and_create(
        &self,
        deal_id: Uuid,
        agent_id: Uuid,
        deal_value: Decimal,
        percentage: Option<Decimal>,
        fixed_amount: Option<Decimal>,
    ) -> AppResult<Commission> {
        let amount = match (percentage, fixed_amount) {
            (Some(pct), _) => {
                if pct <= Decimal::ZERO || pct > Decimal::from(100) {
                    return Err(AppError::Validation("Percentage must be between 0 and 100".into()));
                }
                deal_value * pct / Decimal::from(100)
            },
            (None, Some(amt)) => {
                if amt <= Decimal::ZERO { return Err(AppError::Validation("Commission amount must be positive".into())); }
                amt
            },
            (None, None) => return Err(AppError::Validation("Either percentage or fixed amount required".into())),
        };
        self.repo.create(deal_id, agent_id, amount, percentage).await
    }

    pub async fn for_deal(&self, deal_id: Uuid) -> AppResult<Vec<Commission>> {
        self.repo.find_by_deal(deal_id).await
    }

    pub async fn for_agent(&self, agent_id: Uuid) -> AppResult<Vec<Commission>> {
        self.repo.find_by_agent(agent_id).await
    }

    pub async fn approve(&self, id: Uuid) -> AppResult<Commission> {
        self.repo.approve(id).await
    }

    pub async fn pay(&self, id: Uuid, paid_at: NaiveDate) -> AppResult<Commission> {
        self.repo.mark_paid(id, paid_at).await
    }
}
