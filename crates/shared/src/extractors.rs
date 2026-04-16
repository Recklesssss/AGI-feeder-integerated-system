use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

// ── JWT Claims ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub:        String, // user_id (UUID)
    pub org_id:     String,
    pub role:       String,
    pub exp:        i64,
    pub token_type: String, // "access" | "refresh"
}

// ── CurrentUser ────────────────────────────────────────────────────────────

pub struct CurrentUser(pub Claims);

impl CurrentUser {
    pub fn claims(&self) -> &Claims {
        &self.0
    }

    pub fn user_id(&self) -> Uuid {
        self.0.sub.parse().unwrap_or(Uuid::nil())
    }

    pub fn org_id(&self) -> Uuid {
        self.0.org_id.parse().unwrap_or(Uuid::nil())
    }

    pub fn role(&self) -> &str {
        &self.0.role
    }

    pub fn is_admin(&self) -> bool {
        self.0.role == "admin"
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for CurrentUser {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
            .cloned()
            .map(CurrentUser)
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(serde_json::json!({"error": "Not authenticated"})),
                )
                    .into_response()
            })
    }
}

// ── AdminOnly ──────────────────────────────────────────────────────────────

pub struct AdminOnly(pub Claims);

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for AdminOnly {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let CurrentUser(claims) = CurrentUser::from_request_parts(parts, state).await?;
        if claims.role != "admin" {
            return Err((
                StatusCode::FORBIDDEN,
                Json(serde_json::json!({
                    "error": "This endpoint requires administrator privileges"
                })),
            )
                .into_response());
        }
        Ok(AdminOnly(claims))
    }
}
