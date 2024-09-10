use frankenstein::parameters::SendMessageParams;
use frankenstein::response::ErrorResponse;
use frankenstein::TelegramApi;
use isahc::prelude::*;
use isahc::Request;
use std::path::PathBuf;

static TOKEN: &str = "TOKEN";
static BASE_API_URL: &str = "https://api.telegram.org/bot";
static CHAT_ID: i64 = 1;

pub struct Api {
    pub api_url: String,
}

#[derive(Debug)]
pub enum Error {
    HttpError { code: u16, message: String },
    ApiError(ErrorResponse),
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
        Self::HttpError { code: 500, message }
    }
}

impl From<isahc::Error> for Error {
    fn from(error: isahc::Error) -> Self {
        let message = format!("{error:?}");
        Self::HttpError { code: 500, message }
    }
}

impl TelegramApi for Api {
    type Error = Error;

    fn request<T1: serde::ser::Serialize, T2: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: Option<T1>,
    ) -> Result<T2, Error> {
        let url = format!("{}/{method}", self.api_url);

        let request_builder = Request::post(url).header("Content-Type", "application/json");

        let mut response = match params {
            None => request_builder.body(())?.send()?,
            Some(data) => {
                let json = serde_json::to_string(&data).unwrap();
                request_builder.body(json)?.send()?
            }
        };

        let text = response.text().map_err(|error| Error::HttpError {
            code: 500,
            message: error.to_string(),
        })?;

        let parsed_result: Result<T2, serde_json::Error> = serde_json::from_str(&text);

        parsed_result.map_err(|_| match serde_json::from_str::<ErrorResponse>(&text) {
            Ok(result) => Error::ApiError(result),
            Err(error) => Error::HttpError {
                code: 500,
                message: format!("{error:?}"),
            },
        })
    }

    // isahc doesn't support multipart uploads
    // https://github.com/sagebind/isahc/issues/14
    fn request_with_form_data<T1: serde::ser::Serialize, T2: serde::de::DeserializeOwned>(
        &self,
        _method: &str,
        _params: T1,
        _files: Vec<(&str, PathBuf)>,
    ) -> Result<T2, Error> {
        let message = "isahc doesn't support form data requests".to_string();
        Err(Error::HttpError { code: 500, message })
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
