use crate::error::ServiceError;
use crate::types::*;
use super::user_request::{request_get, request_post, request_put};


/// Login a user
pub async fn login_user(login_info: LoginInfoWrapper) -> Result<UserInfoWrapper, ServiceError> {
    request_post::<LoginInfoWrapper, UserInfoWrapper>("/users/login".to_string(), login_info).await
}

pub async fn current_user() -> Result<UserInfoWrapper, ServiceError> { 
    request_get::<UserInfoWrapper>("/user".to_string()).await
}
/// Register a new user
pub async fn register(register_info: RegisterInfoWrapper) -> Result<UserInfoWrapper, ServiceError> {
    request_post::<RegisterInfoWrapper, UserInfoWrapper>("/users".to_string(), register_info).await
}

/// Save info of current user
pub async fn save(user_update_info: UserUpdateInfoWrapper) -> Result<UserInfoWrapper, ServiceError> {
    request_put::<UserUpdateInfoWrapper, UserInfoWrapper>("/user".to_string(), user_update_info)
        .await
}