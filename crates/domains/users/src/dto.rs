use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use chrono::{DateTime, Utc};
use crate::model::{User, UserStatus};

// ── Request DTOs (only what's still needed via Json body) ─────────────────

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RegisterUser {
    #[validate(length(min = 3, message = "Full name must be at least 3 characters"))]
    pub full_name: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

/// Body for PUT /:id/email
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateEmailDto {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

/// Body for PUT /:id/password
#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordDto {
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

// ── Response DTOs ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id:         Uuid,
    pub email:      String,
    pub full_name:  String,
    pub status:     UserStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ListUserResponse {
    pub users: Vec<UserResponse>,
    pub count: usize,
}

// ── From impls ────────────────────────────────────────────────────────────

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        Self {
            id:         u.id,
            email:      u.email,
            full_name:  u.full_name,
            status:     u.status,
            created_at: u.created_at,
        }
    }
}

impl From<Vec<User>> for ListUserResponse {
    fn from(users: Vec<User>) -> Self {
        let count = users.len();
        Self {
            users: users.into_iter().map(UserResponse::from).collect(),
            count,
        }
    }
}
