use std::sync::Arc;
use sqlx::PgPool;
use dotenvy::dotenv;

// Infra repository implementations 
use infra::postgres_auth_repository::PgAuthRepository;
use infra::postgres_user_repository::PostgresUserRepository;
use infra::postgres_rbac_repository::PgRbacRepository;
use infra::postgres_org_repository::PgOrgRepository;
use infra::repository_pg::PgAssetRepository;
use infra::postgres_client_repository::PgClientRepository;
use infra::postgres_listing_repository::PgListingRepository;
use infra::postgres_invoice_repository::PgInvoiceRepository;
use infra::postgres_ledger_repository::PgLedgerRepository;
use infra::postgres_payment_repository::PgPaymentRepository;
use infra::postgres_audit_repository::PgAuditRepository;

//  Domain services 
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
use infra::postgres_pms_repository::*;
use infra::postgres_rms_repository::*;
use infra::postgres_rems_repository::*;
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

use crate::state::AppState;

pub async fn create_app_state() -> AppState {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment or .env file");

    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "change-me-in-production-min-32-chars!".to_string());

    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to establish PostgreSQL connection pool");

    // PgPool::clone() is Arc-backed — O(1), no actual connection duplication.
    let auth_service = Arc::new(AuthService::new(
        Arc::new(PgAuthRepository    { db: pool.clone() }),
        jwt_secret,
    ));
    let user_service = Arc::new(UserService::new(
        Arc::new(PostgresUserRepository::new(pool.clone())),
    ));
    let rbac_service = Arc::new(RbacService::new(
        Arc::new(PgRbacRepository    { db: pool.clone() }),
    ));
    let org_service = Arc::new(OrgService::new(
        Arc::new(PgOrgRepository     { db: pool.clone() }),
    ));
    let asset_service = Arc::new(AssetService::new(
        Arc::new(PgAssetRepository   { db: pool.clone() }),
    ));
    let client_service = Arc::new(ClientService::new(
        Arc::new(PgClientRepository  { db: pool.clone() }),
    ));
    let listing_service = Arc::new(ListingService::new(
        Arc::new(PgListingRepository { db: pool.clone() }),
    ));
    let invoice_service = Arc::new(InvoiceService::new(
        Arc::new(PgInvoiceRepository { db: pool.clone() }),
    ));
    let ledger_service = Arc::new(LedgerService::new(
        Arc::new(PgLedgerRepository  { db: pool.clone() }),
    ));
    let payment_service = Arc::new(PaymentService::new(
        Arc::new(PgPaymentRepository { db: pool.clone() }),
    ));
    let audit_service = Arc::new(AuditService::new(
        Arc::new(PgAuditRepository   { db: pool.clone() }),
    ));

    let property_service = Arc::new(PropertyService::new(Arc::new(PgPropertyRepository::new(pool.clone()))));
    let unit_service = Arc::new(UnitService::new(Arc::new(PgUnitRepository::new(pool.clone()))));
    let tenant_service = Arc::new(TenantService::new(Arc::new(PgTenantRepository::new(pool.clone()))));
    let lease_service = Arc::new(LeaseService::new(Arc::new(PgLeaseRepository::new(pool.clone()))));
    let maintenance_service = Arc::new(MaintenanceService::new(Arc::new(PgMaintenanceRepository::new(pool.clone()))));

    let restaurant_service = Arc::new(RestaurantService::new(Arc::new(PgRestaurantRepository::new(pool.clone()))));
    let menu_service = Arc::new(MenuService::new(Arc::new(PgMenuRepository::new(pool.clone()))));
    let order_service = Arc::new(OrderService::new(Arc::new(PgOrderRepository::new(pool.clone())), Arc::new(PgMenuRepository::new(pool.clone()))));
    let inventory_service = Arc::new(InventoryService::new(Arc::new(PgInventoryRepository::new(pool.clone()))));
    let stock_service = Arc::new(StockService::new(Arc::new(PgStockRepository::new(pool.clone())), Arc::new(PgInventoryRepository::new(pool.clone()))));

    let deal_service = Arc::new(DealService::new(Arc::new(PgDealRepository::new(pool.clone()))));
    let commission_service = Arc::new(CommissionService::new(Arc::new(PgCommissionRepository::new(pool.clone()))));

    AppState {
        db_pool: pool.clone(),
        auth_service,
        user_service,
        rbac_service,
        org_service,
        asset_service,
        client_service,
        listing_service,
        invoice_service,
        ledger_service,
        payment_service,
        audit_service,
        property_service,
        unit_service,
        tenant_service,
        lease_service,
        maintenance_service,
        restaurant_service,
        menu_service,
        order_service,
        inventory_service,
        stock_service,
        deal_service,
        commission_service,
    }
}

