use crate::app_error::AppError; 
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

impl IntoResponse for AppError {
    fn into_response(self) -> Response { 
        let (status, error_message):(StatusCode,String) = match self {
            AppError::DbError(_msg) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into()),
            AppError::InvalidInput(_msg) => (StatusCode::BAD_REQUEST, "Invalid input provided".into()),
            AppError::ConfigMissing (_msg)=> (StatusCode::NOT_FOUND,"configuration is missing".into()),
            AppError::ConnectionTimeout (_msg)=> (StatusCode::GATEWAY_TIMEOUT, "connection timeout".into()),
            AppError::StartupFailiure (_msg)=>(StatusCode::EXPECTATION_FAILED,"main failed to start".into()),
            AppError::UnAuthorized(msg) => (StatusCode::FORBIDDEN,msg),
        };

        (status, Json(json!({ "error": error_message }))).into_response()
    }

}
