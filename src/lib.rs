#[cfg(feature = "reqwest")]
pub use reqwest;
#[cfg(feature = "ureq")]
pub use ureq;

#[cfg(feature = "async-http-client")]
mod client_reqwest;
#[cfg(feature = "http-client")]
mod client_ureq;
mod error;
pub mod objects;
pub mod parameters;
mod parse_mode;
pub mod response;
#[cfg(feature = "async-telegram-trait")]
mod trait_async;
#[cfg(feature = "telegram-trait")]
mod trait_sync;

#[cfg(feature = "async-http-client")]
pub use client_reqwest::*;
#[cfg(feature = "http-client")]
pub use client_ureq::*;
pub use error::Error;
pub use parse_mode::ParseMode;
#[cfg(feature = "async-telegram-trait")]
pub use trait_async::AsyncTelegramApi;
#[cfg(feature = "telegram-trait")]
pub use trait_sync::TelegramApi;

/// Default Bot API URL
pub const BASE_API_URL: &str = "https://api.telegram.org/bot";
