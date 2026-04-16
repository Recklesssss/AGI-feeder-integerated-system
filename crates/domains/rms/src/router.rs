use axum::Router;
use axum::extract::FromRef;
use std::sync::Arc;

use crate::restaurant::service::RestaurantService;
use crate::menu::service::MenuService;
use crate::order::service::OrderService;
use crate::inventory::service::InventoryService;
use crate::stock::service::StockService;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Arc<RestaurantService>: FromRef<S>,
    Arc<MenuService>: FromRef<S>,
    Arc<OrderService>: FromRef<S>,
    Arc<InventoryService>: FromRef<S>,
    Arc<StockService>: FromRef<S>,
{
    Router::new()
        .nest("/restaurants", crate::restaurant::router::routes())
        .nest("/menus", crate::menu::router::routes())
        .nest("/orders", crate::order::router::routes())
        .nest("/inventory", crate::inventory::router::routes())
        .nest("/stock", crate::stock::router::routes())
}