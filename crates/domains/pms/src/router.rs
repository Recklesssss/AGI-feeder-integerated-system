use axum::Router;
use axum::extract::FromRef;
use std::sync::Arc;

use crate::property::service::PropertyService;
use crate::unit::service::UnitService;
use crate::tenant::service::TenantService;
use crate::lease::service::LeaseService;
use crate::maintenance::service::MaintenanceService;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<PropertyService>: FromRef<S>,
    Arc<UnitService>: FromRef<S>,
    Arc<TenantService>: FromRef<S>,
    Arc<LeaseService>: FromRef<S>,
    Arc<MaintenanceService>: FromRef<S>,
{
    Router::new()
        .nest("/properties", crate::property::router::routes())
        .nest("/units", crate::unit::router::routes())
        .nest("/tenants", crate::tenant::router::routes())
        .nest("/leases", crate::lease::router::routes())
        .nest("/maintenance", crate::maintenance::router::routes())
}