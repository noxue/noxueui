use crate::{
    error::Error,
    types::{
        auth::{LoginInfo, PhoneRegisterCode, RegisterInfo, UserInfo},
        Res,
    },
};

use super::request::{get, post};

/// get register phone code
pub async fn phone_register_code(phone: PhoneRegisterCode) -> Result<Res<bool>, Error> {
    post::<PhoneRegisterCode, Res<bool>>("/users/phone/code", phone).await
}

/// phone register a user
pub async fn register(reg_info: RegisterInfo) -> Result<Res<UserInfo>, Error> {
    post::<RegisterInfo, Res<UserInfo>>("/users/register", reg_info).await
}

/// Login a user
pub async fn login(login_info: LoginInfo) -> Result<Res<UserInfo>, Error> {
    post::<LoginInfo, Res<UserInfo>>("/users/login", login_info).await
}

pub async fn current() -> Result<Res<UserInfo>, Error> {
    get::<Res<UserInfo>>("/users/me").await
}
