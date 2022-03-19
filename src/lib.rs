// TODO: remove and fix (or allow explicitly on the specific problem)
#![allow(
    clippy::large_enum_variant,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_collect,
    clippy::new_without_default,
    clippy::non_ascii_literal,
    clippy::single_match_else,
    clippy::struct_excessive_bools,
    clippy::too_many_arguments,
    clippy::unreadable_literal,
    clippy::use_self,
    clippy::wildcard_imports
)]

#[cfg(any(feature = "http-client", feature = "async-http-client"))]
pub mod api;

#[cfg(any(feature = "telegram-trait", feature = "async-telegram-trait"))]
pub mod api_traits;

#[cfg(any(feature = "http-client", feature = "async-http-client"))]
pub use api::*;

#[cfg(any(feature = "telegram-trait", feature = "async-telegram-trait"))]
pub use api_traits::*;

pub mod api_params;
pub mod objects;
mod parse_mode;

pub use api_params::*;
pub use objects::*;
pub use parse_mode::*;
