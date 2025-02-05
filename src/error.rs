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

    #[error("Read File Error: {0}")]
    ReadFile(#[source] std::io::Error),

    #[cfg(all(feature = "client-reqwest", target_arch = "wasm32"))]
    #[error("Handling files is not yet supported in Wasm due to missing form_data / attachment support. Pull Request welcome!")]
    WasmHasNoFileSupportYet,

    #[cfg(feature = "client-reqwest")]
    #[error("HTTP error: {0}")]
    HttpReqwest(#[source] reqwest::Error),

    #[cfg(feature = "client-ureq")]
    #[error("HTTP error: {0}")]
    HttpUreq(#[from] ureq::Error),
}

impl Error {
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
