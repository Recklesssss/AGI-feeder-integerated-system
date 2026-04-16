use axum::Router;
use axum::extract::FromRef;
use std::sync::Arc;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    // Sub-domain routers are mounted here.
    // Each sub-router brings its own FromRef bound internally.
    Router::new()
        .nest("/clients",    crate::client::router::routes())
        .nest("/deals",      crate::deal::router::routes())
        .nest("/listings",   crate::listing::router::routes())
        .nest("/commissions", crate::commission::router::routes())
}