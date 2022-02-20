use crate::error::ServiceError;
use crate::types::*;
use super::user_request::{request_get, request_post, request_put};


/// Login a user
pub async fn login_user(login_info: LoginInfoWrapper) -> Result<UserInfoWrapper, ServiceError> {
    request_post::<LoginInfoWrapper, UserInfoWrapper>("/users/login".to_string(), login_info).await
}
