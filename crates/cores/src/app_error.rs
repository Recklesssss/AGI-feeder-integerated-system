use thiserror::Error;

/// Unified application error type.
/// All domain-specific errors implement `From<DomainError> for AppError`.
#[derive(Debug, Error)]
pub enum AppError {
    // ── Auth ──────────────────────────────────────────────────────────────
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("JWT error: {0}")]
    Jwt(String),

    // ── Validation / Client errors ────────────────────────────────────────
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    // ── Resource ──────────────────────────────────────────────────────────
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unprocessable entity: {0}")]
    UnprocessableEntity(String),

    // ── Infrastructure ────────────────────────────────────────────────────
    #[error("Database error: {0}")]
    DbError(String),

    #[error("Configuration missing: {0}")]
    ConfigMissing(String),

    #[error("Connection timeout: {0}")]
    ConnectionTimeout(String),

    #[error("Startup failure: {0}")]
    StartupFailure(String),
}