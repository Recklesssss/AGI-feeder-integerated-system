pub mod app_error;
pub mod db_error;
pub mod error;

pub use app_error::AppError;
pub use db_error::DbError;

/// Convenience alias used across every domain and infra crate.
pub type AppResult<T> = Result<T, AppError>;