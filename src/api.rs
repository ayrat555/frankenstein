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

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Error {
    HttpError(HttpError),
    ApiError(ErrorResponse),
    DecodeError(String),
    EncodeError(String),
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct HttpError {
    pub code: u16,
    pub message: String,
}
