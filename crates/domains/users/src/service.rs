use crate::error::UserError;
use crate::model::{User,UserStatus};
use super::repository::UserRepository;
use cores::app_error::AppError;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier,SaltString},
    Argon2
};
use uuid::Uuid;
use chrono::Utc;
use rand_core::OsRng;

pub struct UserService<R:UserRepository> {
    repo: R,
}
impl <R:UserRepository> UserService<R> {
    pub fn new(repo:R)->Self
    {
        Self{repo}
    }
    pub async fn register(&self,email:String,password:String,full_name:String)->Result<User,UserError>{
        if self.repo.get_by_email(&email).await?.is_some(){
            return Err(UserError::EmailAlreadyExists);
        }
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash =  argon2.hash_password(password, &salt)
            .expect("Error during hashing")
            .to_string();
        let user = User {
            id: Uuid::new_v4(),
            email,
            full_name,
            password_hash,
            status: UserStatus::Active,
            created_at: Utc::now(),
        };
        self.repo.register(user).await
    }
    pub  async fn get_by_email(&self,email:String)->Result<Option<User>,UserError>{
        Ok(self.repo.get_by_email(&email).await?)
    }
    pub async fn get_by_id(&self,id:Uuid)->Result<User,UserError>{
        Ok(self.repo.get_by_id(id).await?)
    }
    pub async fn update_email(&self,email:String)->Result<User,UserError>{
        Ok(self.repo.update_email(&email).await?)
    }
    pub async fn change_password(&self,id:Uuid,password:String)->Result<User,UserError>{
        Ok(self.repo.change_password(id, &password).await?)
    }
    pub async fn lock_user(&self,id:Uuid)->Result<UserStatus,UserError>{
        Ok(self.repo.lock_user(id).await?)
    }
    pub async fn unlock_user(self,id:Uuid)->Result<UserStatus,UserError>{
        Ok(self.repo.unlock_user(id).await?)
    }
    pub async fn suspend_user(self,id:Uuid)->Result<UserStatus,UserError>{
        Ok(self.repo.suspend_user(id).await?)
    }
    pub async fn delete_user(self,id:Uuid)->Result<(),UserError>{
        Ok(self.repo.delete_user(id.await?))
    }
    pub async fn list_user(&self,limit:u32)->Result<Vec<User>,UserError>{
        Ok(self.repo.list_user(limit).await?)
    }
}