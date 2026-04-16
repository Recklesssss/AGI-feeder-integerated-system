use std::sync::Arc;
use uuid::Uuid;
use argon2::{
    password_hash::{PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use rand_core::OsRng;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::Utc;

use cores::{AppError, AppResult};
use super::{
    dto::{LoginRequest, RegisterRequest, RefreshRequest, TokenPair},
    repository::AuthRepository,
};

use shared::extractors::Claims;
const ACCESS_TTL_SECS:  i64 = 15 * 60;       // 15 min
const REFRESH_TTL_SECS: i64 = 7 * 24 * 3600; // 7 days

// ── AuthService ───────────────────────────────────────────────────────────

pub struct AuthService {
    repo:       Arc<dyn AuthRepository>,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(repo: Arc<dyn AuthRepository>, jwt_secret: String) -> Self {
        Self { repo, jwt_secret }
    }

    // ── private helpers ───────────────────────────────────────────────────

    fn make_token(&self, user_id: Uuid, org_id: Uuid, role: &str, token_type: &str, ttl: i64)
        -> AppResult<String>
    {
        let exp = Utc::now().timestamp() + ttl;
        let claims = Claims {
            sub:        user_id.to_string(),
            org_id:     org_id.to_string(),
            role:       role.to_owned(),
            exp,
            token_type: token_type.to_owned(),
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::Jwt(e.to_string()))
    }

    fn generate_token_pair(&self, user_id: Uuid, org_id: Uuid, role: &str)
        -> AppResult<TokenPair>
    {
        Ok(TokenPair {
            access_token:  self.make_token(user_id, org_id, role, "access",  ACCESS_TTL_SECS)?,
            refresh_token: self.make_token(user_id, org_id, role, "refresh", REFRESH_TTL_SECS)?,
            expires_in:    ACCESS_TTL_SECS,
        })
    }

    /// Validate any JWT and return its claims. Used by middleware and handlers.
    pub fn validate_token(&self, token: &str) -> AppResult<Claims> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation,
        )
        .map(|data| data.claims)
        .map_err(|e| AppError::Jwt(e.to_string()))
    }

    // ── public API ────────────────────────────────────────────────────────

    pub async fn login(&self, req: LoginRequest) -> AppResult<TokenPair> {
        let creds = self.repo.find_by_email(&req.email).await?
            .ok_or_else(|| AppError::Unauthorized("Invalid email or password".into()))?;

        if !creds.status.is_active() {
            return Err(AppError::Unauthorized("Account is inactive".into()));
        }

        let parsed = PasswordHash::new(&creds.password_hash)
            .map_err(|_| AppError::Unauthorized("Invalid credentials".into()))?;
        Argon2::default()
            .verify_password(req.password.as_bytes(), &parsed)
            .map_err(|_| AppError::Unauthorized("Invalid email or password".into()))?;

        self.generate_token_pair(creds.id, creds.organization_id, "staff")
    }

    pub async fn register(&self, req: RegisterRequest) -> AppResult<Uuid> {
        if self.repo.find_by_email(&req.email).await?.is_some() {
            return Err(AppError::Conflict("Email already registered".into()));
        }
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(req.password.as_bytes(), &salt)
            .map_err(|e| AppError::InvalidInput(format!("Hash error: {e}")))?
            .to_string();

        let id   = Uuid::new_v4();
        let creds = self.repo
            .create_user(id, req.organization_id, &req.email, &req.full_name, &hash)
            .await?;
        Ok(creds.id)
    }

    pub async fn refresh(&self, req: RefreshRequest) -> AppResult<TokenPair> {
        let claims = self.validate_token(&req.refresh_token)?;
        if claims.token_type != "refresh" {
            return Err(AppError::Jwt("Not a refresh token".into()));
        }

        let user_id: Uuid = claims.sub.parse()
            .map_err(|_| AppError::Jwt("Invalid user ID in token".into()))?;
        let org_id:  Uuid = claims.org_id.parse()
            .map_err(|_| AppError::Jwt("Invalid org ID in token".into()))?;

        let creds = self.repo.find_by_id(user_id).await?
            .ok_or_else(|| AppError::Unauthorized("User not found".into()))?;
        if !creds.status.is_active() {
            return Err(AppError::Unauthorized("Account is inactive".into()));
        }

        self.generate_token_pair(user_id, org_id, &claims.role)
    }
}
