use axum::{routing::{get, post}, Router};
use axum::extract::FromRef;
use std::sync::Arc;
use super::handler;
use crate::service::AssetService;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<AssetService>: FromRef<S>,
{
    Router::new()
        .route("/",           post(handler::create_asset_handler))
        .route("/:org_id",    get(handler::list_assets_handler))
}