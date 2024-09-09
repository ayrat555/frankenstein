#[cfg(feature = "async-http-client")]
pub use async_telegram_api_impl::*;
#[cfg(feature = "http-client")]
pub use telegram_api_impl::*;

#[cfg(feature = "async-http-client")]
mod async_telegram_api_impl;
#[cfg(feature = "http-client")]
mod telegram_api_impl;
