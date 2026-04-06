use axum::{routing::{get, post, put, delete}, Router};
use::super::handler;

pub fn routes<S>() -> Router<S> 
where 
    S: Clone + Send + Sync + 'static,
    AppState: FromRef<S>, 
{
    Router::new()
    .route("/register",post(handler::register_handler))
    .route("/get_by_email",get(handler::get_by_email_handler))
    .route("/get_by_id",get(handler::get_by_id_handler))
    .route("/update_email",put(handler::update_email_handler))
    .route("/change_password",put(handler::change_password_handler))
    .route("/lock_user",put(handler::lock_user_handler))
    .route("/unlock_user",put(handler::unlock_user_handler))
    .route("/suspend_user",put(handler::suspend_user_handler))
    .route("/delete_user",delete(handler::delete_user_handler))
    .route("/list_user",get(handler::list_user_handler))
}

