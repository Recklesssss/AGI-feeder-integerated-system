use cores::app_error::AppError;

pub enum UserError {
    UserNotFound,
    EmailAlreadyExists,
    InvalidEmailFormat,
    InvalidPhoneNumber,
    ProfileIncomplete,
    AvatarUploadFailed,
}

impl From<UserError> for AppError{
    fn from(err: UserError) -> Self {
        match err {
            UserError::UserNotFound => AppError::UnAuthorized("user couldn't be found".into()), 
            UserError::AvatarUploadFailed => AppError::InvalidInput("profile picture upload failed".into()),
            UserError::InvalidPhoneNumber => AppError::InvalidInput("invalid phone number".into()),
            UserError::EmailAlreadyExists => AppError::InvalidInput("email already exists".into()),
            UserError::InvalidEmailFormat => AppError::InvalidInput("invalid email format".into()),
            UserError::ProfileIncomplete => AppError::InvalidInput("incomplete profile".into()),   
        }
        
    }
}