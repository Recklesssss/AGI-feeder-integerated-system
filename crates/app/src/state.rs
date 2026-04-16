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
use audit::service::AuditService;
use pms::property::service::PropertyService;
use pms::unit::service::UnitService;
use pms::tenant::service::TenantService;
use pms::lease::service::LeaseService;
use pms::maintenance::service::MaintenanceService;
use rms::restaurant::service::RestaurantService;
use rms::menu::service::MenuService;
use rms::order::service::OrderService;
use rms::inventory::service::InventoryService;
use rms::stock::service::StockService;
use rems::deal::service::DealService;
use rems::commission::service::CommissionService;

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
    pub audit_service:   Arc<AuditService>,
    pub property_service:    Arc<PropertyService>,
    pub unit_service:        Arc<UnitService>,
    pub tenant_service:      Arc<TenantService>,
    pub lease_service:       Arc<LeaseService>,
    pub maintenance_service: Arc<MaintenanceService>,
    pub restaurant_service:  Arc<RestaurantService>,
    pub menu_service:        Arc<MenuService>,
    pub order_service:       Arc<OrderService>,
    pub inventory_service:   Arc<InventoryService>,
    pub stock_service:       Arc<StockService>,
    pub deal_service:        Arc<DealService>,
    pub commission_service:  Arc<CommissionService>,
}

// FromRef implementations and clone for explicit defination

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
impl FromRef<AppState> for Arc<AuditService> {
    fn from_ref(s: &AppState) -> Self { s.audit_service.clone() }
}

impl FromRef<AppState> for Arc<PropertyService> { fn from_ref(s: &AppState) -> Self { s.property_service.clone() } }
impl FromRef<AppState> for Arc<UnitService> { fn from_ref(s: &AppState) -> Self { s.unit_service.clone() } }
impl FromRef<AppState> for Arc<TenantService> { fn from_ref(s: &AppState) -> Self { s.tenant_service.clone() } }
impl FromRef<AppState> for Arc<LeaseService> { fn from_ref(s: &AppState) -> Self { s.lease_service.clone() } }
impl FromRef<AppState> for Arc<MaintenanceService> { fn from_ref(s: &AppState) -> Self { s.maintenance_service.clone() } }
impl FromRef<AppState> for Arc<RestaurantService> { fn from_ref(s: &AppState) -> Self { s.restaurant_service.clone() } }
impl FromRef<AppState> for Arc<MenuService> { fn from_ref(s: &AppState) -> Self { s.menu_service.clone() } }
impl FromRef<AppState> for Arc<OrderService> { fn from_ref(s: &AppState) -> Self { s.order_service.clone() } }
impl FromRef<AppState> for Arc<InventoryService> { fn from_ref(s: &AppState) -> Self { s.inventory_service.clone() } }
impl FromRef<AppState> for Arc<StockService> { fn from_ref(s: &AppState) -> Self { s.stock_service.clone() } }
impl FromRef<AppState> for Arc<DealService> { fn from_ref(s: &AppState) -> Self { s.deal_service.clone() } }
impl FromRef<AppState> for Arc<CommissionService> { fn from_ref(s: &AppState) -> Self { s.commission_service.clone() } }
