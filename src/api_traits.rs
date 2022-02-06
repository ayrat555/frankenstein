use crate::objects::Message;
use serde::{Deserialize, Serialize};

#[cfg(feature = "async-telegram-trait")]
pub mod async_telegram_api;

#[cfg(feature = "telegram-trait")]
pub mod telegram_api;

#[cfg(feature = "async-telegram-trait")]
pub use async_telegram_api::*;

#[cfg(feature = "telegram-trait")]
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
