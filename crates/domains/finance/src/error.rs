use cores::app_error::AppError;

pub enum FinanceError {
    AccountNotFound,
    AccountInactive,
    InsufficientFunds,
    DebitCreditMismatch,
    CurrencyMismatch,
    DuplicateTransaction,
    LedgerLocked,
    InvalidJournalEntry,
    PaymentAlreadyProcessed,
    InvoiceAlreadyPaid,
    InvoiceNotFound,
}

impl From<FinanceError> for AppError{
    fn from(err:FinanceError) -> Self{
        match err {
            FinanceError::AccountInactive => AppError::ConnectionTimeout("Account is inactive".into()),
            FinanceError::AccountNotFound => AppError::InvalidInput("Account not found".into()),
            FinanceError::CurrencyMismatch => AppError::InvalidInput("Currency Missmatch".into()),
            FinanceError::DebitCreditMismatch => AppError::InvalidInput("Debit credit Mismatch".into()),
            FinanceError::DuplicateTransaction => AppError::InvalidInput("Duplicated Transaction".into()),
            FinanceError::InsufficientFunds => AppError::UnAuthorized("Insufficient fund".into()),
            FinanceError::InvalidJournalEntry => AppError::InvalidInput("invalid jornal entry".into()),
            FinanceError::InvoiceAlreadyPaid => AppError::InvalidInput("invoice already paid".into()),
            FinanceError::InvoiceNotFound => AppError::ConfigMissing("invoice not found".into()),
            FinanceError::LedgerLocked => AppError::UnAuthorized(" ledger locked".into()),
            FinanceError::PaymentAlreadyProcessed => AppError::InvalidInput("Payment already processed".into()),
        }
    }
}