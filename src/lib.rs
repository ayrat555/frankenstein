#[cfg(feature = "async-http-client")]
pub use reqwest;
#[cfg(feature = "http-client")]
pub use ureq;

#[cfg(any(feature = "http-client", feature = "async-http-client"))]
mod api;
#[cfg(any(feature = "telegram-trait", feature = "async-telegram-trait"))]
mod api_traits;
mod error;
pub mod objects;
pub mod parameters;
mod parse_mode;
pub mod response;

#[cfg(any(feature = "http-client", feature = "async-http-client"))]
pub use api::*;
#[cfg(any(feature = "telegram-trait", feature = "async-telegram-trait"))]
pub use api_traits::*;
pub use error::Error;
pub use parse_mode::ParseMode;

/// Default Bot API URL
pub const BASE_API_URL: &str = "https://api.telegram.org/bot";
