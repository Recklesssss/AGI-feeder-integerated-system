use crate::domains::users::src::service::UserService;
use ax_extract::FromRef;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
}
impl FromRef<AppState> for Arc<UserService> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.user_service.clone()
    }
}
