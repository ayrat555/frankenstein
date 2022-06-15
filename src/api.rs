use crate::api_traits::ErrorResponse;
use serde::{Deserialize, Serialize};

#[cfg(feature = "async-http-client")]
pub mod async_telegram_api_impl;
#[cfg(feature = "async-http-client")]
pub use async_telegram_api_impl::*;

#[cfg(feature = "http-client")]
pub mod telegram_api_impl;
#[cfg(feature = "http-client")]
pub use telegram_api_impl::*;

pub static BASE_API_URL: &str = "https://api.telegram.org/bot";

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, thiserror::Error)]
#[serde(untagged)]
pub enum Error {
    #[error("{0}")]
    HttpError(HttpError),
    #[error("Api Error {0:?}")]
    ApiError(ErrorResponse),
    #[error("Decode Error {0}")]
    DecodeError(String),
    #[error("Encode Error {0}")]
    EncodeError(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, thiserror::Error)]
#[error("Http Error {code}: {message}")]
pub struct HttpError {
    pub code: u16,
    pub message: String,
}
