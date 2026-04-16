use std::collections::HashSet;
use std::sync::Arc;
use uuid::Uuid;

use cores::{AppError, AppResult};
use crate::error::RbacError;
use super::model::{AuthUser, Permission};
use super::repository::RbacRepository;

pub struct RbacService {
    repo: Arc<dyn RbacRepository>,
}

impl RbacService {
    pub fn new(repo: Arc<dyn RbacRepository>) -> Self {
        Self { repo }
    }

    /// Build an AuthUser by loading all permission keys from the DB and
    /// converting them to Permission enum values via the TryFrom impl in mapper.rs.
    pub async fn build_auth_user(&self, user_id: Uuid) -> AppResult<AuthUser> {
        let keys = self.repo.get_user_permission_keys(user_id).await?;
        let mut permissions = HashSet::new();

        for key in keys {
            // TryFrom<String> is implemented in mapper.rs (type Error = RbacError)
            let perm = Permission::try_from(key)
                .map_err(|e| AppError::from(e))?;
            permissions.insert(perm);
        }

        Ok(AuthUser { id: user_id, permissions })
    }

    /// Assert that `user` holds `permission`. Returns Err(Forbidden) if not.
    pub fn check(&self, user: &AuthUser, permission: Permission) -> AppResult<()> {
        if user.permissions.contains(&permission) {
            Ok(())
        } else {
            Err(AppError::from(RbacError::PermissionDenied(
                "You do not have the required permission".into(),
            )))
        }
    }
}