use crate::{
    error::Error,
    types::{
        auth::{LoginInfo, UserInfo},
        Res,
    },
};

use super::request::{get, post};

/// Login a user
pub async fn login(login_info: LoginInfo) -> Result<Res<UserInfo>, Error> {
    post::<LoginInfo, Res<UserInfo>>("/users/login", login_info).await
}

pub async fn current() -> Result<Res<UserInfo>, Error> {
    get::<Res<UserInfo>>("/user").await
}
