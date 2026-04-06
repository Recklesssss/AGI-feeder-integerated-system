use users::router as user_router;

use axum::{routing::get, Router};
use crate::state::AppState; 

async fn home_handler() -> &'static str {
    "Welcome to the Home Page"
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(home_handler))
        
        .nest("/users", user_router::routes())
        
        .with_state(state) 
}
