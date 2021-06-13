// TODO: remove and fix (or allow explicitly on the specific problem)
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_collect,
    clippy::new_without_default,
    clippy::non_ascii_literal,
    clippy::single_match_else,
    clippy::too_many_arguments,
    clippy::unreadable_literal,
    clippy::wildcard_imports
)]

pub mod api;
pub mod api_params;
pub mod objects;

pub use api::*;
pub use api_params::*;
pub use objects::*;

#[cfg(feature = "impl")]
pub mod api_impl;

#[cfg(feature = "impl")]
pub use api_impl::*;
