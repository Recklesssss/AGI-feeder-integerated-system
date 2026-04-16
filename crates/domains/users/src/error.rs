use cores::app_error::AppError;

#[derive(Debug)]
pub enum UserError {
    UserNotFound,
    EmailAlreadyExists,
    InvalidEmailFormat,
    InvalidPhoneNumber,
    ProfileIncomplete,
    AvatarUploadFailed,
}

impl From<UserError> for AppError {
    fn from(err: UserError) -> Self {
        match err {
            // Bug fix: was AppError::UnAuthorized (wrong name) → AppError::NotFound
            UserError::UserNotFound        => AppError::NotFound("user not found".into()),
            // Bug fix: was AppError::InvalidInput — semantically Conflict is correct for duplicates
            UserError::EmailAlreadyExists  => AppError::Conflict("email already exists".into()),
            UserError::InvalidEmailFormat  => AppError::InvalidInput("invalid email format".into()),
            UserError::InvalidPhoneNumber  => AppError::InvalidInput("invalid phone number".into()),
            UserError::ProfileIncomplete   => AppError::InvalidInput("incomplete profile".into()),
            UserError::AvatarUploadFailed  => AppError::InvalidInput("profile picture upload failed".into()),
        }
    }
}
