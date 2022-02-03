use std::collections::HashMap;

use crate::{types::auth::{LoginInfo, UserInfo}, error::Error};

use super::request::{post, get};


/// Login a user
pub async fn login(login_info: LoginInfo) -> Result<UserInfo, Error> {
    post::<LoginInfo, UserInfo>("/users/login", login_info).await
}


pub async fn current()->Result<HashMap<String,String>, Error>{
    get::<HashMap<String,String>>("/user").await
}
