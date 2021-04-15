use crate::api_params::CopyMessageParams;
use crate::api_params::DeleteWebhookParams;
use crate::api_params::EditMessageLiveLocationParams;
use crate::api_params::ForwardMessageParams;
use crate::api_params::GetFileParams;
use crate::api_params::GetUpdatesParams;
use crate::api_params::GetUserProfilePhotosParams;
use crate::api_params::KickChatMemberParams;
use crate::api_params::SendChatActionParams;
use crate::api_params::SendContactParams;
use crate::api_params::SendDiceParams;
use crate::api_params::SendLocationParams;
use crate::api_params::SendMessageParams;
use crate::api_params::SendPollParams;
use crate::api_params::SendVenueParams;
use crate::api_params::SetWebhookParams;
use crate::api_params::StopMessageLiveLocationParams;
use crate::api_params::UnbanChatMemberParams;
use crate::objects::File;
use crate::objects::Message;
use crate::objects::MessageId;
use crate::objects::Update;
use crate::objects::User;
use crate::objects::UserProfilePhotos;
use crate::objects::WebhookInfo;
use isahc::{prelude::*, Request};
use serde::{Deserialize, Serialize};

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

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageLiveLocationResponse {
    Message(ApiResponse<Message>),
    Bool(ApiResponse<bool>),
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

    pub fn get_webhook_info(&self) -> Result<ApiResponse<WebhookInfo>, isahc::Error> {
        self.request_without_body("getWebhookInfo")
    }

    pub fn get_me(&self) -> Result<ApiResponse<User>, isahc::Error> {
        self.request_without_body("getMe")
    }

    pub fn log_out(&self) -> Result<ApiResponse<bool>, isahc::Error> {
        self.request_without_body("logOut")
    }

    pub fn close(&self) -> Result<ApiResponse<bool>, isahc::Error> {
        self.request_without_body("close")
    }

    pub fn forward_message(
        &self,
        params: ForwardMessageParams,
    ) -> Result<ApiResponse<Message>, isahc::Error> {
        self.request("forwardMessage", Some(params))
    }

    pub fn copy_message(
        &self,
        params: CopyMessageParams,
    ) -> Result<ApiResponse<MessageId>, isahc::Error> {
        self.request("copyMessage", Some(params))
    }

    pub fn send_location(
        &self,
        params: SendLocationParams,
    ) -> Result<ApiResponse<Message>, isahc::Error> {
        self.request("sendLocation", Some(params))
    }

    pub fn edit_message_live_location(
        &self,
        params: EditMessageLiveLocationParams,
    ) -> Result<MessageLiveLocationResponse, isahc::Error> {
        self.request("editMessageLiveLocation", Some(params))
    }

    pub fn stop_message_live_location(
        &self,
        params: StopMessageLiveLocationParams,
    ) -> Result<MessageLiveLocationResponse, isahc::Error> {
        self.request("stopMessageLiveLocation", Some(params))
    }

    pub fn send_venue(
        &self,
        params: SendVenueParams,
    ) -> Result<ApiResponse<Message>, isahc::Error> {
        self.request("sendVenue", Some(params))
    }

    pub fn send_contact(
        &self,
        params: SendContactParams,
    ) -> Result<ApiResponse<Message>, isahc::Error> {
        self.request("sendContact", Some(params))
    }

    pub fn send_poll(&self, params: SendPollParams) -> Result<ApiResponse<Message>, isahc::Error> {
        self.request("sendPoll", Some(params))
    }

    pub fn send_dice(&self, params: SendDiceParams) -> Result<ApiResponse<Message>, isahc::Error> {
        self.request("sendDice", Some(params))
    }

    pub fn send_chat_action(
        &self,
        params: SendChatActionParams,
    ) -> Result<ApiResponse<bool>, isahc::Error> {
        self.request("sendChatAction", Some(params))
    }

    pub fn get_user_profile_photos(
        &self,
        params: GetUserProfilePhotosParams,
    ) -> Result<ApiResponse<UserProfilePhotos>, isahc::Error> {
        self.request("getUserProfilePhotos", Some(params))
    }

    pub fn get_file(&self, params: GetFileParams) -> Result<ApiResponse<File>, isahc::Error> {
        self.request("getFile", Some(params))
    }

    pub fn kick_chat_member(
        &self,
        params: KickChatMemberParams,
    ) -> Result<ApiResponse<bool>, isahc::Error> {
        self.request("kickChatMember", Some(params))
    }

    pub fn unban_chat_member(
        &self,
        params: UnbanChatMemberParams,
    ) -> Result<ApiResponse<bool>, isahc::Error> {
        self.request("unbanChatMember", Some(params))
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

        let parsed_response: T2 = serde_json::from_reader(response.body_mut()).unwrap();

        Ok(parsed_response)
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

    #[test]
    fn get_webhook_info_success() {
        let response_string = "{\"ok\":true,\"result\":{\"url\":\"\",\"has_custom_certificate\":false,\"pending_update_count\":0,\"allowed_updates\":[\"message\"]}}";
        let api = api_with_mock("/getWebhookInfo", response_string);

        let response = api.get_webhook_info().unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn get_me_success() {
        let response_string = "{\"ok\":true,\"result\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\",\"can_join_groups\":true,\"can_read_all_group_messages\":false,\"supports_inline_queries\":false}}";
        let api = api_with_mock("/getMe", response_string);

        let response = api.get_me().unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn log_out_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let api = api_with_mock("/logOut", response_string);

        let response = api.log_out().unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn close_failure() {
        let response_string = "{\"ok\":false,\"description\":\"Unauthorized\",\"error_code\":401}";

        let api = api_with_mock("/close", response_string);

        let response = api.close().unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn close_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let api = api_with_mock("/close", response_string);

        let response = api.close().unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn forward_message_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2747,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618294971,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"text\":\"Hello\"}}";
        let params = ForwardMessageParams::new(
            ChatIdEnum::IsizeVariant(275808073),
            ChatIdEnum::IsizeVariant(275808073),
            2747,
        );

        let api = api_with_mock("/forwardMessage", response_string);

        let response = api.forward_message(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn copy_message_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2749}}";
        let params = CopyMessageParams::new(
            ChatIdEnum::IsizeVariant(275808073),
            ChatIdEnum::IsizeVariant(275808073),
            2747,
        );

        let api = api_with_mock("/copyMessage", response_string);

        let response = api.copy_message(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_location_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2750,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618382060,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"location\":{\"longitude\":6.949981,\"latitude\":49.700002}}}";
        let params = SendLocationParams::new(ChatIdEnum::IsizeVariant(275808073), 49.7, 6.95);
        let api = api_with_mock("/sendLocation", response_string);

        let response = api.send_location(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn edit_message_live_location_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2752,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618382998,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"edit_date\":1618383189,\"location\":{\"longitude\":6.96,\"latitude\":49.800009,\"live_period\":300}}}";
        let mut params = EditMessageLiveLocationParams::new(49.8, 6.96);
        params.set_message_id(Some(2752));
        params.set_chat_id(Some(ChatIdEnum::IsizeVariant(275808073)));

        let api = api_with_mock("/editMessageLiveLocation", response_string);

        let response = api.edit_message_live_location(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn stop_message_live_location_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2752,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618382998,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"edit_date\":1618383189,\"location\":{\"longitude\":6.96,\"latitude\":49.800009,\"live_period\":300}}}";
        let mut params = StopMessageLiveLocationParams::new();
        params.set_message_id(Some(2752));
        params.set_chat_id(Some(ChatIdEnum::IsizeVariant(275808073)));

        let api = api_with_mock("/stopMessageLiveLocation", response_string);

        let response = api.stop_message_live_location(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_venue_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2754,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618410490,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"venue\":{\"location\":{\"longitude\":6.949981,\"latitude\":49.700002},\"title\":\"Meow\",\"address\":\"Hoof\"},\"location\":{\"longitude\":6.949981,\"latitude\":49.700002}}}";
        let params = SendVenueParams::new(
            ChatIdEnum::IsizeVariant(275808073),
            49.7,
            6.95,
            "Meow".to_string(),
            "Hoof".to_string(),
        );

        let api = api_with_mock("/sendVenue", response_string);

        let response = api.send_venue(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_contact_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2755,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618465934,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"contact\":{\"phone_number\":\"49\",\"first_name\":\"Meow\"}}}";
        let params = SendContactParams::new(
            ChatIdEnum::IsizeVariant(275808073),
            "49".to_string(),
            "Meow".to_string(),
        );

        let api = api_with_mock("/sendContact", response_string);

        let response = api.send_contact(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_poll_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2756,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618466302,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"poll\":{\"id\":\"5458519832506925078\",\"question\":\"are you?\",\"options\":[{\"text\":\"1\",\"voter_count\":0},{\"text\":\"2\",\"voter_count\":0}],\"total_voter_count\":0,\"is_closed\":false,\"is_anonymous\":true,\"type\":\"regular\",\"allows_multiple_answers\":false}}}";
        let params = SendPollParams::new(
            ChatIdEnum::IsizeVariant(275808073),
            "are you?".to_string(),
            vec!["1".to_string(), "2".to_string()],
        );

        let api = api_with_mock("/sendPoll", response_string);

        let response = api.send_poll(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_dice_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2757,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618467133,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"dice\":{\"emoji\":\"🎲\",\"value\":5}}}";
        let params = SendDiceParams::new(ChatIdEnum::IsizeVariant(275808073));

        let api = api_with_mock("/sendDice", response_string);

        let response = api.send_dice(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_chat_action_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params =
            SendChatActionParams::new(ChatIdEnum::IsizeVariant(275808073), "typing".to_string());

        let api = api_with_mock("/sendChatAction", response_string);

        let response = api.send_chat_action(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn get_user_profile_photos_success() {
        let response_string = "{\"ok\":true,\"result\":{\"total_count\":3,\"photos\":[[{\"file_id\":\"AgACAgIAAxUAAWB332IlzabFGWzaMrOdQ4ODVLyaAAKypzEbSX9wEEzMxT7F-grc3UA5DwAEAQADAgADYQADg0kCAAEfBA\",\"file_unique_id\":\"AQAD3UA5DwAEg0kCAAE\",\"width\":160,\"height\":160,\"file_size\":8068},{\"file_id\":\"AgACAgIAAxUAAWB332IlzabFGWzaMrOdQ4ODVLyaAAKypzEbSX9wEEzMxT7F-grc3UA5DwAEAQADAgADYgADhEkCAAEfBA\",\"file_unique_id\":\"AQAD3UA5DwAEhEkCAAE\",\"width\":320,\"height\":320,\"file_size\":22765},{\"file_id\":\"AgACAgIAAxUAAWB332IlzabFGWzaMrOdQ4ODVLyaAAKypzEbSX9wEEzMxT7F-grc3UA5DwAEAQADAgADYwADhUkCAAEfBA\",\"file_unique_id\":\"AQAD3UA5DwAEhUkCAAE\",\"width\":640,\"height\":640,\"file_size\":65663}],[{\"file_id\":\"AgACAgIAAxUAAWB332JpnZNv9ZNeZeIt1FFCdOroAAKwpzEbSX9wEOb5okUMX3tVSRdLDQAEAQADAgADYQADZj0KAAEfBA\",\"file_unique_id\":\"AQADSRdLDQAEZj0KAAE\",\"width\":160,\"height\":160,\"file_size\":13459},{\"file_id\":\"AgACAgIAAxUAAWB332JpnZNv9ZNeZeIt1FFCdOroAAKwpzEbSX9wEOb5okUMX3tVSRdLDQAEAQADAgADYgADZz0KAAEfBA\",\"file_unique_id\":\"AQADSRdLDQAEZz0KAAE\",\"width\":320,\"height\":320,\"file_size\":41243},{\"file_id\":\"AgACAgIAAxUAAWB332JpnZNv9ZNeZeIt1FFCdOroAAKwpzEbSX9wEOb5okUMX3tVSRdLDQAEAQADAgADYwADaD0KAAEfBA\",\"file_unique_id\":\"AQADSRdLDQAEaD0KAAE\",\"width\":640,\"height\":640,\"file_size\":114427}],[{\"file_id\":\"AgACAgIAAxUAAWB332ISVowq4pXLx3y1o-7WQteeAAKvpzEbSX9wEBlOkdDjqlYW1Du3DQAEAQADAgADYQADdkwAAh8E\",\"file_unique_id\":\"AQAD1Du3DQAEdkwAAg\",\"width\":160,\"height\":160,\"file_size\":6631},{\"file_id\":\"AgACAgIAAxUAAWB332ISVowq4pXLx3y1o-7WQteeAAKvpzEbSX9wEBlOkdDjqlYW1Du3DQAEAQADAgADYgADd0wAAh8E\",\"file_unique_id\":\"AQAD1Du3DQAEd0wAAg\",\"width\":320,\"height\":320,\"file_size\":20495},{\"file_id\":\"AgACAgIAAxUAAWB332ISVowq4pXLx3y1o-7WQteeAAKvpzEbSX9wEBlOkdDjqlYW1Du3DQAEAQADAgADYwADeEwAAh8E\",\"file_unique_id\":\"AQAD1Du3DQAEeEwAAg\",\"width\":640,\"height\":640,\"file_size\":54395}]]}}";
        let params = GetUserProfilePhotosParams::new(275808073);

        let api = api_with_mock("/getUserProfilePhotos", response_string);

        let response = api.get_user_profile_photos(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn get_file_success() {
        let response_string = "{\"ok\":true,\"result\":{\"file_id\":\"AgACAgIAAxUAAWB332IlzabFGWzaMrOdQ4ODVLyaAAKypzEbSX9wEEzMxT7F-grc3UA5DwAEAQADAgADYQADg0kCAAEfBA\",\"file_unique_id\":\"AQAD3UA5DwAEg0kCAAE\",\"file_size\":8068,\"file_path\":\"photos/file_0.jpg\"}}";
        let  params = GetFileParams::new(
        "AgACAgIAAxUAAWB332IlzabFGWzaMrOdQ4ODVLyaAAKypzEbSX9wEEzMxT7F-grc3UA5DwAEAQADAgADYQADg0kCAAEfBA".to_string()
    );

        let api = api_with_mock("/getFile", response_string);

        let response = api.get_file(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn kick_chat_member_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = KickChatMemberParams::new(ChatIdEnum::IsizeVariant(-1001368460856), 275808073);

        let api = api_with_mock("/kickChatMember", response_string);

        let response = api.kick_chat_member(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn unban_chat_member_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params =
            UnbanChatMemberParams::new(ChatIdEnum::IsizeVariant(-1001368460856), 275808072);

        let api = api_with_mock("/unbanChatMember", response_string);

        let response = api.unban_chat_member(params).unwrap();

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
