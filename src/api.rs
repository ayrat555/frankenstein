use crate::api_params::DeleteWebhookParams;
use crate::api_params::GetUpdatesParams;
use crate::api_params::SendMessageParams;
use crate::api_params::SetWebhookParams;
use crate::objects::Message;
use crate::objects::Update;
use crate::objects::WebhookInfo;
use isahc::{prelude::*, Request};
use serde::{Deserialize, Serialize};
use serde_json::Value;

static BASE_API_URL: &'static str = "https://api.telegram.org/bot";

#[derive(PartialEq, Debug)]
pub struct API {
    api_url: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct MethodResponse<T> {
    pub ok: bool,
    pub result: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub ok: bool,
    pub description: String,
    pub error_code: u64,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Error(ErrorResponse),
    ApiResult(MethodResponse<T>),
}

impl API {
    pub fn new(api_key: String) -> Self {
        let api_url = format!("{}{}", BASE_API_URL, api_key);

        Self { api_url }
    }

    pub fn new_url(api_url: String) -> Self {
        Self { api_url }
    }

    pub fn get_updates(
        &self,
        params: GetUpdatesParams,
    ) -> Result<ApiResponse<Vec<Update>>, isahc::Error> {
        self.request("getUpdates", Some(params))
    }

    pub fn send_message(
        &self,
        params: SendMessageParams,
    ) -> Result<ApiResponse<Message>, isahc::Error> {
        self.request("sendMessage", Some(params))
    }

    pub fn set_webhook(&self, params: SetWebhookParams) -> Result<ApiResponse<bool>, isahc::Error> {
        self.request("setWebhook", Some(params))
    }

    pub fn delete_webhook(
        &self,
        params: DeleteWebhookParams,
    ) -> Result<ApiResponse<bool>, isahc::Error> {
        self.request("deleteWebhook", Some(params))
    }

    pub fn get_webhook_info(&self) -> Result<WebhookInfo, isahc::Error> {
        self.request_without_body("getWebhookInfo")
    }

    fn request_without_body<T2: serde::de::DeserializeOwned>(
        &self,
        method: &str,
    ) -> Result<T2, isahc::Error> {
        let params: Option<()> = None;

        self.request(method, params)
    }

    fn request<T1: serde::ser::Serialize, T2: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: Option<T1>,
    ) -> Result<T2, isahc::Error> {
        let url = format!("{}/{}", self.api_url, method);

        let request_builder = Request::post(url).header("Content-Type", "application/json");

        let mut response = match params {
            None => request_builder.body(())?.send()?,
            Some(data) => {
                let json = serde_json::to_string(&data).unwrap();
                request_builder.body(json)?.send()?
            }
        };

        let response: T2 = serde_json::from_reader(response.body_mut()).unwrap();

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_params::*;
    use httpmock::Method::POST;
    use httpmock::MockServer;

    #[test]
    fn new_sets_correct_url() {
        let api = API::new("hey".to_string());

        assert_eq!(
            API {
                api_url: "https://api.telegram.org/bothey".to_string()
            },
            api
        );
    }

    #[test]
    fn get_updates_success() {
        let response_string = "{\"ok\":true,\"result\":[{\"update_id\":379656753,\"message\":{\"message_id\":2741,\"from\":{\"id\":275808073,\"is_bot\":false,\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\",\"username\":\"Ayrat555\",\"language_code\":\"en\"},\"date\":1618149703,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"text\":\"dsaf\"}}]}";
        let mut params = GetUpdatesParams::new();
        params.set_allowed_updates(Some(vec!["message".to_string()]));

        let api = api_with_mock("/getUpdates", response_string);

        let response = api.get_updates(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();

        assert_eq!(response_string, json);

        match response {
            ApiResponse::ApiResult(MethodResponse {
                ok: true,
                result: updates,
                description: None,
            }) => {
                assert_eq!(1, updates.len());

                let update = &updates[0];

                assert_eq!(379656753, update.update_id())
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn send_message_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2746,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618207352,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"text\":\"Hello!\"}}";
        let params =
            SendMessageParams::new(ChatIdEnum::IsizeVariant(275808073), "Hello!".to_string());
        let api = api_with_mock("/sendMessage", response_string);

        let response = api.send_message(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_message_failure() {
        let response_string =
            "{\"ok\":false,\"description\":\"Bad Request: chat not found\",\"error_code\":400}";
        let params = SendMessageParams::new(ChatIdEnum::IsizeVariant(1), "Hello!".to_string());
        let api = api_with_mock("/sendMessage", response_string);

        let response = api.send_message(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn set_webhook_success() {
        let response_string =
            "{\"ok\":true,\"result\":true,\"description\":\"Webhook is already deleted\"}";
        let params = SetWebhookParams::new("".to_string());
        let api = api_with_mock("/setWebhook", response_string);

        let response = api.set_webhook(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn delete_webhook_success() {
        let response_string =
            "{\"ok\":true,\"result\":true,\"description\":\"Webhook is already deleted\"}";
        let params = DeleteWebhookParams::new();
        let api = api_with_mock("/deleteWebhook", response_string);

        let response = api.delete_webhook(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    fn api_with_mock(path: &str, response: &str) -> API {
        let server = MockServer::start();

        server.mock(|when, then| {
            when.method(POST).path(path);
            then.status(200).body(response);
        });

        API::new_url(server.url(""))
    }
}
