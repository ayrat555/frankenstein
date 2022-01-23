use super::Error;
use super::HttpError;
use crate::api_traits::AsyncTelegramApi;
use async_trait::async_trait;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AsyncApi {
    pub api_url: String,
    client: reqwest::Client,
}

impl AsyncApi {
    pub fn new(api_key: &str) -> Self {
        let api_url = format!("{}{}", super::BASE_API_URL, api_key);
        let client = reqwest::Client::new();
        Self { api_url, client }
    }

    pub fn new_url(api_url: String) -> Self {
        let client = reqwest::Client::new();
        Self { api_url, client }
    }

    pub fn encode_params<T: serde::ser::Serialize + std::fmt::Debug>(
        params: &T,
    ) -> Result<String, Error> {
        serde_json::to_string(params)
            .map_err(|e| Error::EncodeError(format!("{:?} : {:?}", e, params)))
    }

    pub async fn decode_response<T: serde::de::DeserializeOwned>(
        response: reqwest::Response,
    ) -> Result<T, Error> {
        match response.text().await {
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

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        let message = error.to_string();
        let code = if let Some(status_code) = error.status() {
            status_code.as_u16()
        } else {
            500
        };

        let error = HttpError { code, message };
        Self::HttpError(error)
    }
}

#[async_trait]
impl AsyncTelegramApi for AsyncApi {
    type Error = Error;

    async fn request<
        T1: serde::ser::Serialize + std::fmt::Debug + std::marker::Send,
        T2: serde::de::DeserializeOwned,
    >(
        &self,
        method: &str,
        params: Option<T1>,
    ) -> Result<T2, Self::Error> {
        let url = format!("{}/{}", self.api_url, method);

        let mut prepared_request = self
            .client
            .post(url)
            .header("Content-Type", "application/json");

        prepared_request = if let Some(data) = params {
            let json_string = Self::encode_params(&data)?;

            prepared_request.body(json_string)
        } else {
            prepared_request
        };

        let response = prepared_request.send().await?;
        let parsed_response: T2 = Self::decode_response(response).await?;

        Ok(parsed_response)
    }

    async fn request_with_form_data<
        T1: serde::ser::Serialize + std::fmt::Debug + std::marker::Send,
        T2: serde::de::DeserializeOwned,
    >(
        &self,
        _method: &str,
        _params: T1,
        _files: Vec<(&str, PathBuf)>,
    ) -> Result<T2, Self::Error> {
        let error = HttpError {
            code: 500,
            message: "doesn't support form data requests yet".to_string(),
        };

        Err(Error::HttpError(error))
    }
}
