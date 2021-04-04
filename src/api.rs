use crate::api_params::GetUpdatesParams;
use crate::objects::Update;
use reqwest::Client;

static BASE_API_URL: &'static str = "https://api.telegram.org/bot";

pub struct API {
    api_key: String,
    api_url: String,
}

impl API {
    pub fn new(api_key: String) -> Self {
        let api_url = format!("{}{}", BASE_API_URL, api_key);

        Self { api_key, api_url }
    }

    pub async fn get_updates(&self, params: GetUpdatesParams) -> Result<(), reqwest::Error> {
        let url = format!("{}/{}", self.api_url, "getUpdates");

        eprintln!(
            "{:?}",
            Client::new().post(&url).json(&params).send().await? // .json()
                                                                 // .await?
        );

        Ok(())
    }
}
