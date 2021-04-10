use crate::api_params::GetUpdatesParams;
use crate::api_params::SendMessageParams;
use crate::objects::Message;
use crate::objects::Update;
use isahc::{prelude::*, Request};
use serde::{Deserialize, Serialize};
use serde_json::Value;

static BASE_API_URL: &'static str = "https://api.telegram.org/bot";

pub struct API {
    api_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse1 {
    ok: bool,
    result: Vec<Update>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse2 {
    ok: bool,
    result: Message,
}

impl API {
    pub fn new(api_key: String) -> Self {
        let api_url = format!("{}{}", BASE_API_URL, api_key);

        Self { api_url }
    }

    pub fn get_updates(&self, params: GetUpdatesParams) -> Result<ApiResponse1, isahc::Error> {
        let url = format!("{}/{}", self.api_url, "getUpdates");

        let json = serde_json::to_string(&params).unwrap();
        let mut response = isahc::post(url, json)?;

        let response: ApiResponse1 = serde_json::from_reader(response.body_mut()).unwrap();

        Ok(response)
    }

    pub fn send_message(&self, params: SendMessageParams) -> Result<Value, isahc::Error> {
        let url = format!("{}/{}", self.api_url, "sendMessage");

        let json = serde_json::to_string(&params).unwrap();

        println!("json: {}", json);
        // let mut response = isahc::post(url, json).header("Content-Type", "application/json")

        let mut response = Request::post(url)
            .header("Content-Type", "application/json")
            .body(json)?
            .send()?;

        let response: Value = serde_json::from_reader(response.body_mut()).unwrap();

        Ok(response)
    }
}
