use crate::objects::Message;
use serde::{Deserialize, Serialize};

pub mod async_telegram_api;
pub mod telegram_api;

pub use async_telegram_api::*;
pub use telegram_api::*;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct MethodResponse<T> {
    pub ok: bool,
    pub result: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub ok: bool,
    pub description: String,
    pub error_code: u64,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditMessageResponse {
    Message(MethodResponse<Message>),
    Bool(MethodResponse<bool>),
}
