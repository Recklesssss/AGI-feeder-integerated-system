use sqlx::{PgPool, Row};
use uuid::Uuid;
use async_trait::async_trait;

use cores::AppResult;
use auth::{model::{AuthCredentials, UserAuthStatus}, repository::AuthRepository};

pub struct PgAuthRepository {
    pub db: PgPool,
}

fn map_row(row: &sqlx::postgres::PgRow) -> Result<AuthCredentials, sqlx::Error> {
    let status_str: String = row.try_get("status")?;
    Ok(AuthCredentials {
        id:              row.try_get("id")?,
        organization_id: row.try_get("organization_id")?,
        email:           row.try_get("email")?,
        password_hash:   row.try_get("password_hash")?,
        status:          UserAuthStatus::from_str(&status_str),
    })
}

#[async_trait]
impl AuthRepository for PgAuthRepository {
    async fn find_by_email(&self, email: &str) -> AppResult<Option<AuthCredentials>> {
        let row = sqlx::query(
            "SELECT id, organization_id, email, password_hash, status
             FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        match row {
            Some(r) => Ok(Some(map_row(&r).map_err(cores::AppError::from)?)),
            None    => Ok(None),
        }
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<AuthCredentials>> {
        let row = sqlx::query(
            "SELECT id, organization_id, email, password_hash, status
             FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        match row {
            Some(r) => Ok(Some(map_row(&r).map_err(cores::AppError::from)?)),
            None    => Ok(None),
        }
    }

    async fn create_user(
        &self, id: Uuid, org_id: Uuid, email: &str, full_name: &str, password_hash: &str,
    ) -> AppResult<AuthCredentials> {
        sqlx::query(
            "INSERT INTO users (id, organization_id, email, full_name, password_hash, status)
             VALUES ($1, $2, $3, $4, $5, 'active')",
        )
        .bind(id)
        .bind(org_id)
        .bind(email)
        .bind(full_name)
        .bind(password_hash)
        .execute(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        self.find_by_id(id).await?.ok_or_else(|| {
            cores::AppError::DbError("Failed to retrieve newly created user".into())
        })
    }
}
