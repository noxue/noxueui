use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod auth;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(default)]
#[serde(bound = "T: DeserializeOwned")]
pub struct Res<T>
where
    T: Serialize + Default + Debug,
{
    pub code: i32,
    pub msg: String,
    pub data: T,
}
