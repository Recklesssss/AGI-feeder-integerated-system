use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use chrono::{DateTime, Utc};
use crate::model::{User, UserStatus};

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUser {
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub full_name: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)] 
pub struct GetByEmail {
    #[validate(email)]
    pub id: Uuid,
    pub email : String,
    pub full_name : String,
    pub status : UserStatus,
    pub created_at : DateTime<Utc>
}

#[derive(Debug, Deserialize, Serialize)] 
pub struct GetById {
    pub id: Uuid,
    pub email : String,
    pub full_name : String,
    pub status : UserStatus,
    pub created_at : DateTime<Utc>

}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateEmail {
    #[validate(email)]
    pub email: String,
    pub id :Uuid
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePassword {
    #[validate(length(min = 8))]
    pub password: String,
    pub id : Uuid,
}

#[derive(Debug, Deserialize)]
pub struct LockUser {
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UnlockUser {
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct SuspendUser {
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct DeleteUser {
    pub id: Uuid,
}

#[derive(Debug, Deserialize)] 
pub struct ListUser {
    pub limit: u32,
}

impl From<User> for GetById {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            full_name : user.full_name,
            email: user.email,
            status : user.status,
            created_at : user.created_at,
        }
    }
}
impl From<User> for GetByEmail {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            full_name : user.full_name,
            email: user.email,
            status : user.status,
            created_at : user.created_at,
        }
    }
}
