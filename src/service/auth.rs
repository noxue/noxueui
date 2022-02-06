use crate::{
    error::Error,
    types::{
        auth::{ForgetInfo, LoginInfo, PhoneForgetCode, PhoneRegisterCode, RegisterInfo, UserInfo},
        Res,
    },
};

use super::request::{get, post};

/// get register phone code
pub async fn phone_register_code(phone: PhoneRegisterCode) -> Result<Res<bool>, Error> {
    post::<PhoneRegisterCode, Res<bool>>("/users/phone/code", phone).await
}

/// get reset password phone code
pub async fn phone_forget_code(phone: PhoneForgetCode) -> Result<Res<bool>, Error> {
    post::<PhoneForgetCode, Res<bool>>("/users/phone/reset/code", phone).await
}

/// phone register a user
pub async fn register(reg_info: RegisterInfo) -> Result<Res<UserInfo>, Error> {
    post::<RegisterInfo, Res<UserInfo>>("/users/register", reg_info).await
}
/// reset password
pub async fn forget(forget_info: ForgetInfo) -> Result<Res<UserInfo>, Error> {
    post::<ForgetInfo, Res<UserInfo>>("/users/forget", forget_info).await
}

/// Login a user
pub async fn login(login_info: LoginInfo) -> Result<Res<UserInfo>, Error> {
    post::<LoginInfo, Res<UserInfo>>("/users/login", login_info).await
}

pub async fn current() -> Result<Res<UserInfo>, Error> {
    get::<Res<UserInfo>>("/users/me").await
}
