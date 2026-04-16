use axum::{routing::get, Router, Json};
use serde_json::Value;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(list_rentals))
}

async fn list_rentals() -> Json<Value> {
    Json(serde_json::json!({
        "message": "RMS module is active. Rental listing pending full implementation."
    }))
}