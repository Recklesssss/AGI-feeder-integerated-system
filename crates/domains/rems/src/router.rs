use axum::Router;
use axum::extract::FromRef;
use std::sync::Arc;
use crate::{
    client::service::ClientService,
    listing::service::ListingService,
    deal::service::DealService,
    commission::service::CommissionService,
};

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<ClientService>:     FromRef<S>,
    Arc<ListingService>:    FromRef<S>,
    Arc<DealService>:       FromRef<S>,
    Arc<CommissionService>: FromRef<S>,
{
    Router::new()
        .nest("/clients",     crate::client::router::routes())
        .nest("/listings",    crate::listing::router::routes())
        .nest("/deals",       crate::deal::router::routes())
        .nest("/commissions", crate::commission::router::routes())
}
