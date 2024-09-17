use serde::{Deserialize, Serialize};

use crate::response::ErrorResponse;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
#[serde(untagged)]
pub enum Error {
    #[error("Http Error {code}: {message}")]
    Http { code: u16, message: String },
    #[error("Api Error {0:?}")]
    Api(ErrorResponse),
    #[error("Decode Error {0}")]
    Decode(String),
    #[error("Encode Error {0}")]
    Encode(String),
}
