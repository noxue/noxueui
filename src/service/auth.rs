use crate::{types::auth::{LoginInfoWrapper, UserInfoWrapper}, error::Error};

use super::request::post;


/// Login a user
pub async fn login(login_info: LoginInfoWrapper) -> Result<UserInfoWrapper, Error> {
    post::<LoginInfoWrapper, UserInfoWrapper>("/users/login".to_string(), login_info).await
}
