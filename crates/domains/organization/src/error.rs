use cores::app_error::AppError;

pub enum OrganizationError {
    OrganizationNotFound,
    OrganizationAlreadyExists,
    MemberAlreadyExists,
    MemberNotFound,
    CannotDeleteDefaultOrganization,
    CrossTenantAccessDenied,
}

impl From<OrganizationError> for AppError {
    fn from(err:OrganizationError) -> Self{
        match err {
            OrganizationError::CannotDeleteDefaultOrganization => AppError::UnAuthorized("inseficient athorization".into()),
            OrganizationError::MemberNotFound => AppError::InvalidInput("member not found".into()),
            OrganizationError::CrossTenantAccessDenied => AppError::UnAuthorized("Not owned tenant access".into()),
            OrganizationError::MemberAlreadyExists => AppError::InvalidInput("Member already exists".into()),
            OrganizationError::OrganizationAlreadyExists => AppError::InvalidInput("Organization already exists".into()),
            OrganizationError::OrganizationNotFound => AppError::InvalidInput("Organization not found".into()),
        }
    }
    
}