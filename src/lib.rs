#[cfg(any(feature = "http-client", feature = "async-http-client"))]
pub mod api;

#[cfg(any(feature = "telegram-trait", feature = "async-telegram-trait"))]
pub mod api_traits;

#[cfg(any(feature = "http-client", feature = "async-http-client"))]
pub use api::*;

#[cfg(any(feature = "telegram-trait", feature = "async-telegram-trait"))]
pub use api_traits::*;

#[doc(hidden)]
#[cfg(feature = "async-http-client")]
pub use reqwest;

#[doc(hidden)]
#[cfg(feature = "http-client")]
pub use ureq;

pub mod api_params;
pub mod objects;
mod parse_mode;

pub use api_params::*;
pub use objects::*;
pub use parse_mode::*;
