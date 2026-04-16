use sqlx::{PgPool, Row};
use uuid::Uuid;
use async_trait::async_trait;

use cores::AppResult;
use organization::{
    model::{Organization, OrgStatus},
    repository::OrgRepository,
};

pub struct PgOrgRepository {
    pub db: PgPool,
}

fn map_org(row: &sqlx::postgres::PgRow) -> Result<Organization, sqlx::Error> {
    let status_str: String = row.try_get("status")?;
    Ok(Organization {
        id:         row.try_get("id")?,
        name:       row.try_get("name")?,
        status:     OrgStatus::from_str(&status_str),
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        deleted_at: row.try_get("deleted_at")?,
    })
}

#[async_trait]
impl OrgRepository for PgOrgRepository {
    async fn create(&self, id: Uuid, name: &str) -> AppResult<Organization> {
        let row = sqlx::query(
            "INSERT INTO organizations (id, name, status)
             VALUES ($1, $2, 'active')
             RETURNING id, name, status, created_at, updated_at, deleted_at",
        )
        .bind(id)
        .bind(name)
        .fetch_one(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        map_org(&row).map_err(cores::AppError::from)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Organization>> {
        let row = sqlx::query(
            "SELECT id, name, status, created_at, updated_at, deleted_at
             FROM organizations WHERE id = $1 AND deleted_at IS NULL",
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        match row {
            Some(r) => Ok(Some(map_org(&r).map_err(cores::AppError::from)?)),
            None    => Ok(None),
        }
    }

    async fn find_all(&self, limit: i64, offset: i64) -> AppResult<(Vec<Organization>, i64)> {
        let rows = sqlx::query(
            "SELECT id, name, status, created_at, updated_at, deleted_at FROM organizations
             WHERE deleted_at IS NULL
             ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        let total: i64 = sqlx::query("SELECT COUNT(*) as count FROM organizations WHERE deleted_at IS NULL")
            .fetch_one(&self.db)
            .await
            .map_err(cores::AppError::from)?
            .get("count");

        let orgs: AppResult<Vec<Organization>> = rows.iter()
            .map(|r| map_org(r).map_err(cores::AppError::from))
            .collect();

        Ok((orgs?, total))
    }

    async fn update_status(&self, id: Uuid, status: &str) -> AppResult<Organization> {
        let row = sqlx::query(
            "UPDATE organizations SET status = $1, updated_at = NOW()
             WHERE id = $2 AND deleted_at IS NULL
             RETURNING id, name, status, created_at, updated_at, deleted_at",
        )
        .bind(status)
        .bind(id)
        .fetch_one(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        map_org(&row).map_err(cores::AppError::from)
    }

    async fn soft_delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query("UPDATE organizations SET status = 'deleted', deleted_at = NOW(), updated_at = NOW() WHERE id = $1 AND deleted_at IS NULL")
            .bind(id)
            .execute(&self.db)
            .await
            .map_err(cores::AppError::from)?;
        Ok(())
    }
}
