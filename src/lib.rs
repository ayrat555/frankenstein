#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(test, allow(dead_code))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "client-reqwest")]
pub use reqwest;
#[cfg(feature = "client-ureq")]
pub use ureq;

pub use self::error::Error;
pub use self::parse_mode::ParseMode;
#[cfg(feature = "trait-async")]
pub use self::trait_async::AsyncTelegramApi;
#[cfg(feature = "trait-sync")]
pub use self::trait_sync::TelegramApi;

#[cfg(feature = "client-reqwest")]
pub mod client_reqwest;
#[cfg(feature = "client-ureq")]
pub mod client_ureq;
mod error;
pub mod games;
pub mod gifts;
pub mod inline_mode;
pub mod input_file;
pub mod input_media;
#[cfg(any(feature = "client-reqwest", feature = "client-ureq"))]
mod json;
mod macros;
pub mod methods;
mod parse_mode;
pub mod passport;
pub mod payments;
pub mod response;
pub mod stickers;
#[cfg(test)]
mod test_json;
#[cfg(feature = "trait-async")]
mod trait_async;
#[cfg(feature = "trait-sync")]
mod trait_sync;
pub mod types;
pub mod updates;

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
