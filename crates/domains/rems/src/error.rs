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
        match err {
            RemsError::BudgetExceeded                  => AppError::Forbidden("Budget exceeded".into()),
            RemsError::ContractorNotFound              => AppError::NotFound("Contractor not found".into()),
            RemsError::DuplicateContractorAssignment   => AppError::Conflict("Duplicate contractor assignment".into()),
            RemsError::InvalidProjectStatus            => AppError::UnprocessableEntity("Invalid project status".into()),
            RemsError::MilestoneAlreadyCompleted       => AppError::Conflict("Milestone already completed".into()),
            RemsError::ProjectNotFound                 => AppError::NotFound("Project not found".into()),
        }
    }
}