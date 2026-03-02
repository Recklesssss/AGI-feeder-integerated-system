use cores::app_error::AppError;

pub enum AuthError {
    InvalidCredentials,
    UserNotFound,
    AccountLocked,
    EmailNotVerified,
    TokenExpired,
    InvalidToken,
    RefreshTokenInvalid,
    PasswordTooWeak,
    PasswordMismatch,
}
impl From<AuthError> for AppError{
    fn from(err:AuthError)->Self{
        match err{
            AuthError::AccountLocked => AppError::UnAuthorized("account locked".into()) ,
            AuthError::InvalidCredentials => AppError::UnAuthorized("incorrect password".into()),
            AuthError::EmailNotVerified => AppError::UnAuthorized("email not varified".into()),
            AuthError::InvalidToken => AppError::UnAuthorized("token doesn't match".into()),
            AuthError::PasswordMismatch => AppError::UnAuthorized("password miss match".into()),
            AuthError::RefreshTokenInvalid => AppError::UnAuthorized("refreshed token invalid".into()),
            AuthError::PasswordTooWeak => AppError::UnAuthorized("Passwors is too weak".into()),
            AuthError::TokenExpired => AppError::UnAuthorized("token has expired".into()),
            AuthError::UserNotFound => AppError::UnAuthorized("User not found".into()),

        }
    }
}