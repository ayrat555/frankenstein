#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(test, allow(dead_code))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "client-reqwest")]
pub use reqwest;
#[cfg(feature = "client-ureq")]
pub use ureq;

pub use self::api_params::*;
pub use self::error::Error;
pub use self::input_file::*;
pub use self::objects::*;
pub use self::parse_mode::ParseMode;
pub use self::response::*;
#[cfg(feature = "trait-async")]
pub use self::trait_async::AsyncTelegramApi;
#[cfg(feature = "trait-sync")]
pub use self::trait_sync::TelegramApi;

pub mod api_params;
#[cfg(feature = "client-reqwest")]
pub mod client_reqwest;
#[cfg(feature = "client-ureq")]
pub mod client_ureq;
mod error;
pub mod input_file;
#[cfg(any(feature = "client-reqwest", feature = "client-ureq"))]
mod json;
mod macros;
pub mod objects;
mod parse_mode;
pub mod response;
#[cfg(test)]
mod test_json;
#[cfg(feature = "trait-async")]
mod trait_async;
#[cfg(feature = "trait-sync")]
mod trait_sync;

/// Default Bot API URL
pub const BASE_API_URL: &str = "https://api.telegram.org/bot";

#[deprecated(
    since = "0.39.0",
    note = "enable the client-reqwest feature and use frankenstein::client_reqwest::Bot instead"
)]
#[doc(hidden)]
pub struct AsyncApi;
#[deprecated(
    since = "0.39.0",
    note = "enable the client-ureq feature and use frankenstein::client_ureq::Bot instead"
)]
#[doc(hidden)]
pub struct Api;
