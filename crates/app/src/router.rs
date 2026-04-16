use axum::{routing::get, Router, Json, extract::State};
use axum::middleware as axum_middleware;
use serde_json::Value;

use crate::state::AppState;
use crate::middleware::jwt_auth;
use tower_http::trace::TraceLayer;

async fn health(State(state): State<AppState>) -> Json<Value> {
    let db_status = match sqlx::query("SELECT 1").fetch_one(&state.db_pool).await {
        Ok(_) => "ok",
        Err(_) => "error",
    };

    Json(serde_json::json!({
        "status": "ok",
        "api": "AGI Enterprise Platform",
        "database": db_status
    }))
}


pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health 
        .route("/health", get(health))
        //  Auth
        .nest("/api/v1/auth",          auth::router::routes())

        //  Protected domains (JWT required) 
        .nest("/api/v1/users",         users::router::routes())
        .nest("/api/v1/organizations", organization::router::routes())
        .nest("/api/v1/assets",        assets::router::routes())
        .nest("/api/v1/rems",          rems::router::routes())
        .nest("/api/v1/finance",       finance::router::routes())

        // Admin (JWT required + admin role via AdminOnly extractor) 
        .nest("/api/v1/admin/rbac",    rbac::router::routes())
        .nest("/api/v1/audit",         audit::router::routes())

        //  Stubs / Implementations pending
        .nest("/api/v1/properties",    pms::router::routes())
        .nest("/api/v1/rentals",       rms::router::routes())

        //  Tracing Middleware
        .layer(TraceLayer::new_for_http())

        //  Apply JWT middleware globally 
        .layer(axum_middleware::from_fn_with_state(state.clone(), jwt_auth))
        //  Resolve all domain state
        .with_state(state)
}

