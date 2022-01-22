use crate::traits::ErrorResponse;
use serde::{Deserialize, Serialize};
use ureq::Response;

pub mod telegram_api_impl;
pub use telegram_api_impl::*;

static BASE_API_URL: &str = "https://api.telegram.org/bot";

#[derive(PartialEq, Debug, Clone)]
pub struct Api {
    pub api_url: String,
}

impl Api {
    pub fn new(api_key: &str) -> Self {
        let api_url = format!("{}{}", BASE_API_URL, api_key);
        Self { api_url }
    }

    pub const fn new_url(api_url: String) -> Self {
        Self { api_url }
    }

    pub fn encode_params<T: serde::ser::Serialize + std::fmt::Debug>(
        params: &T,
    ) -> Result<String, Error> {
        serde_json::to_string(params)
            .map_err(|e| Error::EncodeError(format!("{:?} : {:?}", e, params)))
    }

    pub fn decode_response<T: serde::de::DeserializeOwned>(response: Response) -> Result<T, Error> {
        match response.into_string() {
            Ok(message) => {
                let json_result: Result<T, serde_json::Error> = serde_json::from_str(&message);

                match json_result {
                    Ok(result) => Ok(result),
                    Err(e) => {
                        let err = Error::DecodeError(format!("{:?} : {:?}", e, &message));
                        Err(err)
                    }
                }
            }
            Err(e) => {
                let err = Error::DecodeError(format!("Failed to decode response: {:?}", e));
                Err(err)
            }
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Error {
    HttpError(HttpError),
    ApiError(ErrorResponse),
    DecodeError(String),
    EncodeError(String),
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct HttpError {
    pub code: u16,
    pub message: String,
}
