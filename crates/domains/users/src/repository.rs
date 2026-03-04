use cores::app_error::AppError;
use crate::model::User;
use async_trait::async_trait;
use uuid::Uuid;


#[async_trait]
pub trait UserRepository {
    async fn register(&self, user:User)->Result<User,AppError>;
    async fn get_by_id(&self,id:Uuid)->Result<User,AppError>;
    async fn get_by_email(&self,email:String)->Result<User,AppError>;
    async fn update_email(&self,email:String)->Result<User,AppError>;
    async fn change_password(&self,id:Uuid,password:String)->Result<(),AppError>;
    async fn lock_user(&self,id:Uuid)->Result<(),AppError>;
    async fn unlock_user(&self,id:Uuid)->Result<(),AppError>;
    async fn suspend_user(&self,id:Uuid)->Result<(),AppError>;
    async fn delete_user(&self, id:Uuid)->Result<(),AppError>;
    async fn list_user(&self,limit:u32,id:Uuid)->Result<Vec<User>,AppError>;
}