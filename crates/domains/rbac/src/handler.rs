use axum::{extract::{State, Path}, Json};
use std::sync::Arc;
use uuid::Uuid;

use cores::AppResult;
use shared::extractors::AdminOnly;
use super::service::RbacService;

/// Returns all permission keys held by the given user.
/// Protected: requires admin role (enforced by AdminOnly extractor locally).
pub async fn get_user_permissions(
    _admin: AdminOnly,
    State(svc): State<Arc<RbacService>>,
    Path(user_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let auth_user = svc.build_auth_user(user_id).await?;
    let perms: Vec<String> = auth_user
        .permissions
        .into_iter()
        .map(|p| format!("{:?}", p))
        .collect();
    Ok(Json(serde_json::json!({
        "user_id":     user_id,
        "permissions": perms,
        "count":       perms.len(),
    })))
}