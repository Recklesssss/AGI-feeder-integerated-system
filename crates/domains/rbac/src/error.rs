use cores::app_error::AppError;

pub enum RbacError {
    RoleNotFound,
    PermissionNotFound,
    RoleAlreadyAssigned,
    PermissionDenied,
    CannotDeleteSystemRole,
}
impl From<RbacError> for AppError{
    fn from(err:RbacError)-> Self{
        match err{
            RbacError::CannotDeleteSystemRole => AppError::UnAuthorized("can't delete system role".into()),
            RbacError::PermissionDenied => AppError::UnAuthorized("Access denied".into()),
            RbacError::PermissionNotFound => AppError::InvalidInput("not authorized".into()),
            RbacError::RoleAlreadyAssigned => AppError::InvalidInput("already assigned".into()),
            RbacError::RoleNotFound => AppError::InvalidInput("Role is not found".into()),
        }
    }
}