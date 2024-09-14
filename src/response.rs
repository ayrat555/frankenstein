//! Raw response objects returned by the Telegram API.
//!
//! Mainly useful when implementing the `TelegramApi` trait.

#![allow(clippy::module_name_repetitions)]

use crate::objects::{Message, ResponseParameters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MethodResponse<T> {
    /// Always true
    pub ok: bool,
    pub result: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Error on an unsuccessful request.
///
/// `ok` equals false and the error is explained in the `description`.
/// An Integer `error_code` field is also returned, but its contents are subject to change in the future.
/// Some errors may also have an optional field `parameters` of the type `ResponseParameters`, which can help to automatically handle the error.
///
/// See <https://core.telegram.org/bots/api#making-requests>
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorResponse {
    /// Always false
    pub ok: bool,
    pub description: String,
    /// Contents are subject to change in the future
    pub error_code: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ResponseParameters>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MessageOrBool {
    Message(Message),
    Bool(bool),
}
