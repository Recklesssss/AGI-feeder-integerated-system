use cores::AppError;

/// Payload-bearing RBAC error variants (call sites pass descriptive messages).
#[derive(Debug)]
pub enum RbacError {
    RoleNotFound(String),
    PermissionNotFound(String),
    RoleAlreadyAssigned(String),
    PermissionDenied(String),
    CannotDeleteSystemRole(String),
}

impl From<RbacError> for AppError {
    fn from(err: RbacError) -> Self {
        match err {
            RbacError::CannotDeleteSystemRole(m) => AppError::Forbidden(m),
            RbacError::PermissionDenied(m)       => AppError::Forbidden(m),
            RbacError::PermissionNotFound(m)      => AppError::NotFound(m),
            RbacError::RoleAlreadyAssigned(m)     => AppError::Conflict(m),
            RbacError::RoleNotFound(m)            => AppError::NotFound(m),
        }
    }
}