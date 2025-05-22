use async_trait::async_trait;
use bon::Builder;
use serde_json::Value;

use crate::input_file::InputFile;
use crate::trait_async::AsyncTelegramApi;
use crate::Error;

/// Asynchronous [`AsyncTelegramApi`] implementation with [`reqwest`]
#[derive(Debug, Clone, Builder)]
#[must_use = "Bot needs to be used in order to be useful"]
pub struct Bot {
    #[builder(into)]
    pub api_url: String,

    #[builder(default = default_client())]
    pub client: reqwest::Client,
}

fn default_client() -> reqwest::Client {
    let client_builder = reqwest::ClientBuilder::new();

    #[cfg(not(target_arch = "wasm32"))]
    let client_builder = client_builder
        .connect_timeout(std::time::Duration::from_secs(10))
        .timeout(std::time::Duration::from_secs(500));

    client_builder.build().unwrap()
}

impl Bot {
    /// Create a new `Bot`. You can use [`Bot::new_url`] or [`Bot::builder`] for more options.
    pub fn new(api_key: &str) -> Self {
        Self::new_url(format!("{}{api_key}", crate::BASE_API_URL))
    }

    /// Create a new `Bot`. You can use [`Bot::builder`] for more options.
    pub fn new_url<S: Into<String>>(api_url: S) -> Self {
        Self::builder().api_url(api_url).build()
    }

    async fn decode_response<Output>(response: reqwest::Response) -> Result<Output, Error>
    where
        Output: serde::de::DeserializeOwned,
    {
        let success = response.status().is_success();
        let message = response.text().await?;
        if success {
            Ok(crate::json::decode(&message)?)
        } else {
            Err(Error::Api(crate::json::decode(&message)?))
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        // Prevent leakage of the bot token as its within the path
        Self::HttpReqwest(error.without_url())
    }
}

// Wasm target need not be `Send` because it is single-threaded
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl AsyncTelegramApi for Bot {
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
        }
        let response = prepared_request.send().await?;
        Self::decode_response(response).await
    }

    #[cfg_attr(target_arch = "wasm32", allow(unused_variables))]
    async fn request_with_form_data<Params, Output>(
        &self,
        method: &str,
        params: Params,
        files: Vec<(std::borrow::Cow<'static, str>, InputFile)>,
    ) -> Result<Output, Self::Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug + std::marker::Send,
        Output: serde::de::DeserializeOwned,
    {
        let json_string = crate::json::encode(&params)?;
        let json_struct: serde_json::Map<String, Value> =
            serde_json::from_str(&json_string).unwrap();
        let file_keys: Vec<&str> = files.iter().map(|(key, _)| key.as_ref()).collect();

        let mut form = reqwest::multipart::Form::new();
        for (key, val) in json_struct {
            if !file_keys.contains(&key.as_str()) {
                let val = match val {
                    Value::String(val) => val,
                    other => other.to_string(),
                };
                form = form.text(key, val);
            }
        }

        for (parameter_name, input_file) in files {
            let part = match input_file {
                InputFile::Bytes { bytes, file_name } => {
                    // The reqwest::multipart stuff requires 'static which we can not grant here.
                    // So we provide owned data by cloning it.
                    reqwest::multipart::Part::bytes(bytes).file_name(file_name)
                }

                #[cfg(not(target_arch = "wasm32"))]
                InputFile::Path(path) => reqwest::multipart::Part::file(path)
                    .await
                    .map_err(crate::Error::ReadFile)?,
            };
            form = form.part(parameter_name, part);
        }

        let url = format!("{}/{method}", self.api_url);

        let response = self.client.post(url).multipart(form).send().await?;
        Self::decode_response(response).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::methods::SendMessageParams;

    #[tokio::test]
    async fn async_send_message_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2746,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618207352,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"text\":\"Hello!\"}}";
        let params = SendMessageParams::builder()
            .chat_id(275808073)
            .text("Hello!")
            .build();
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/sendMessage")
            .with_status(200)
            .with_body(response_string)
            .create_async()
            .await;
        let api = Bot::new_url(server.url());

        let response = api.send_message(&params).await.unwrap();
        mock.assert();
        drop(server);

        crate::test_json::assert_json_str(&response, response_string);
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
        let mock = server
            .mock("POST", "/sendMessage")
            .with_status(400)
            .with_body(response_string)
            .create_async()
            .await;
        let api = Bot::new_url(server.url());

        let error = api.send_message(&params).await.unwrap_err().unwrap_api();
        mock.assert();
        drop(server);

        assert_eq!(error.description, "Bad Request: chat not found");
        assert_eq!(error.error_code, 400);
        assert_eq!(error.parameters, None);
        assert!(!error.ok);
    }
}
