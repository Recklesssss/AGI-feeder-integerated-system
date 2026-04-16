use std::sync::Arc;
use axum::extract::FromRef;

use auth::service::AuthService;
use users::service::UserService;
use rbac::service::RbacService;
use organization::service::OrgService;
use assets::service::AssetService;
use rems::client::service::ClientService;
use rems::listing::service::ListingService;
use finance::invoice::service::InvoiceService;
use finance::ledger::service::LedgerService;
use finance::payment::service::PaymentService;

use sqlx::PgPool;

/// Central application state shared across all Axum handlers.
#[derive(Clone)]
pub struct AppState {
    pub db_pool:         PgPool,
    pub auth_service:    Arc<AuthService>,
    pub user_service:    Arc<UserService>,
    pub rbac_service:    Arc<RbacService>,
    pub org_service:     Arc<OrgService>,
    pub asset_service:   Arc<AssetService>,
    pub client_service:  Arc<ClientService>,
    pub listing_service: Arc<ListingService>,
    pub invoice_service: Arc<InvoiceService>,
    pub ledger_service:  Arc<LedgerService>,
    pub payment_service: Arc<PaymentService>,
}

// ── FromRef implementations ────────────────────────────────────────────────

impl FromRef<AppState> for Arc<AuthService> {
    fn from_ref(s: &AppState) -> Self { s.auth_service.clone() }
}
impl FromRef<AppState> for Arc<UserService> {
    fn from_ref(s: &AppState) -> Self { s.user_service.clone() }
}
impl FromRef<AppState> for Arc<RbacService> {
    fn from_ref(s: &AppState) -> Self { s.rbac_service.clone() }
}
impl FromRef<AppState> for Arc<OrgService> {
    fn from_ref(s: &AppState) -> Self { s.org_service.clone() }
}
impl FromRef<AppState> for Arc<AssetService> {
    fn from_ref(s: &AppState) -> Self { s.asset_service.clone() }
}
impl FromRef<AppState> for Arc<ClientService> {
    fn from_ref(s: &AppState) -> Self { s.client_service.clone() }
}
impl FromRef<AppState> for Arc<ListingService> {
    fn from_ref(s: &AppState) -> Self { s.listing_service.clone() }
}
impl FromRef<AppState> for Arc<InvoiceService> {
    fn from_ref(s: &AppState) -> Self { s.invoice_service.clone() }
}
impl FromRef<AppState> for Arc<LedgerService> {
    fn from_ref(s: &AppState) -> Self { s.ledger_service.clone() }
}
impl FromRef<AppState> for Arc<PaymentService> {
    fn from_ref(s: &AppState) -> Self { s.payment_service.clone() }
}
