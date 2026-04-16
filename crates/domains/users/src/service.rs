use crate::error::UserError;
use crate::model::{User, UserStatus};
use super::repository::UserRepository;
use cores::AppResult;
use argon2::{
    password_hash::{PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use uuid::Uuid;
use chrono::Utc;
use rand_core::OsRng;
use std::sync::Arc;

pub struct UserService {
    repo: Arc<dyn UserRepository + Send + Sync>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn register(
        &self,
        email: String,
        password: String,
        full_name: String,
    ) -> AppResult<User> {
        if self.repo.get_by_email(&email).await?.is_some() {
            return Err(UserError::EmailAlreadyExists.into());
        }

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| cores::AppError::InvalidInput(format!("Hashing error: {e}")))?
            .to_string();

        let user = User {
            id:            Uuid::new_v4(),
            email,
            full_name,
            password_hash,
            status:        UserStatus::Active,
            created_at:    Utc::now(),
        };

        self.repo.register(user).await
    }

    pub async fn get_by_email(&self, email: String) -> AppResult<Option<User>> {
        self.repo.get_by_email(&email).await
    }

    pub async fn get_by_id(&self, id: Uuid) -> AppResult<User> {
        self.repo.get_by_id(id).await
    }

    /// Update the email address for user `id`.
    pub async fn update_email(&self, id: Uuid, email: String) -> AppResult<User> {
        self.repo.update_email(id, &email).await
    }

    /// Change the stored password for user `id` (plain-text — service hashes it).
    pub async fn change_password(&self, id: Uuid, new_password: String) -> AppResult<User> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(new_password.as_bytes(), &salt)
            .map_err(|e| cores::AppError::InvalidInput(format!("Hashing error: {e}")))?
            .to_string();
        self.repo.change_password(id, &hash).await
    }

    pub async fn verify_password(&self, user: &User, password: &str) -> AppResult<bool> {
        let parsed = PasswordHash::new(&user.password_hash)
            .map_err(|e| cores::AppError::InvalidInput(format!("Hash parse error: {e}")))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok())
    }

    pub async fn lock_user(&self, id: Uuid) -> AppResult<UserStatus> {
        self.repo.lock_user(id).await
    }

    pub async fn unlock_user(&self, id: Uuid) -> AppResult<UserStatus> {
        self.repo.unlock_user(id).await
    }

    pub async fn suspend_user(&self, id: Uuid) -> AppResult<UserStatus> {
        self.repo.suspend_user(id).await
    }

    pub async fn delete_user(&self, id: Uuid) -> AppResult<()> {
        self.repo.delete_user(id).await
    }

    pub async fn list_user(&self, limit: u32) -> AppResult<Vec<User>> {
        self.repo.list_user(limit).await
    }
}