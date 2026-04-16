use axum::{extract::{State, Path, Query}, Json, http::StatusCode};
use std::sync::Arc;
use serde::Deserialize;
use uuid::Uuid;

use cores::{AppError, AppResult};
use crate::dto::{RegisterUser, UpdateEmailDto, ChangePasswordDto, UserResponse, ListUserResponse};
use crate::service::UserService;

// ── Query extractors ───────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct EmailQuery {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    #[serde(default = "default_limit")]
    pub limit:  u32,
    #[serde(default)]
    pub offset: u32,
}
fn default_limit() -> u32 { 20 }

// ── Handlers ───────────────────────────────────────────────────────────────

pub async fn register_handler(
    State(svc): State<Arc<UserService>>,
    Json(payload): Json<RegisterUser>,
) -> AppResult<(StatusCode, Json<UserResponse>)> {
    use validator::Validate;
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let user = svc.register(payload.email, payload.password, payload.full_name).await?;
    Ok((StatusCode::CREATED, Json(UserResponse::from(user))))
}

pub async fn get_by_id_handler(
    State(svc): State<Arc<UserService>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<UserResponse>> {
    let user = svc.get_by_id(id).await?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn get_by_email_handler(
    State(svc): State<Arc<UserService>>,
    Query(q): Query<EmailQuery>,
) -> AppResult<Json<UserResponse>> {
    let user = svc.get_by_email(q.email).await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn list_users_handler(
    State(svc): State<Arc<UserService>>,
    Query(q): Query<ListQuery>,
) -> AppResult<Json<ListUserResponse>> {
    let users = svc.list_user(q.limit).await?;
    Ok(Json(ListUserResponse::from(users)))
}

pub async fn update_email_handler(
    State(svc): State<Arc<UserService>>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateEmailDto>,
) -> AppResult<Json<UserResponse>> {
    use validator::Validate;
    body.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let user = svc.update_email(id, body.email).await?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn change_password_handler(
    State(svc): State<Arc<UserService>>,
    Path(id): Path<Uuid>,
    Json(body): Json<ChangePasswordDto>,
) -> AppResult<StatusCode> {
    use validator::Validate;
    body.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    svc.change_password(id, body.password).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn lock_user_handler(
    State(svc): State<Arc<UserService>>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    svc.lock_user(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn unlock_user_handler(
    State(svc): State<Arc<UserService>>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    svc.unlock_user(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn suspend_user_handler(
    State(svc): State<Arc<UserService>>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    svc.suspend_user(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_user_handler(
    State(svc): State<Arc<UserService>>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    svc.delete_user(id).await?;
    Ok(StatusCode::NO_CONTENT)
}