use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct UserInfo {
    pub username: String,
    pub token: Option<String>,
    pub image: Option<String>,
}

impl UserInfo {
    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
    pub r#type: String,
}
