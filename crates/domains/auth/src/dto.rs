use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// ── Request DTOs ──────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 2, message = "Name too short"))]
    pub full_name: String,
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    pub organization_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

// ── Response DTOs ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token:  String,
    pub refresh_token: String,
    pub expires_in:    i64,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token:  String,
    pub refresh_token: String,
    pub expires_in:    i64,
    pub token_type:    String,
}

impl TokenResponse {
    pub fn bearer(pair: TokenPair) -> Self {
        Self {
            access_token:  pair.access_token,
            refresh_token: pair.refresh_token,
            expires_in:    pair.expires_in,
            token_type:    "Bearer".into(),
        }
    }
}
