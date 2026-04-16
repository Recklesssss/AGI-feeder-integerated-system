use sqlx::{PgPool, Row};
use uuid::Uuid;
use async_trait::async_trait;

use cores::{AppError, AppResult};
use rbac::repository::RbacRepository;

pub struct PgRbacRepository {
    pub db: PgPool,
}

#[async_trait]
impl RbacRepository for PgRbacRepository {
    async fn get_user_permission_keys(
        &self,
        user_id: Uuid,
    ) -> AppResult<Vec<String>> {
        let rows = sqlx::query(
            r#"
            SELECT p.key
            FROM user_roles ur
            JOIN role_permissions rp ON rp.role_id = ur.role_id
            JOIN permissions p ON p.id = rp.permission_id
            WHERE ur.user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.db)
        .await
        .map_err(AppError::from)?;

        Ok(rows
            .into_iter()
            .map(|r| r.get::<String, _>("key"))
            .collect())
    }
}