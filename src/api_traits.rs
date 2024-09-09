#[cfg(feature = "async-telegram-trait")]
mod async_telegram_api;
#[cfg(feature = "telegram-trait")]
mod telegram_api;

#[cfg(feature = "async-telegram-trait")]
pub use async_telegram_api::*;
#[cfg(feature = "telegram-trait")]
pub use telegram_api::*;
