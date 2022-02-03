use dotenv_codegen::dotenv;
use gloo::storage::{LocalStorage, Storage};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::Error;
use crate::types::Res;
use crate::types::auth::UserInfo;

const API_ROOT: &str = dotenv!("API_ROOT");
const TOKEN_KEY: &str = "noxue.com.token";

lazy_static! {
    /// Jwt token read from local storage.
    pub static ref TOKEN: RwLock<Option<String>> = {
        if let Ok(token) = LocalStorage::get(TOKEN_KEY) {
            RwLock::new(Some(token))
        } else {
            RwLock::new(None)
        }
    };
}

/// 把 jwt token 保存在 LocalStorage 中
pub fn set_token(token: Option<String>) {
    if let Some(t) = token.clone() {
        LocalStorage::set(TOKEN_KEY, t).expect("failed to set");
    } else {
        LocalStorage::delete(TOKEN_KEY);
    }
    let mut token_lock = TOKEN.write();
    *token_lock = token;
}

/// 获取 jwt token.
pub fn get_token() -> Option<String> {
    let token_lock = TOKEN.read();
    token_lock.clone()
}

/// 用于创建所有类型的请求函数
pub async fn request<B, T>(method: reqwest::Method, url: &str, body: B) -> Result<T, Error>
where
    T: Serialize + DeserializeOwned + 'static + std::fmt::Debug + Default,
    B: Serialize + std::fmt::Debug,
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let url = format!("{}{}", API_ROOT, url);
    let mut builder = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");
    if let Some(token) = get_token() {
        builder = builder.bearer_auth(token);
    }

    if allow_body {
        builder = builder.json(&body);
    }

    let response = builder.send().await;

    if let Ok(res) = response {
        
        if res.status().is_success() {
          
            let data = res.json::<T>().await;
            if let Ok(data) = data {
                log::debug!("Response: {:?}", data);
                Ok(data)
            } else {
                log::debug!("Response err: {:?}", data);
                Err(Error::DeserializeError)
            }
        } else {
            match res.status().as_u16() {
                401 => Err(Error::Unauthorized),
                403 => Err(Error::Forbidden),
                404 => Err(Error::NotFound),
                500 => Err(Error::InternalServerError),
                _ => Err(Error::RequestError),
            }
        }
    } else {
        Err(Error::RequestError)
    }
}

/// Delete request
pub async fn delete<T>(url: &str) -> Result<T, Error>
where
    T: Serialize + DeserializeOwned + 'static + std::fmt::Debug + Default,
{
    request(reqwest::Method::DELETE, url, ()).await
}

/// Get request
pub async fn get<T>(url: &str) -> Result<T, Error>
where
    T: Serialize + DeserializeOwned + 'static + std::fmt::Debug + Default,
{
    request(reqwest::Method::GET, url, ()).await
}

/// Post request with a body
pub async fn post<B, T>(url: &str, body: B) -> Result<T, Error>
where
    T: Serialize + DeserializeOwned + 'static + std::fmt::Debug + Default,
    B: Serialize + std::fmt::Debug,
{
    request(reqwest::Method::POST, url, body).await
}

/// Put request with a body
pub async fn put<B, T>(url: &str, body: B) -> Result<T, Error>
where
    T: Serialize + DeserializeOwned + 'static + std::fmt::Debug + Default,
    B: Serialize + std::fmt::Debug,
{
    request(reqwest::Method::PUT, url, body).await
}

/// 用户设置分页
pub fn limit(count: u32, p: u32) -> String {
    let offset = if p > 0 { p * count } else { 0 };
    format!("limit={}&offset={}", count, offset)
}
