use axum::{routing::get, Router, Json};
use serde_json::Value;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(list_properties))
}

async fn list_properties() -> Json<Value> {
    Json(serde_json::json!({
        "message": "PMS module is active. Property fetching pending full implementation."
    }))
}