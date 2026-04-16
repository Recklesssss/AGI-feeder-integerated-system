/// DbError — database-layer error type.
///
/// Per the architecture rule: database logic has its own error type that
/// implements `From<DbError> for AppError`. All repository impls return
/// `Result<T, DbError>` internally and callers convert via `?` or `.map_err`.
///
/// This module lives in `cores` so that `From<DbError> for AppError` can be
/// implemented without violating Rust orphan rules (both types are local here).
use thiserror::Error;
use super::app_error::AppError;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Record not found")]
    NotFound,

    #[error("Unique constraint violated: {0}")]
    UniqueViolation(String),

    #[error("Foreign key constraint violated: {0}")]
    ForeignKeyViolation(String),

    #[error("Database connection error: {0}")]
    Connection(String),

    #[error("Query execution error: {0}")]
    Query(String),
}

// ── From<sqlx::Error> for DbError ──────────────────────────────────────────
impl From<sqlx::Error> for DbError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => DbError::NotFound,
            sqlx::Error::Database(ref db_err) => {
                if db_err.is_unique_violation() {
                    DbError::UniqueViolation(db_err.to_string())
                } else if db_err.is_foreign_key_violation() {
                    DbError::ForeignKeyViolation(db_err.to_string())
                } else {
                    DbError::Query(e.to_string())
                }
            }
            sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed => {
                DbError::Connection(e.to_string())
            }
            _ => DbError::Query(e.to_string()),
        }
    }
}

// ── From<DbError> for AppError ─────────────────────────────────────────────
// Both types are in `cores`, so no orphan rule violation.
impl From<DbError> for AppError {
    fn from(err: DbError) -> Self {
        match err {
            DbError::NotFound              => AppError::NotFound("Record not found".into()),
            DbError::UniqueViolation(m)    => AppError::Conflict(m),
            DbError::ForeignKeyViolation(m)=> AppError::Conflict(m),
            DbError::Connection(m)         => AppError::ConnectionTimeout(m),
            DbError::Query(m)              => AppError::DbError(m),
        }
    }
}

// ── Convenience: From<sqlx::Error> for AppError (via DbError) ─────────────
impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::from(DbError::from(e))
    }
}
