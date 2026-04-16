use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use cores::{AppError, AppResult};
use users::{
    model::{User, UserStatus},
    repository::UserRepository,
};

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

// ── private helpers ────────────────────────────────────────────────────────

fn status_to_str(s: &UserStatus) -> &'static str {
    match s {
        UserStatus::Active    => "active",
        UserStatus::Locked    => "locked",
        UserStatus::Suspended => "suspended",
    }
}

fn status_from_str(s: &str) -> UserStatus {
    match s {
        "locked"    => UserStatus::Locked,
        "suspended" => UserStatus::Suspended,
        _           => UserStatus::Active,
    }
}

fn map_row(row: &sqlx::postgres::PgRow) -> Result<User, sqlx::Error> {
    let status_str: String = row.try_get("status")?;
    Ok(User {
        id:            row.try_get("id")?,
        email:         row.try_get("email")?,
        full_name:     row.try_get("full_name")?,
        password_hash: row.try_get("password_hash")?,
        status:        status_from_str(&status_str),
        created_at:    row.try_get("created_at")?,
        deleted_at:    row.try_get("deleted_at")?,
    })
}

// ── trait implementation ───────────────────────────────────────────────────

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn register(&self, user: User) -> AppResult<User> {
        sqlx::query(
            "INSERT INTO users (id, email, full_name, password_hash, status, created_at)
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(user.id)
        .bind(&user.email)
        .bind(&user.full_name)
        .bind(&user.password_hash)
        .bind(status_to_str(&user.status))
        .bind(user.created_at)
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(user)
    }

    async fn get_by_id(&self, id: Uuid) -> AppResult<User> {
        let row = sqlx::query(
            "SELECT id, email, full_name, password_hash, status, created_at, deleted_at
             FROM users WHERE id = $1 AND deleted_at IS NULL",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::from)?;

        map_row(&row).map_err(AppError::from)
    }

    async fn get_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let row = sqlx::query(
            "SELECT id, email, full_name, password_hash, status, created_at, deleted_at
             FROM users WHERE email = $1 AND deleted_at IS NULL",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::from)?;

        match row {
            Some(r) => Ok(Some(map_row(&r).map_err(AppError::from)?)),
            None    => Ok(None),
        }
    }

    async fn update_email(&self, id: Uuid, email: &str) -> AppResult<User> {
        sqlx::query("UPDATE users SET email = $1 WHERE id = $2 AND deleted_at IS NULL")
            .bind(email)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::from)?;
        self.get_by_id(id).await
    }

    async fn change_password(&self, id: Uuid, password_hash: &str) -> AppResult<User> {
        // NOTE: caller (UserService) is responsible for hashing before passing here
        sqlx::query("UPDATE users SET password_hash = $1 WHERE id = $2 AND deleted_at IS NULL")
            .bind(password_hash)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::from)?;
        self.get_by_id(id).await
    }

    async fn lock_user(&self, id: Uuid) -> AppResult<UserStatus> {
        sqlx::query("UPDATE users SET status = 'locked' WHERE id = $1 AND deleted_at IS NULL")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::from)?;
        Ok(UserStatus::Locked)
    }

    async fn unlock_user(&self, id: Uuid) -> AppResult<UserStatus> {
        sqlx::query("UPDATE users SET status = 'active' WHERE id = $1 AND deleted_at IS NULL")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::from)?;
        Ok(UserStatus::Active)
    }

    async fn suspend_user(&self, id: Uuid) -> AppResult<UserStatus> {
        sqlx::query("UPDATE users SET status = 'suspended' WHERE id = $1 AND deleted_at IS NULL")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::from)?;
        Ok(UserStatus::Suspended)
    }

    async fn delete_user(&self, id: Uuid) -> AppResult<()> {
        sqlx::query("UPDATE users SET status = 'deleted', deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn list_user(&self, limit: u32) -> AppResult<Vec<User>> {
        let rows = sqlx::query(
            "SELECT id, email, full_name, password_hash, status, created_at, deleted_at
             FROM users WHERE deleted_at IS NULL LIMIT $1",
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::from)?;

        rows.iter()
            .map(|r| map_row(r).map_err(AppError::from))
            .collect()
    }
}