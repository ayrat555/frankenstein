#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(test, allow(dead_code))]

#[cfg(feature = "client-reqwest")]
pub use reqwest;
#[cfg(feature = "client-ureq")]
pub use ureq;

pub use self::api_params::*;
#[cfg(feature = "client-reqwest")]
pub use self::client_reqwest::*;
#[cfg(feature = "client-ureq")]
pub use self::client_ureq::*;
pub use self::error::Error;
pub use self::objects::*;
pub use self::parse_mode::ParseMode;
pub use self::response::*;
#[cfg(feature = "trait-async")]
pub use self::trait_async::AsyncTelegramApi;
#[cfg(feature = "trait-sync")]
pub use self::trait_sync::TelegramApi;

pub mod api_params;
#[cfg(feature = "client-reqwest")]
mod client_reqwest;
#[cfg(feature = "client-ureq")]
mod client_ureq;
mod error;
#[cfg(any(test, feature = "client-ureq", feature = "client-reqwest"))]
mod json;
mod macros;
pub mod objects;
mod parse_mode;
pub mod response;
#[cfg(feature = "trait-async")]
mod trait_async;
#[cfg(feature = "trait-sync")]
mod trait_sync;

/// Default Bot API URL
pub const BASE_API_URL: &str = "https://api.telegram.org/bot";
