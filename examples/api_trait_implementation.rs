use std::path::PathBuf;

use frankenstein::api_params::SendMessageParams;
use frankenstein::response::ErrorResponse;
use frankenstein::TelegramApi;
use isahc::prelude::*;
use isahc::Request;

static TOKEN: &str = "TOKEN";
static BASE_API_URL: &str = "https://api.telegram.org/bot";
static CHAT_ID: i64 = 1;

pub struct Api {
    pub api_url: String,
}

#[derive(Debug)]
pub enum Error {
    Http { code: u16, message: String },
    Api(ErrorResponse),
}

impl Api {
    #[must_use]
    pub fn new(api_key: &str) -> Self {
        let api_url = format!("{BASE_API_URL}{api_key}");
        Self { api_url }
    }

    #[must_use]
    pub const fn new_url(api_url: String) -> Self {
        Self { api_url }
    }
}

impl From<isahc::http::Error> for Error {
    fn from(error: isahc::http::Error) -> Self {
        let message = format!("{error:?}");
        Self::Http { code: 500, message }
    }
}

impl From<isahc::Error> for Error {
    fn from(error: isahc::Error) -> Self {
        let message = format!("{error:?}");
        Self::Http { code: 500, message }
    }
}

impl TelegramApi for Api {
    type Error = Error;

    fn request<Params, Output>(
        &self,
        method: &str,
        params: Option<Params>,
    ) -> Result<Output, Self::Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug,
        Output: serde::de::DeserializeOwned,
    {
        let url = format!("{}/{method}", self.api_url);

        let request_builder = Request::post(url).header("Content-Type", "application/json");

        let mut response = match params {
            None => request_builder.body(())?.send()?,
            Some(data) => {
                let json = serde_json::to_string(&data).unwrap();
                request_builder.body(json)?.send()?
            }
        };

        let text = response.text().map_err(|error| Error::Http {
            code: 500,
            message: format!("{error:?}"),
        })?;

        serde_json::from_str(&text).map_err(|_| {
            match serde_json::from_str::<ErrorResponse>(&text) {
                Ok(result) => Error::Api(result),
                Err(error) => Error::Http {
                    code: 500,
                    message: format!("{error:?}"),
                },
            }
        })
    }

    // isahc doesn't support multipart uploads
    // https://github.com/sagebind/isahc/issues/14
    fn request_with_form_data<Params, Output>(
        &self,
        _method: &str,
        _params: Params,
        _files: Vec<(&str, PathBuf)>,
    ) -> Result<Output, Self::Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug,
        Output: serde::de::DeserializeOwned,
    {
        let message = "isahc doesn't support form data requests".to_string();
        Err(Error::Http { code: 500, message })
    }
}

fn main() {
    let api = Api::new(TOKEN);

    let params = SendMessageParams::builder()
        .chat_id(CHAT_ID)
        .text("Hello!")
        .build();

    let result = api.send_message(&params);

    eprintln!("{result:?}");
}
