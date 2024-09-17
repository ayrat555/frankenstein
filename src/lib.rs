#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "async-http-client")]
pub use reqwest;
#[cfg(feature = "http-client")]
pub use ureq;

pub use self::api_params::*;
#[cfg(feature = "async-http-client")]
pub use self::client_reqwest::*;
#[cfg(feature = "http-client")]
pub use self::client_ureq::*;
pub use self::error::Error;
pub use self::objects::*;
pub use self::parse_mode::ParseMode;
pub use self::response::*;
#[cfg(feature = "async-telegram-trait")]
pub use self::trait_async::AsyncTelegramApi;
#[cfg(feature = "telegram-trait")]
pub use self::trait_sync::TelegramApi;

pub mod api_params;
#[cfg(feature = "async-http-client")]
mod client_reqwest;
#[cfg(feature = "http-client")]
mod client_ureq;
mod error;
#[cfg(any(feature = "http-client", feature = "async-http-client"))]
mod json;
mod macros;
pub mod objects;
mod parse_mode;
pub mod response;
#[cfg(feature = "async-telegram-trait")]
mod trait_async;
#[cfg(feature = "telegram-trait")]
mod trait_sync;

/// Default Bot API URL
pub const BASE_API_URL: &str = "https://api.telegram.org/bot";
