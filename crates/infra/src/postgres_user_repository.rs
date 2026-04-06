use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domains::users::model::{User, UserStatus};
use crate::domains::users::repository::UserRepository;
use cores::app_error::AppError;

pub struct PostgresUserRepository {
    
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn register(&self, user: User) -> Result<User, AppError> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, email, full_name, password_hash, status, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, email, full_name, password_hash, status as "status: UserStatus", created_at
            "#,
            user.id,
            user.email,
            user.full_name,
            user.password_hash,
            user.status as UserStatus,
            user.created_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::from) // Ensure AppError implements From<sqlx::Error>
    }

    async fn get_by_id(&self, id: Uuid) -> Result<User, AppError> {
        sqlx::query_as!(
            User,
            r#"SELECT id, email, full_name, password_hash, status as "status: UserStatus", created_at 
               FROM users WHERE id = $1"#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::from)
    }

    async fn get_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        sqlx::query_as!(
            User,
            r#"SELECT id, email, full_name, password_hash, status as "status: UserStatus", created_at 
               FROM users WHERE email = $1"#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::from)
    }

    async fn lock_user(&self, id: Uuid) -> Result<UserStatus, AppError> {
        let status = UserStatus::Locked;
        sqlx::query!(
            "UPDATE users SET status = $1 WHERE id = $2",
            status as UserStatus,
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(status)
    }

    async fn delete_user(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn list_user(&self, limit: u32) -> Result<Vec<User>, AppError> {
        sqlx::query_as!(
            User,
            r#"SELECT id, email, full_name, password_hash, status as "status: UserStatus", created_at 
               FROM users LIMIT $1"#,
            limit as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::from)
    }

}