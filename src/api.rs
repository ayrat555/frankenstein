use crate::api_params::DeleteWebhookParams;
use crate::api_params::GetUpdatesParams;
use crate::api_params::SendMessageParams;
use crate::api_params::SetWebhookParams;
use crate::objects::Message;
use crate::objects::Update;
use isahc::{prelude::*, Request};
use serde::{Deserialize, Serialize};
use serde_json::Value;

static BASE_API_URL: &'static str = "https://api.telegram.org/bot";

pub struct API {
    api_url: String,
}

#[derive(Debug, Deserialize)]
pub struct MethodResponse<T> {
    ok: bool,
    result: T,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    ok: bool,
    description: String,
    error_code: u64,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Generic(ErrorResponse),
    ApiResult(MethodResponse<T>),
}

impl API {
    pub fn new(api_key: String) -> Self {
        let api_url = format!("{}{}", BASE_API_URL, api_key);

        Self { api_url }
    }

    pub fn get_updates(
        &self,
        params: GetUpdatesParams,
    ) -> Result<ApiResponse<Vec<Update>>, isahc::Error> {
        self.request("getUpdates", params)
    }

    pub fn send_message(
        &self,
        params: SendMessageParams,
    ) -> Result<ApiResponse<Message>, isahc::Error> {
        self.request("sendMessage", params)
    }

    pub fn set_webhook(&self, params: SetWebhookParams) -> Result<ApiResponse<bool>, isahc::Error> {
        self.request("setWebhook", params)
    }

    pub fn delete_webhook(
        &self,
        params: DeleteWebhookParams,
    ) -> Result<ApiResponse<bool>, isahc::Error> {
        self.request("deleteWebhook", params)
    }

    fn request<T1: serde::ser::Serialize, T2: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: T1,
    ) -> Result<T2, isahc::Error> {
        let url = format!("{}/{}", self.api_url, method);
        let json = serde_json::to_string(&params).unwrap();

        let mut response = Request::post(url)
            .header("Content-Type", "application/json")
            .body(json)?
            .send()?;

        let response: T2 = serde_json::from_reader(response.body_mut()).unwrap();

        Ok(response)
    }
}
