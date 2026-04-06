mod state;

#[tokio::main]
async fn main() {
    let state = state::AppState::new().await; 
    
    let app = routes::create_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
