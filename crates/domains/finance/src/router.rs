use axum::Router;
use axum::extract::FromRef;
use std::sync::Arc;

use crate::invoice::service::InvoiceService;
use crate::ledger::service::LedgerService;
use crate::payment::service::PaymentService;


pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<InvoiceService>: FromRef<S>,
    Arc<LedgerService>:  FromRef<S>,
    Arc<PaymentService>: FromRef<S>,
{
    Router::new()
        .nest("/invoices", crate::invoice::router::routes())
        .nest("/ledger",   crate::ledger::router::routes())
        .nest("/payments", crate::payment::router::routes())
}