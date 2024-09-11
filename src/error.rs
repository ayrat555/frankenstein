use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
#[serde(untagged)]
pub enum Error {
    #[error("Http Error {code}: {message}")]
    Http { code: u16, message: String },
    #[error("Api Error {0:?}")]
    Api(crate::response::ErrorResponse),
    #[error("Decode Error {0}")]
    Decode(String),
    #[error("Encode Error {0}")]
    Encode(String),
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        let message = error.to_string();
        let code = error
            .status()
            .map_or(500, |status_code| status_code.as_u16());
        Self::Http { code, message }
    }
}

#[cfg(feature = "ureq")]
impl From<ureq::Error> for Error {
    fn from(error: ureq::Error) -> Self {
        match error {
            ureq::Error::Status(code, response) => match response.into_string() {
                Ok(message) => match serde_json::from_str(&message) {
                    Ok(json_result) => Self::Api(json_result),
                    Err(_) => Self::Http { code, message },
                },
                Err(_) => Self::Http {
                    code,
                    message: "Failed to decode response".to_string(),
                },
            },
            ureq::Error::Transport(transport_error) => Self::Http {
                message: format!("{transport_error:?}"),
                code: 500,
            },
        }
    }
}
