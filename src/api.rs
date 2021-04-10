use crate::api_params::GetUpdatesParams;
use crate::objects::Update;
use serde::{Deserialize, Serialize};

static BASE_API_URL: &'static str = "https://api.telegram.org/bot";

pub struct API {
    api_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    ok: bool,
    result: Vec<Update>,
}

impl API {
    pub fn new(api_key: String) -> Self {
        let api_url = format!("{}{}", BASE_API_URL, api_key);

        Self { api_url }
    }

    pub fn get_updates(&self, params: GetUpdatesParams) -> Result<ApiResponse, isahc::Error> {
        let url = format!("{}/{}", self.api_url, "getUpdates");

        let json = serde_json::to_string(&params).unwrap();
        let mut response = isahc::post(url, json)?;

        let response: ApiResponse = serde_json::from_reader(response.body_mut()).unwrap();

        Ok(response)
    }
}
