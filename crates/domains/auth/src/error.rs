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

impl From<AuthError> for AppError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::AccountLocked       => AppError::Unauthorized("account locked".into()),
            AuthError::InvalidCredentials  => AppError::Unauthorized("incorrect password".into()),
            AuthError::EmailNotVerified    => AppError::Unauthorized("email not verified".into()),
            AuthError::InvalidToken        => AppError::Jwt("token doesn't match".into()),
            AuthError::PasswordMismatch    => AppError::InvalidInput("password mismatch".into()),
            AuthError::RefreshTokenInvalid => AppError::Jwt("refresh token invalid".into()),
            AuthError::PasswordTooWeak     => AppError::Validation("password is too weak".into()),
            AuthError::TokenExpired        => AppError::Jwt("token has expired".into()),
            AuthError::UserNotFound        => AppError::NotFound("user not found".into()),
        }
    }
}