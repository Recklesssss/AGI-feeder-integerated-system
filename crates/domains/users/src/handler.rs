use axum::{extract::State, Json, http::StatusCode};
use crate::dto::{RegisterUser, GetByEmail, GetById, UpdateEmail, ChangePassword, LockUser, UnlockUser, SuspendUser, DeleteUser, ListUser};
use crate::service::UserService;
use cores::app_error::AppError;
use validator::Validate;
use std::sync::Arc;

pub async fn register_handler(
    State(user_service): State<Arc<UserService>>, 
    Json(payload): Json<RegisterUser>,   
) -> Result<(StatusCode, Json<RegisterUser>), AppError> {
    
    payload.validate().map_err(AppError::from)?;

    let new_user = user_service.register(payload.email, payload.password, payload.full_name).await?;

    Ok((StatusCode::CREATED, Json(RegisterUser::from(new_user))))
}
pub async fn get_by_email_handler (

    State(user_service):State<Arc<UserService>>,

    Json(payload):Json<GetByEmail>,

)->Result<(StatusCode, Json<GetByEmail>), AppError> {

    payload.validate().map_err(AppError::from)?;

    let user = user_service.get_by_email(payload.email).await?;

    Ok((StatusCode::CREATED, Json(GetByEmail::from(user))))
}
pub async fn get_by_id_handler (
    State(user_service): State<Arc<UserService>>, 

    Json(payload): Json<GetById>
)-> Result<(StatusCode, Json<GetById>), AppError>{

    let user = user_service.get_by_id(payload.id).await?;

    Ok((StatusCode::CREATED, Json(GetById::from(user))))
}

pub async fn update_email_handler(
    State(user_service):State<Arc<UserService>>
    , Json(payload): Json<UpdateEmail>
)-> Result<(StatusCode), AppError> {

    payload.validate().map_err(AppError::from)?;
    
    let user = user_service.update_email(payload.email).await?;
    Ok((StatusCode::Ok))
}

pub async fn change_password_handler(
    State(user_service) : State<Arc<UserService>>,
    Json(payload): Json<ChangePassword>
)->Result<(StatusCode),AppError>{
    let user = user_service(payload.id,payload.password).await?;
    Ok((StatusCode::Ok))
}

pub async fn lock_user_handler(
    State(user_service) : State<Arc<UserService>>,
    Json(payload): Json<LockUser>
)-> Result<(StatusCode), AppError>{
    let user = user_service.lock_user(payload.id).await?;
    Ok((StatusCode::Ok))    
}

pub async fn unlock_user_handler(
    State(user_service) : State<Arc<UserService>>,
    Json(payload): Json<UnlockUser>
)-> Result<(StatusCode), AppError>{
    let user = user_service.unlock_user(payload.id).await?;
    Ok((StatusCode::Ok))    
} 

pub async fn suspend_user_handler(
    State(user_service) : State<Arc<UserService>>,
    Json(payload): Json<SuspendUser>
)-> Result<(StatusCode), AppError>{
    let user = user_service.suspend_user(payload.id).await?;
    Ok((StatusCode::Ok))    
}

pub async fn delete_user_handler(
    State(user_service) : State<Arc<UserService>>,
    Json(payload): Json<DeleteUser>
)-> Result<(StatusCode), AppError>{
    let user = user_service.delete_user(payload.id).await?;
    Ok((StatusCode::Ok))    
}

pub async fn delete_user_handler(
    State(user_service) : State<Arc<UserService>>,
    Json(payload): Json<DeleteUser>
)-> Result<(StatusCode), AppError>{
    let user = user_service.delete_user(payload.id).await?;
    Ok((StatusCode::Ok))    
}

pub async fn list_user_handler(
    State(user_service) : State<Arc<UserService>>,
    Json(payload): Json<ListUser>
)-> Result<(StatusCode, Json<ListUser>), AppError>{
    let user = user_service.list_user(payload.limit).await?;

    Ok((StatusCode::Ok, Json(ListUser::from(user))))    
}