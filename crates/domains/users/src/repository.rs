use async_trait::async_trait;
use uuid::Uuid;
use cores::AppResult;
use crate::model::{User, UserStatus};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn register(&self, user: User) -> AppResult<User>;
    async fn get_by_id(&self, id: Uuid) -> AppResult<User>;
    async fn get_by_email(&self, email: &str) -> AppResult<Option<User>>;
    /// Update email for the user identified by `id`.
    async fn update_email(&self, id: Uuid, email: &str) -> AppResult<User>;
    /// Store a pre-hashed password for the user identified by `id`.
    async fn change_password(&self, id: Uuid, password_hash: &str) -> AppResult<User>;
    async fn lock_user(&self, id: Uuid) -> AppResult<UserStatus>;
    async fn unlock_user(&self, id: Uuid) -> AppResult<UserStatus>;
    async fn suspend_user(&self, id: Uuid) -> AppResult<UserStatus>;
    async fn delete_user(&self, id: Uuid) -> AppResult<()>;
    async fn list_user(&self, limit: u32) -> AppResult<Vec<User>>;
}