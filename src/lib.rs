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

pub mod api;
pub mod api_params;
pub mod api_traits;
pub mod objects;

pub use api::*;
pub use api_params::*;
pub use api_traits::*;
pub use objects::*;

// #[cfg(feature = "http-client")]
// pub mod api_impl;

// #[cfg(feature = "http-client")]
// pub use api_impl::*;
