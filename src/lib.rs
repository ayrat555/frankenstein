// TODO: remove and fix (or allow explicitly on the specific problem)
#![allow(
    clippy::missing_errors_doc,
    clippy::single_match_else,
    clippy::struct_excessive_bools,
    clippy::unreadable_literal,

    // from clippy::nursery
    // clippy::derive_partial_eq_without_eq,
    // clippy::option_if_let_else,
    // clippy::significant_drop_tightening,
    // clippy::use_self,
)]

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
