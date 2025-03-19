//! Raw response objects returned by the Telegram API.
//!
//! Mainly useful when implementing the `TelegramApi` trait.

#![allow(clippy::module_name_repetitions)]

use serde::{Deserialize, Serialize};

use crate::types::Message;

/// Response on successful request.
///
/// `ok` is always `true`.
/// Some methods may return a human-readable `description` of the result.
/// The result of the query can be found in the 'result' field.
///
/// See <https://core.telegram.org/bots/api#making-requests>
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MethodResponse<T> {
    /// Always true
    pub ok: bool,
    pub description: Option<String>,
    pub result: T,
}

/// Error on an unsuccessful request.
///
/// `ok` equals false and the error is explained in the `description`.
/// An Integer `error_code` field is also returned, but its contents are subject to change in the future.
/// Some errors may also have an optional field `parameters` of the type `ResponseParameters`, which can help to automatically handle the error.
///
/// See <https://core.telegram.org/bots/api#making-requests>
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorResponse {
    /// Always false
    pub ok: bool,
    pub description: String,
    /// Contents are subject to change in the future
    pub error_code: u64,
    pub parameters: Option<ResponseParameters>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MessageOrBool {
    Message(Message),
    Bool(bool),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde_with::skip_serializing_none]
pub struct ResponseParameters {
    pub migrate_to_chat_id: Option<i64>,
    pub retry_after: Option<u16>,
}
