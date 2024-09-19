use std::path::PathBuf;
use std::time::Duration;

use async_trait::async_trait;
use bon::Builder;
use reqwest::multipart;
use serde_json::Value;
use tokio::fs::File;

use crate::trait_async::AsyncTelegramApi;
use crate::Error;

/// Asynchronous [`AsyncTelegramApi`] client implementation with [`reqwest`].
#[derive(Debug, Clone, Builder)]
#[must_use = "API needs to be used in order to be useful"]
pub struct AsyncApi {
    #[builder(into)]
    pub api_url: String,

    #[builder(
        default = reqwest::ClientBuilder::new()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(500))
            .build()
            .unwrap()
    )]
    pub client: reqwest::Client,
}

impl AsyncApi {
    /// Create a new `AsyncApi`. You can use [`AsyncApi::new_url`] or [`AsyncApi::builder`] for more options.
    pub fn new(api_key: &str) -> Self {
        Self::new_url(format!("{}{api_key}", crate::BASE_API_URL))
    }

    /// Create a new `AsyncApi`. You can use [`AsyncApi::builder`] for more options.
    pub fn new_url<S: Into<String>>(api_url: S) -> Self {
        Self::builder().api_url(api_url).build()
    }

    pub async fn decode_response<Output>(response: reqwest::Response) -> Result<Output, Error>
    where
        Output: serde::de::DeserializeOwned,
    {
        let status_code = response.status().as_u16();
        match response.text().await {
            Ok(message) => {
                if status_code == 200 {
                    Ok(crate::json::decode(&message)?)
                } else {
                    Err(Error::Api(crate::json::decode(&message)?))
                }
            }
            Err(error) => Err(Error::Decode(format!("{error:?}"))),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        let message = error.to_string();
        let code = error
            .status()
            .map_or(500, |status_code| status_code.as_u16());
        Self::Http { code, message }
    }
}

#[async_trait]
impl AsyncTelegramApi for AsyncApi {
    type Error = Error;

    async fn request<Params, Output>(
        &self,
        method: &str,
        params: Option<Params>,
    ) -> Result<Output, Self::Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug + std::marker::Send,
        Output: serde::de::DeserializeOwned,
    {
        let url = format!("{}/{method}", self.api_url);
        let mut prepared_request = self
            .client
            .post(url)
            .header("Content-Type", "application/json");
        if let Some(params) = params {
            let json_string = crate::json::encode(&params)?;
            prepared_request = prepared_request.body(json_string);
        };
        let response = prepared_request.send().await?;
        Self::decode_response(response).await
    }

    async fn request_with_form_data<Params, Output>(
        &self,
        method: &str,
        params: Params,
        files: Vec<(&str, PathBuf)>,
    ) -> Result<Output, Self::Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug + std::marker::Send,
        Output: serde::de::DeserializeOwned,
    {
        let json_string = crate::json::encode(&params)?;
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
        for (key, val) in json_struct.as_object().unwrap() {
            if !file_keys.contains(&key.as_str()) {
                let val = match val {
                    Value::String(val) => val.to_string(),
                    other => other.to_string(),
                };

                form = form.text(key.clone(), val);
            }
        }

        for (parameter_name, file_path, file_name) in files_with_paths {
            let file = File::open(file_path)
                .await
                .map_err(|error| Error::Encode(error.to_string()))?;
            let part = multipart::Part::stream(file).file_name(file_name);
            form = form.part(parameter_name, part);
        }

        let url = format!("{}/{method}", self.api_url);

        let response = self.client.post(url).multipart(form).send().await?;
        Self::decode_response(response).await
    }
}

#[cfg(test)]
mod async_tests {
    use super::*;
    use crate::api_params::SendMessageParams;
    use crate::json;

    #[tokio::test]
    async fn async_send_message_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2746,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618207352,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"text\":\"Hello!\"}}";
        let params = SendMessageParams::builder()
            .chat_id(275808073)
            .text("Hello!")
            .build();
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("POST", "/sendMessage")
            .with_status(200)
            .with_body(response_string)
            .create_async()
            .await;
        let api = AsyncApi::new_url(server.url());

        let response = api.send_message(&params).await.unwrap();
        json::assert_str(&response, response_string);
    }

    #[tokio::test]
    async fn send_message_failure() {
        let response_string =
            "{\"ok\":false,\"description\":\"Bad Request: chat not found\",\"error_code\":400}";
        let params = SendMessageParams::builder()
            .chat_id(1)
            .text("Hello!")
            .build();
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("POST", "/sendMessage")
            .with_status(400)
            .with_body(response_string)
            .create_async()
            .await;
        let api = AsyncApi::new_url(server.url());
        let error = api.send_message(&params).await.unwrap_err().unwrap_api();
        assert_eq!(error.description, "Bad Request: chat not found");
        assert_eq!(error.error_code, 400);
        assert_eq!(error.parameters, None);
        assert!(!error.ok);
    }
}
