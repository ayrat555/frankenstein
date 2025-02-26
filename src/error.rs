use crate::response::ErrorResponse;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Api Error {0:?}")]
    Api(ErrorResponse),

    #[cfg(any(feature = "client-reqwest", feature = "client-ureq"))]
    #[error("JSON Decode Error: {source} on {input}")]
    JsonDecode {
        source: serde_json::Error,
        input: String,
    },
    #[cfg(any(feature = "client-reqwest", feature = "client-ureq"))]
    #[error("JSON Encode Error: {source} on {input}")]
    JsonEncode {
        source: serde_json::Error,
        input: String,
    },

    #[cfg(feature = "client-reqwest")]
    #[error("HTTP error: {0}")]
    HttpReqwest(#[source] reqwest::Error),

    #[cfg(feature = "client-ureq")]
    #[error("HTTP error: {0}")]
    HttpUreq(#[from] ureq::Error),
}

impl Error {
    #[allow(irrefutable_let_patterns)] // See https://github.com/rust-lang/rust/issues/72469
    #[cfg(test)]
    #[track_caller]
    pub(crate) fn unwrap_api(self) -> ErrorResponse {
        if let Self::Api(api) = self {
            api
        } else {
            panic!("API Error expected: {self}");
        }
    }
}
