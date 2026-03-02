use cores::app_error::AppError;

pub enum RemsError {
    ProjectNotFound,
    BudgetExceeded,
    ContractorNotFound,
    MilestoneAlreadyCompleted,
    InvalidProjectStatus,
    DuplicateContractorAssignment,
}

impl From<RemsError> for AppError {
    fn from(err: RemsError) -> Self {
        match err{
            RemsError::BudgetExceeded => AppError::InvalidInput(" Budgent exceeded".into()),
            RemsError::ContractorNotFound => AppError::ConfigMissing("contractor not found".into()),
            RemsError::DuplicateContractorAssignment => AppError::InvalidInput("Duplicated contractor assignment".into()),
            RemsError::InvalidProjectStatus => AppError::InvalidInput("invalid project status".into()),
            RemsError::MilestoneAlreadyCompleted => AppError::InvalidInput("Milestone already completed".into()),
            RemsError::ProjectNotFound => AppError::ConfigMissing("Project not found".into()),
        }
    }
}