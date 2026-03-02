pub enum AppError {
    DbError(String),
    InvalidInput(String),
    ConnectionTimeout(String),
    StartupFailiure(String),
    ConfigMissing(String),   
    UnAuthorized(String),
}