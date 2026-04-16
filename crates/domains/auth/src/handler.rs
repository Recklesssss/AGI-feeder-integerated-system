use axum::{extract::State, Json};
use std::sync::Arc;
use validator::Validate;

use cores::{AppError, AppResult};
use super::{
    dto::{LoginRequest, RegisterRequest, RefreshRequest, TokenResponse},
    service::AuthService,
};

pub async fn login(
    State(svc): State<Arc<AuthService>>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<TokenResponse>> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let pair = svc.login(req).await?;
    Ok(Json(TokenResponse::bearer(pair)))
}

pub async fn register(
    State(svc): State<Arc<AuthService>>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<Json<serde_json::Value>> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let user_id = svc.register(req).await?;
    Ok(Json(serde_json::json!({ "success": true, "user_id": user_id })))
}

pub async fn refresh(
    State(svc): State<Arc<AuthService>>,
    Json(req): Json<RefreshRequest>,
) -> AppResult<Json<TokenResponse>> {
    let pair = svc.refresh(req).await?;
    Ok(Json(TokenResponse::bearer(pair)))
}
