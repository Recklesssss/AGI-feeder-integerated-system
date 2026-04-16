use crate::app_error::AppError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message): (StatusCode, String) = match self {
            AppError::Unauthorized(m)         => (StatusCode::UNAUTHORIZED,            m),
            AppError::Forbidden(m)            => (StatusCode::FORBIDDEN,               m),
            AppError::Jwt(m)                  => (StatusCode::UNAUTHORIZED,            m),
            AppError::Validation(m)           => (StatusCode::UNPROCESSABLE_ENTITY,   m),
            AppError::InvalidInput(m)         => (StatusCode::BAD_REQUEST,             m),
            AppError::Conflict(m)             => (StatusCode::CONFLICT,                m),
            AppError::NotFound(m)             => (StatusCode::NOT_FOUND,               m),
            AppError::UnprocessableEntity(m)  => (StatusCode::UNPROCESSABLE_ENTITY,   m),
            AppError::DbError(m)              => (StatusCode::INTERNAL_SERVER_ERROR,   m),
            AppError::ConfigMissing(m)        => (StatusCode::INTERNAL_SERVER_ERROR,   m),
            AppError::ConnectionTimeout(m)    => (StatusCode::GATEWAY_TIMEOUT,         m),
            AppError::StartupFailure(m)       => (StatusCode::INTERNAL_SERVER_ERROR,   m),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
