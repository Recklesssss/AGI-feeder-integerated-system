use cores::app_error::AppError;

pub enum PmsError {
    PropertyNotFound,
    UnitNotFound,
    LeaseNotFound,
    LeaseAlreadyActive,
    LeaseExpired,
    TenantNotFound,
    UnitAlreadyOccupied,
    InvalidLeaseDates,
    RentAlreadyGenerated,
    MaintenanceRequestNotFound,
}

impl From<PmsError> for AppError {
    fn from(err: PmsError) -> Self {
        match err {
            PmsError::InvalidLeaseDates => AppError::ConnectionTimeout("invalid lease date".into()),
            PmsError::LeaseAlreadyActive => AppError::InvalidInput("Lease already active".into()),
            PmsError::LeaseExpired => AppError::ConnectionTimeout("Lease Expired".into()),
            PmsError::LeaseNotFound => AppError::InvalidInput("Lease not found".into()),
            PmsError::MaintenanceRequestNotFound => AppError::InvalidInput("Maintenance Request not found".into()),
            PmsError::PropertyNotFound => AppError::InvalidInput("Property Not found".into()),
            PmsError::RentAlreadyGenerated => AppError::InvalidInput("rent already generated".into()),
            PmsError::TenantNotFound => AppError::InvalidInput("Tenant Not found".into()),
            PmsError::UnitAlreadyOccupied => AppError::InvalidInput("Unit already occupied".into()),
            PmsError::UnitNotFound => AppError::ConfigMissing("Unit not found".into()),
        }
    }
}