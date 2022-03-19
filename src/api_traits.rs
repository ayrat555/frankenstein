use crate::objects::{Message, ResponseParameters};
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
    /// Always true
    pub ok: bool,
    pub result: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
/// \[…\] an unsuccessful request, `ok` equals false and the error is explained in the `description`.
/// An Integer `error_code` field is also returned, but its contents are subject to change in the future.
/// Some errors may also have an optional field `parameters` of the type `ResponseParameters`, which can help to automatically handle the error.
///
/// See <https://core.telegram.org/bots/api#making-requests>
pub struct ErrorResponse {
    /// Always false
    pub ok: bool,
    pub description: String,
    /// Contents are subject to change in the future
    pub error_code: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ResponseParameters>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditMessageResponse {
    Message(MethodResponse<Message>),
    Bool(MethodResponse<bool>),
}
