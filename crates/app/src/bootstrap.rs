use super::state::AppState;
use sqlx::PgPool; 
use dotenvy::dotenv;
use infra::postgres_user_repository::PostgresUserRepository;

pub async fn create_app_state(db_url: &str) -> AppState {
    dotenv().ok(); 

    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env or environment");
    
    let pool = PgPool::connect(db_url).await.expect("Failed to connect to DB");
    let repo = PostgresUserRepository { pool };

    let user_service = UserService::new(repo);

    AppState {
        user_service: Arc::new(user_service),
    }
}
