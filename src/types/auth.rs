use serde::{Deserialize, Serialize};

use crate::service::request::get_token;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct UserInfo {
    pub username: String,
    pub token: Option<String>,
    pub image: Option<String>,
}

impl UserInfo {
    pub fn is_authenticated(&self) -> bool {
        get_token().is_some()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RegisterInfo {
    pub username: String,
    pub password: String,
    pub password_confirm: String,
    pub code: String,
    pub r#type: String,
}


/// 用于请求验证码
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct PhoneRegisterCode {
    pub phone: String,
}

