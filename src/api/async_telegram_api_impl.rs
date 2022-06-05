use super::DeserializeJson;
use super::Error;
use super::HttpError;
use crate::api_traits::AsyncTelegramApi;
use crate::api_traits::ErrorResponse;
use async_trait::async_trait;
use reqwest::multipart;
use serde_json::Value;
use std::path::PathBuf;
use tokio::fs::File;

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

    pub const fn new_with_client(client: reqwest::Client, api_url: String) -> Self {
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
        let status_code = response.status().as_u16();
        match response.text().await {
            Ok(message) => {
                if status_code == 200 {
                    let success_response: T = Self::parse_json(&message)?;
                    return Ok(success_response);
                }

                let error_response: ErrorResponse = Self::parse_json(&message)?;
                Err(Error::ApiError(error_response))
            }
            Err(e) => {
                let err = Error::DecodeError(format!("Failed to decode response: {:?}", e));
                Err(err)
            }
        }
    }

    fn parse_json<T: serde::de::DeserializeOwned>(body: &str) -> Result<T, Error> {
        match Self::deserialize_json(body) {
            Ok(result) => Ok(result),

            Err(e) => {
                let err = Error::DecodeError(format!("{:?} : {:?}", e, body));
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

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        let message = error.to_string();

        Self::EncodeError(message)
    }
}

impl DeserializeJson for AsyncApi {}

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
        method: &str,
        params: T1,
        files: Vec<(&str, PathBuf)>,
    ) -> Result<T2, Self::Error> {
        let json_string = Self::encode_params(&params)?;
        let json_struct: Value = serde_json::from_str(&json_string).unwrap();
        let file_keys: Vec<&str> = files.iter().map(|(key, _)| *key).collect();
        let files_with_paths: Vec<(String, &str, String)> = files
            .iter()
            .map(|(key, path)| {
                (
                    (*key).to_string(),
                    path.to_str().unwrap(),
                    path.file_name().unwrap().to_str().unwrap().to_string(),
                )
            })
            .collect();

        let mut form = multipart::Form::new();
        for (key, val) in json_struct.as_object().unwrap().iter() {
            if !file_keys.contains(&key.as_str()) {
                let val = match val {
                    &Value::String(ref val) => val.to_string(),
                    other => other.to_string(),
                };

                form = form.text(key.clone(), val.clone());
            }
        }

        for (parameter_name, file_path, file_name) in files_with_paths {
            let file = File::open(file_path).await?;

            let part = multipart::Part::stream(file).file_name(file_name);
            form = form.part(parameter_name, part);
        }

        let url = format!("{}/{}", self.api_url, method);

        let response = self.client.post(url).multipart(form).send().await?;
        let parsed_response: T2 = Self::decode_response(response).await?;

        Ok(parsed_response)
    }
}

#[cfg(test)]
mod async_tests {
    use super::AsyncApi;
    use super::Error;
    use crate::api_params::SendMessageParams;
    use crate::api_traits::ErrorResponse;
    use crate::AsyncTelegramApi;

    #[tokio::test]
    async fn async_send_message_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2746,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618207352,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"text\":\"Hello!\"}}";
        let params = SendMessageParams::builder()
            .chat_id(275808073)
            .text("Hello!")
            .build();
        let _m = mockito::mock("POST", "/sendMessage")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = AsyncApi::new_url(mockito::server_url());

        let response = api.send_message(&params).await.unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[tokio::test]
    async fn send_message_failure() {
        let response_string =
            "{\"ok\":false,\"description\":\"Bad Request: chat not found\",\"error_code\":400}";
        let params = SendMessageParams::builder()
            .chat_id(1)
            .text("Hello!")
            .build();
        let _m = mockito::mock("POST", "/sendMessage")
            .with_status(400)
            .with_body(response_string)
            .create();
        let api = AsyncApi::new_url(mockito::server_url());

        if let Err(Error::ApiError(ErrorResponse {
            ok: false,
            description,
            error_code: 400,
            parameters: None,
        })) = api.send_message(&params).await
        {
            assert_eq!("Bad Request: chat not found".to_string(), description);
        } else {
            panic!("Error was expected but there is none");
        }
    }
}
