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
#[non_exhaustive]
#[serde(untagged)]
pub enum Error {
    #[error("Http Error {code}: {message}")]
    Http { code: u16, message: String },
    #[error("Api Error {0:?}")]
    Api(ErrorResponse),
    #[error("Decode Error {0}")]
    Decode(String),
    #[error("Encode Error {0}")]
    Encode(String),
}
