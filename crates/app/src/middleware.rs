pub mod auth;

/// Request/Response logging (T-16 — pending implementation).
pub mod logging;

/// Permission-based RBAC guard middleware (T-02 — see app::extractors for handler-level RBAC).
pub mod rbca;

/// Multi-tenant context middleware (future).
pub mod tenant;



use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};

use crate::state::AppState;

/// Routes that bypass JWT validation entirely.
const PUBLIC_PATHS: &[&str] = &[
    "/health",
    "/api/v1/auth/login",
    "/api/v1/auth/register",
    "/api/v1/auth/refresh",
];


pub async fn jwt_auth(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let path = req.uri().path().to_owned();

    //  Bypass public routes 
    if PUBLIC_PATHS.contains(&path.as_str()) {
        return next.run(req).await;
    }

    // Extract Bearer token 
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(str::to_owned);

    let token = match token {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Missing Authorization header. Expected: Bearer <token>"
                })),
            )
                .into_response()
        }
    };

    //  Validate JWT
    let claims = match state.auth_service.validate_token(&token) {
        Ok(c)  => c,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Invalid or expired access token"
                })),
            )
                .into_response()
        }
    };

    //  Store claims in extensions for downstream extractors 
    req.extensions_mut().insert(claims);
    next.run(req).await
}
