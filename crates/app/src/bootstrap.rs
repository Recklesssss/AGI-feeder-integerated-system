use std::sync::Arc;
use sqlx::PgPool;
use dotenvy::dotenv;

// ── Infra repository implementations ──────────────────────────────────────
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

// ── Domain services ────────────────────────────────────────────────────────
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
    }
}
