use axum::Router;
use axum::extract::FromRef;
use std::sync::Arc;
use crate::{
    client::service::ClientService,
    listing::service::ListingService,
};

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<ClientService>:  FromRef<S>,
    Arc<ListingService>: FromRef<S>,
{
    Router::new()
        .nest("/clients",  crate::client::router::routes())
        .nest("/listings", crate::listing::router::routes())
        // deal and commission will be nested here once implemented
}
