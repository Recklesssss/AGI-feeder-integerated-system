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

impl From<FinanceError> for AppError {
    fn from(err: FinanceError) -> Self {
        match err {
            FinanceError::AccountInactive          => AppError::Forbidden("Account is inactive".into()),
            FinanceError::AccountNotFound          => AppError::NotFound("Account not found".into()),
            FinanceError::CurrencyMismatch         => AppError::InvalidInput("Currency mismatch".into()),
            FinanceError::DebitCreditMismatch      => AppError::InvalidInput("Debit/credit mismatch".into()),
            FinanceError::DuplicateTransaction     => AppError::Conflict("Duplicate transaction".into()),
            FinanceError::InsufficientFunds        => AppError::Forbidden("Insufficient funds".into()),
            FinanceError::InvalidJournalEntry      => AppError::Validation("Invalid journal entry".into()),
            FinanceError::InvoiceAlreadyPaid       => AppError::Conflict("Invoice already paid".into()),
            FinanceError::InvoiceNotFound          => AppError::NotFound("Invoice not found".into()),
            FinanceError::LedgerLocked             => AppError::Forbidden("Ledger is locked".into()),
            FinanceError::PaymentAlreadyProcessed  => AppError::Conflict("Payment already processed".into()),
        }
    }
}