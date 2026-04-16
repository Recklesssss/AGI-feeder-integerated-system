use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "agi_platform=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting AGI Enterprise Platform...");

    // Load .env, build DB pool, instantiate all domain services
    let state = app::bootstrap::create_app_state().await;

    // Assemble all domain routes with state + JWT middleware
    let router = app::router::create_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind port 3000");

    println!("AGI Enterprise Platform listening on http://0.0.0.0:3000");

    axum::serve(listener, router)
        .await
        .expect("Server crashed");
}
