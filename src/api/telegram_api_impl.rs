use super::Error;
use super::HttpError;
use crate::api_traits::ErrorResponse;
use crate::api_traits::TelegramApi;
use multipart::client::lazy::Multipart;
use serde_json::Value;
use std::path::PathBuf;
use std::time::Duration;
use typed_builder::TypedBuilder;
use ureq::Response;

#[derive(Debug, Clone, TypedBuilder)]
pub struct Api {
    #[builder(setter(into))]
    pub api_url: String,
    #[builder(default_code = "ureq::builder().timeout(Duration::from_secs(500)).build()")]
    pub request_agent: ureq::Agent,
}

impl Api {
    /// Create a new `Api`. You can use `Api::builder()` for more options.
    pub fn new(api_key: &str) -> Self {
        let api_url = format!("{}{api_key}", super::BASE_API_URL);

        Self::builder().api_url(api_url).build()
    }

    /// Create a new `Api`. You can use `Api::builder()` for more options.
    pub fn new_url<T: Into<String>>(api_url: T) -> Self {
        Api::builder().api_url(api_url).build()
    }

    pub fn encode_params<T: serde::ser::Serialize + std::fmt::Debug>(
        params: &T,
    ) -> Result<String, Error> {
        serde_json::to_string(params).map_err(|e| Error::Encode(format!("{e:?} : {params:?}")))
    }

    pub fn decode_response<T: serde::de::DeserializeOwned>(response: Response) -> Result<T, Error> {
        match response.into_string() {
            Ok(message) => {
                let json_result: Result<T, serde_json::Error> = serde_json::from_str(&message);

                match json_result {
                    Ok(result) => Ok(result),
                    Err(e) => {
                        let err = Error::Decode(format!("{e:?} : {message:?}"));
                        Err(err)
                    }
                }
            }
            Err(e) => {
                let err = Error::Decode(format!("Failed to decode response: {e:?}"));
                Err(err)
            }
        }
    }
}

impl From<ureq::Error> for Error {
    fn from(error: ureq::Error) -> Self {
        match error {
            ureq::Error::Status(code, response) => match response.into_string() {
                Ok(message) => {
                    let json_result: Result<ErrorResponse, serde_json::Error> =
                        serde_json::from_str(&message);

                    match json_result {
                        Ok(result) => Self::Api(result),
                        Err(_) => {
                            let error = HttpError { code, message };
                            Self::Http(error)
                        }
                    }
                }
                Err(_) => {
                    let message = "Failed to decode response".to_string();
                    let error = HttpError { code, message };
                    Self::Http(error)
                }
            },
            ureq::Error::Transport(transport_error) => {
                let message = format!("{transport_error:?}");
                let error = HttpError { message, code: 500 };
                Self::Http(error)
            }
        }
    }
}

impl TelegramApi for Api {
    type Error = Error;

    fn request<T1: serde::ser::Serialize + std::fmt::Debug, T2: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: Option<T1>,
    ) -> Result<T2, Error> {
        let url = format!("{}/{method}", self.api_url);
        let prepared_request = self
            .request_agent
            .post(&url)
            .set("Content-Type", "application/json");

        let response = match params {
            None => prepared_request.call()?,
            Some(data) => {
                let json = Self::encode_params(&data)?;

                prepared_request.send_string(&json)?
            }
        };

        let parsed_response: T2 = Self::decode_response(response)?;

        Ok(parsed_response)
    }

    fn request_with_form_data<
        T1: serde::ser::Serialize + std::fmt::Debug,
        T2: serde::de::DeserializeOwned,
    >(
        &self,
        method: &str,
        params: T1,
        files: Vec<(&str, PathBuf)>,
    ) -> Result<T2, Error> {
        let json_string = Self::encode_params(&params)?;
        let json_struct: Value = serde_json::from_str(&json_string).unwrap();
        let file_keys: Vec<&str> = files.iter().map(|(key, _)| *key).collect();
        let files_with_names: Vec<(&str, Option<&str>, PathBuf)> = files
            .iter()
            .map(|(key, path)| (*key, path.file_name().unwrap().to_str(), path.clone()))
            .collect();

        let mut form = Multipart::new();
        for (key, val) in json_struct.as_object().unwrap() {
            if !file_keys.contains(&key.as_str()) {
                let val = match val {
                    Value::String(val) => val.to_string(),
                    other => other.to_string(),
                };

                form.add_text(key, val);
            }
        }

        for (parameter_name, file_name, file_path) in files_with_names {
            let file = std::fs::File::open(&file_path).unwrap();
            let file_extension = file_path
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("");
            let mime = mime_guess::from_ext(file_extension).first_or_octet_stream();

            form.add_stream(parameter_name, file, file_name, Some(mime));
        }

        let url = format!("{}/{method}", self.api_url);
        let form_data = form.prepare().unwrap();
        let response = self
            .request_agent
            .post(&url)
            .set(
                "Content-Type",
                &format!("multipart/form-data; boundary={}", form_data.boundary()),
            )
            .send(form_data)?;

        let parsed_response: T2 = Self::decode_response(response)?;

        Ok(parsed_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_params::AnswerCallbackQueryParams;
    use crate::api_params::AnswerInlineQueryParams;
    use crate::api_params::BanChatMemberParams;
    use crate::api_params::BotCommandScope;
    use crate::api_params::BotCommandScopeChat;
    use crate::api_params::ChatAction;
    use crate::api_params::ChatId;
    use crate::api_params::CopyMessageParams;
    use crate::api_params::CreateChatInviteLinkParams;
    use crate::api_params::DeleteChatPhotoParams;
    use crate::api_params::DeleteChatStickerSetParams;
    use crate::api_params::DeleteMessageParams;
    use crate::api_params::DeleteMyCommandsParams;
    use crate::api_params::DeleteWebhookParams;
    use crate::api_params::EditChatInviteLinkParams;
    use crate::api_params::EditMessageCaptionParams;
    use crate::api_params::EditMessageLiveLocationParams;
    use crate::api_params::EditMessageMediaParams;
    use crate::api_params::EditMessageTextParams;
    use crate::api_params::ExportChatInviteLinkParams;
    use crate::api_params::File;
    use crate::api_params::ForwardMessageParams;
    use crate::api_params::GetChatAdministratorsParams;
    use crate::api_params::GetChatMemberCountParams;
    use crate::api_params::GetChatMemberParams;
    use crate::api_params::GetChatParams;
    use crate::api_params::GetFileParams;
    use crate::api_params::GetMyCommandsParams;
    use crate::api_params::GetStickerSetParams;
    use crate::api_params::GetUpdatesParams;
    use crate::api_params::GetUserProfilePhotosParams;
    use crate::api_params::InlineQueryResult;
    use crate::api_params::InputFile;
    use crate::api_params::InputMedia;
    use crate::api_params::InputMediaPhoto;
    use crate::api_params::LeaveChatParams;
    use crate::api_params::Media;
    use crate::api_params::PinChatMessageParams;
    use crate::api_params::PromoteChatMemberParams;
    use crate::api_params::RestrictChatMemberParams;
    use crate::api_params::RevokeChatInviteLinkParams;
    use crate::api_params::SendAnimationParams;
    use crate::api_params::SendAudioParams;
    use crate::api_params::SendChatActionParams;
    use crate::api_params::SendContactParams;
    use crate::api_params::SendDiceParams;
    use crate::api_params::SendDocumentParams;
    use crate::api_params::SendLocationParams;
    use crate::api_params::SendMediaGroupParams;
    use crate::api_params::SendMessageParams;
    use crate::api_params::SendPhotoParams;
    use crate::api_params::SendPollParams;
    use crate::api_params::SendStickerParams;
    use crate::api_params::SendVenueParams;
    use crate::api_params::SendVideoNoteParams;
    use crate::api_params::SendVideoParams;
    use crate::api_params::SendVoiceParams;
    use crate::api_params::SetChatAdministratorCustomTitleParams;
    use crate::api_params::SetChatDescriptionParams;
    use crate::api_params::SetChatPermissionsParams;
    use crate::api_params::SetChatPhotoParams;
    use crate::api_params::SetChatStickerSetParams;
    use crate::api_params::SetChatTitleParams;
    use crate::api_params::SetMyCommandsParams;
    use crate::api_params::SetWebhookParams;
    use crate::api_params::StopMessageLiveLocationParams;
    use crate::api_params::StopPollParams;
    use crate::api_params::UnbanChatMemberParams;
    use crate::api_params::UnpinChatMessageParams;
    use crate::objects::BotCommand;
    use crate::objects::ChatPermissions;
    use crate::objects::InlineQueryResultVenue;
    use crate::AllowedUpdate;

    #[test]
    fn new_sets_correct_url() {
        let api = Api::new("hey");

        assert_eq!("https://api.telegram.org/bothey", api.api_url);
    }

    #[test]
    fn get_updates_success() {
        let response_string = "{\"ok\":true,\"result\":[{\"update_id\":379656753,\"message\":{\"message_id\":2741,\"from\":{\"id\":275808073,\"is_bot\":false,\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\",\"username\":\"Ayrat555\",\"language_code\":\"en\"},\"date\":1618149703,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"text\":\"dsaf\"}}]}";
        let params = GetUpdatesParams::builder()
            .allowed_updates(vec![AllowedUpdate::Message])
            .build();

        let _m = mockito::mock("POST", "/getUpdates")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.get_updates(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();

        assert_eq!(response_string, json);
        assert_eq!(1, response.result.len());

        let update = &response.result[0];

        assert_eq!(379656753, update.update_id);
    }

    #[test]
    fn send_message_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2746,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618207352,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"text\":\"Hello!\"}}";
        let params = SendMessageParams::builder()
            .chat_id(275808073)
            .text("Hello!")
            .build();
        let _m = mockito::mock("POST", "/sendMessage")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.send_message(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_message_failure() {
        let response_string =
            "{\"ok\":false,\"description\":\"Bad Request: chat not found\",\"error_code\":400}";
        let params = SendMessageParams::builder()
            .chat_id(1)
            .text("Hello!")
            .build();
        let _m = mockito::mock("POST", "/sendMessage")
            .with_status(400)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        if let Err(Error::Api(ErrorResponse {
            ok: false,
            description,
            error_code: 400,
            parameters: None,
        })) = api.send_message(&params)
        {
            assert_eq!("Bad Request: chat not found".to_string(), description);
        } else {
            panic!("Error was expected but there is none");
        }
    }

    #[test]
    fn set_webhook_success() {
        let response_string =
            "{\"ok\":true,\"result\":true,\"description\":\"Webhook is already deleted\"}";
        let params = SetWebhookParams::builder().url("").build();
        let _m = mockito::mock("POST", "/setWebhook")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.set_webhook(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn delete_webhook_success() {
        let response_string =
            "{\"ok\":true,\"result\":true,\"description\":\"Webhook is already deleted\"}";
        let params = DeleteWebhookParams::builder().build();
        let _m = mockito::mock("POST", "/deleteWebhook")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.delete_webhook(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn get_webhook_info_success() {
        let response_string = "{\"ok\":true,\"result\":{\"url\":\"\",\"has_custom_certificate\":false,\"pending_update_count\":0,\"allowed_updates\":[\"message\"]}}";
        let _m = mockito::mock("POST", "/getWebhookInfo")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.get_webhook_info().unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn get_me_success() {
        let response_string = "{\"ok\":true,\"result\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\",\"can_join_groups\":true,\"can_read_all_group_messages\":false,\"supports_inline_queries\":false}}";
        let _m = mockito::mock("POST", "/getMe")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.get_me().unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn log_out_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let _m = mockito::mock("POST", "/logOut")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.log_out().unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn close_failure() {
        let response_string = "{\"ok\":false,\"description\":\"Unauthorized\",\"error_code\":401}";

        let _m = mockito::mock("POST", "/close")
            .with_status(400)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.close().err();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn close_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let _m = mockito::mock("POST", "/close")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.close().unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn forward_message_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2747,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618294971,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"text\":\"Hello\"}}";
        let params = ForwardMessageParams::builder()
            .chat_id(275808073)
            .from_chat_id(275808073)
            .message_id(2747)
            .build();

        let _m = mockito::mock("POST", "/forwardMessage")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.forward_message(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn copy_message_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2749}}";
        let params = CopyMessageParams::builder()
            .chat_id(275808073)
            .from_chat_id(275808073)
            .message_id(2747)
            .build();

        let _m = mockito::mock("POST", "/copyMessage")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.copy_message(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_location_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2750,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618382060,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"location\":{\"longitude\":6.949981,\"latitude\":49.700002}}}";
        let params = SendLocationParams::builder()
            .chat_id(275808073)
            .latitude(49.7)
            .longitude(6.95)
            .build();

        let _m = mockito::mock("POST", "/sendLocation")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.send_location(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn edit_message_live_location_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2752,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618382998,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"edit_date\":1618383189,\"location\":{\"longitude\":6.96,\"latitude\":49.800009,\"live_period\":300}}}";
        let params = EditMessageLiveLocationParams::builder()
            .chat_id(275808073)
            .message_id(2752)
            .latitude(49.7)
            .longitude(6.95)
            .build();

        let _m = mockito::mock("POST", "/editMessageLiveLocation")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.edit_message_live_location(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn stop_message_live_location_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2752,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618382998,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"edit_date\":1618383189,\"location\":{\"longitude\":6.96,\"latitude\":49.800009,\"live_period\":300}}}";
        let params = StopMessageLiveLocationParams::builder()
            .chat_id(275808073)
            .message_id(2752)
            .build();

        let _m = mockito::mock("POST", "/stopMessageLiveLocation")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.stop_message_live_location(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_venue_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2754,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618410490,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"venue\":{\"location\":{\"longitude\":6.949981,\"latitude\":49.700002},\"title\":\"Meow\",\"address\":\"Hoof\"},\"location\":{\"longitude\":6.949981,\"latitude\":49.700002}}}";
        let params = SendVenueParams::builder()
            .chat_id(275808073)
            .latitude(49.7)
            .longitude(6.95)
            .title("Meow")
            .address("Hoof")
            .build();

        let _m = mockito::mock("POST", "/sendVenue")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.send_venue(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_contact_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2755,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618465934,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"contact\":{\"phone_number\":\"49\",\"first_name\":\"Meow\"}}}";
        let params = SendContactParams::builder()
            .chat_id(275808073)
            .phone_number("49")
            .first_name("Meow")
            .build();

        let _m = mockito::mock("POST", "/sendContact")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.send_contact(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_poll_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2756,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618466302,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"poll\":{\"id\":\"5458519832506925078\",\"question\":\"are you?\",\"options\":[{\"text\":\"1\",\"voter_count\":0},{\"text\":\"2\",\"voter_count\":0}],\"total_voter_count\":0,\"is_closed\":false,\"is_anonymous\":true,\"type\":\"regular\",\"allows_multiple_answers\":false}}}";
        let params = SendPollParams::builder()
            .chat_id(275808073)
            .question("are you?")
            .options(vec!["1".to_string(), "2".to_string()])
            .build();

        let _m = mockito::mock("POST", "/sendPoll")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.send_poll(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_dice_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2757,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618467133,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"dice\":{\"emoji\":\"🎲\",\"value\":5}}}";
        let params = SendDiceParams::builder().chat_id(275808073).build();

        let _m = mockito::mock("POST", "/sendDice")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.send_dice(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_chat_action_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = SendChatActionParams::builder()
            .chat_id(275808073)
            .action(ChatAction::Typing)
            .build();

        let _m = mockito::mock("POST", "/sendChatAction")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.send_chat_action(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn get_user_profile_photos_success() {
        let response_string = "{\"ok\":true,\"result\":{\"total_count\":3,\"photos\":[[{\"file_id\":\"AgACAgIAAxUAAWB332IlzabFGWzaMrOdQ4ODVLyaAAKypzEbSX9wEEzMxT7F-grc3UA5DwAEAQADAgADYQADg0kCAAEfBA\",\"file_unique_id\":\"AQAD3UA5DwAEg0kCAAE\",\"width\":160,\"height\":160,\"file_size\":8068},{\"file_id\":\"AgACAgIAAxUAAWB332IlzabFGWzaMrOdQ4ODVLyaAAKypzEbSX9wEEzMxT7F-grc3UA5DwAEAQADAgADYgADhEkCAAEfBA\",\"file_unique_id\":\"AQAD3UA5DwAEhEkCAAE\",\"width\":320,\"height\":320,\"file_size\":22765},{\"file_id\":\"AgACAgIAAxUAAWB332IlzabFGWzaMrOdQ4ODVLyaAAKypzEbSX9wEEzMxT7F-grc3UA5DwAEAQADAgADYwADhUkCAAEfBA\",\"file_unique_id\":\"AQAD3UA5DwAEhUkCAAE\",\"width\":640,\"height\":640,\"file_size\":65663}],[{\"file_id\":\"AgACAgIAAxUAAWB332JpnZNv9ZNeZeIt1FFCdOroAAKwpzEbSX9wEOb5okUMX3tVSRdLDQAEAQADAgADYQADZj0KAAEfBA\",\"file_unique_id\":\"AQADSRdLDQAEZj0KAAE\",\"width\":160,\"height\":160,\"file_size\":13459},{\"file_id\":\"AgACAgIAAxUAAWB332JpnZNv9ZNeZeIt1FFCdOroAAKwpzEbSX9wEOb5okUMX3tVSRdLDQAEAQADAgADYgADZz0KAAEfBA\",\"file_unique_id\":\"AQADSRdLDQAEZz0KAAE\",\"width\":320,\"height\":320,\"file_size\":41243},{\"file_id\":\"AgACAgIAAxUAAWB332JpnZNv9ZNeZeIt1FFCdOroAAKwpzEbSX9wEOb5okUMX3tVSRdLDQAEAQADAgADYwADaD0KAAEfBA\",\"file_unique_id\":\"AQADSRdLDQAEaD0KAAE\",\"width\":640,\"height\":640,\"file_size\":114427}],[{\"file_id\":\"AgACAgIAAxUAAWB332ISVowq4pXLx3y1o-7WQteeAAKvpzEbSX9wEBlOkdDjqlYW1Du3DQAEAQADAgADYQADdkwAAh8E\",\"file_unique_id\":\"AQAD1Du3DQAEdkwAAg\",\"width\":160,\"height\":160,\"file_size\":6631},{\"file_id\":\"AgACAgIAAxUAAWB332ISVowq4pXLx3y1o-7WQteeAAKvpzEbSX9wEBlOkdDjqlYW1Du3DQAEAQADAgADYgADd0wAAh8E\",\"file_unique_id\":\"AQAD1Du3DQAEd0wAAg\",\"width\":320,\"height\":320,\"file_size\":20495},{\"file_id\":\"AgACAgIAAxUAAWB332ISVowq4pXLx3y1o-7WQteeAAKvpzEbSX9wEBlOkdDjqlYW1Du3DQAEAQADAgADYwADeEwAAh8E\",\"file_unique_id\":\"AQAD1Du3DQAEeEwAAg\",\"width\":640,\"height\":640,\"file_size\":54395}]]}}";
        let params = GetUserProfilePhotosParams::builder()
            .user_id(275808073_u64)
            .build();

        let _m = mockito::mock("POST", "/getUserProfilePhotos")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.get_user_profile_photos(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn get_file_success() {
        let response_string = "{\"ok\":true,\"result\":{\"file_id\":\"AgACAgIAAxUAAWB332IlzabFGWzaMrOdQ4ODVLyaAAKypzEbSX9wEEzMxT7F-grc3UA5DwAEAQADAgADYQADg0kCAAEfBA\",\"file_unique_id\":\"AQAD3UA5DwAEg0kCAAE\",\"file_size\":8068,\"file_path\":\"photos/file_0.jpg\"}}";
        let  params = GetFileParams::builder()
            .file_id("AgACAgIAAxUAAWB332IlzabFGWzaMrOdQ4ODVLyaAAKypzEbSX9wEEzMxT7F-grc3UA5DwAEAQADAgADYQADg0kCAAEfBA")
            .build();

        let _m = mockito::mock("POST", "/getFile")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.get_file(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn ban_chat_member_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = BanChatMemberParams::builder()
            .chat_id(-1001368460856)
            .user_id(275808073_u64)
            .build();

        let _m = mockito::mock("POST", "/banChatMember")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.ban_chat_member(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn unban_chat_member_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = UnbanChatMemberParams::builder()
            .chat_id(-1001368460856)
            .user_id(275808072_u64)
            .build();

        let _m = mockito::mock("POST", "/unbanChatMember")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.unban_chat_member(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn restrict_chat_member_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let perm = ChatPermissions::builder()
            .can_add_web_page_previews(true)
            .build();

        let params = RestrictChatMemberParams::builder()
            .chat_id(-1001368460856)
            .user_id(275808073_u64)
            .permissions(perm)
            .build();

        let _m = mockito::mock("POST", "/restrictChatMember")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.restrict_chat_member(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn promote_chat_member_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = PromoteChatMemberParams::builder()
            .chat_id(-1001368460856)
            .user_id(275808073_u64)
            .can_change_info(true)
            .build();

        let _m = mockito::mock("POST", "/promoteChatMember")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.promote_chat_member(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn set_chat_administrator_custom_title_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = SetChatAdministratorCustomTitleParams::builder()
            .chat_id(-1001368460856)
            .user_id(275808073_u64)
            .custom_title("King")
            .build();

        let _m = mockito::mock("POST", "/setChatAdministratorCustomTitle")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.set_chat_administrator_custom_title(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn set_chat_permissions_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let perm = ChatPermissions::builder()
            .can_add_web_page_previews(true)
            .build();

        let params = SetChatPermissionsParams::builder()
            .chat_id(-1001368460856)
            .permissions(perm)
            .build();

        let _m = mockito::mock("POST", "/setChatPermissions")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.set_chat_permissions(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn export_chat_invite_link_success() {
        let response_string = "{\"ok\":true,\"result\":\"https://t.me/joinchat/txIUDwjfk7M2ODEy\"}";

        let params = ExportChatInviteLinkParams::builder()
            .chat_id(-1001368460856)
            .build();

        let _m = mockito::mock("POST", "/exportChatInviteLink")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.export_chat_invite_link(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn create_chat_invite_link_success() {
        let response_string = "{\"ok\":true,\"result\":{\"invite_link\":\"https://t.me/joinchat/yFEnSaeiQm83ZTNi\",\"creator\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"creates_join_request\":false,\"is_primary\":false,\"is_revoked\":false}}";

        let params = CreateChatInviteLinkParams::builder()
            .chat_id(-1001368460856)
            .build();

        let _m = mockito::mock("POST", "/createChatInviteLink")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.create_chat_invite_link(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn edit_chat_invite_link_success() {
        let response_string = "{\"ok\":true,\"result\":{\"invite_link\":\"https://t.me/joinchat/O458bA8hQ0MzNmQy\",\"creator\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"creates_join_request\":false,\"is_primary\":false,\"is_revoked\":false}}";

        let params = EditChatInviteLinkParams::builder()
            .chat_id(-1001368460856)
            .invite_link("https://t.me/joinchat/O458bA8hQ0MzNmQy")
            .build();

        let _m = mockito::mock("POST", "/editChatInviteLink")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.edit_chat_invite_link(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn revoke_chat_invite_link_success() {
        let response_string = "{\"ok\":true,\"result\":{\"invite_link\":\"https://t.me/joinchat/O458bA8hQ0MzNmQy\",\"creator\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"creates_join_request\":false,\"is_primary\":false,\"is_revoked\":true}}";

        let params = RevokeChatInviteLinkParams::builder()
            .chat_id(-1001368460856)
            .invite_link("https://t.me/joinchat/O458bA8hQ0MzNmQy")
            .build();

        let _m = mockito::mock("POST", "/revokeChatInviteLink")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.revoke_chat_invite_link(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn delete_chat_photo_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = DeleteChatPhotoParams::builder()
            .chat_id(-1001368460856)
            .build();

        let _m = mockito::mock("POST", "/deleteChatPhoto")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.delete_chat_photo(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_photo_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2763,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618730180,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"photo\":[{\"file_id\":\"AgACAgIAAxkDAAIKy2B73MQXIhoDDmLXjcUjgqGf-m8bAALjsDEbORLgS-s4BkBzcC5DYvIBny4AAwEAAwIAA20AA0U3AwABHwQ\",\"file_unique_id\":\"AQADYvIBny4AA0U3AwAB\",\"width\":320,\"height\":320,\"file_size\":19968},{\"file_id\":\"AgACAgIAAxkDAAIKy2B73MQXIhoDDmLXjcUjgqGf-m8bAALjsDEbORLgS-s4BkBzcC5DYvIBny4AAwEAAwIAA3gAA0Y3AwABHwQ\",\"file_unique_id\":\"AQADYvIBny4AA0Y3AwAB\",\"width\":799,\"height\":800,\"file_size\":63581},{\"file_id\":\"AgACAgIAAxkDAAIKy2B73MQXIhoDDmLXjcUjgqGf-m8bAALjsDEbORLgS-s4BkBzcC5DYvIBny4AAwEAAwIAA3kAA0M3AwABHwQ\",\"file_unique_id\":\"AQADYvIBny4AA0M3AwAB\",\"width\":847,\"height\":848,\"file_size\":63763}]}}";
        let file = std::path::PathBuf::from("./frankenstein_logo.png");
        let params = SendPhotoParams::builder()
            .chat_id(275808073)
            .photo(file)
            .build();

        let _m = mockito::mock("POST", "/sendPhoto")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.send_photo(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_audio_file_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2766,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618735176,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"audio\":{\"file_id\":\"CQACAgIAAxkDAAIKzmB78EjK-iOHo-HKC-M6p4r0jGdmAALkDAACORLgS5co1z0uFAKgHwQ\",\"file_unique_id\":\"AgAD5AwAAjkS4Es\",\"duration\":123,\"title\":\"Way Back Home\",\"file_name\":\"audio.mp3\",\"mime_type\":\"audio/mpeg\",\"file_size\":2957092}}}";
        let file = std::path::PathBuf::from("./frankenstein_logo.png");
        let params = SendAudioParams::builder()
            .chat_id(275808073)
            .audio(file)
            .build();
        let _m = mockito::mock("POST", "/sendAudio")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.send_audio(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_audio_file_with_thumb_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2766,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618735176,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"audio\":{\"file_id\":\"CQACAgIAAxkDAAIKzmB78EjK-iOHo-HKC-M6p4r0jGdmAALkDAACORLgS5co1z0uFAKgHwQ\",\"file_unique_id\":\"AgAD5AwAAjkS4Es\",\"duration\":123,\"title\":\"Way Back Home\",\"file_name\":\"audio.mp3\",\"mime_type\":\"audio/mpeg\",\"file_size\":2957092}}}";

        let file = std::path::PathBuf::from("./frankenstein_logo.png");
        let params = SendAudioParams::builder()
            .chat_id(275808073)
            .audio(file.clone())
            .thumb(file)
            .build();
        let _m = mockito::mock("POST", "/sendAudio")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.send_audio(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_audio_file_id_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2769,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618735333,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"audio\":{\"file_id\":\"CQACAgIAAxkDAAIK0WB78OUFavWx6fjzCQ_d5qnu_R7mAALkDAACORLgS5co1z0uFAKgHwQ\",\"file_unique_id\":\"AgAD5AwAAjkS4Es\",\"duration\":123,\"title\":\"Way Back Home\",\"file_name\":\"audio.mp3\",\"mime_type\":\"audio/mpeg\",\"file_size\":2957092}}}";
        let file = File::String(
            "CQACAgIAAxkDAAIKzmB78EjK-iOHo-HKC-M6p4r0jGdmAALkDAACORLgS5co1z0uFAKgHwQ".to_string(),
        );
        let params = SendAudioParams::builder()
            .chat_id(275808073)
            .audio(file)
            .build();

        let _m = mockito::mock("POST", "/sendAudio")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.send_audio(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_document_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2770,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618737593,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"audio\":{\"file_id\":\"CQACAgIAAxkDAAIK0mB7-bnnewABfdaFKK4NzVLQ7BvgCwAC6gwAAjkS4Et_njaNR8IUMB8E\",\"file_unique_id\":\"AgAD6gwAAjkS4Es\",\"duration\":123,\"title\":\"Way Back Home\",\"file_name\":\"audio.mp3\",\"mime_type\":\"audio/mpeg\",\"file_size\":2957092}}}";

        let file = std::path::PathBuf::from("./frankenstein_logo.png");
        let params = SendDocumentParams::builder()
            .chat_id(275808073)
            .document(file)
            .build();

        let _m = mockito::mock("POST", "/sendDocument")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.send_document(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_video_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2770,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618737593,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"audio\":{\"file_id\":\"CQACAgIAAxkDAAIK0mB7-bnnewABfdaFKK4NzVLQ7BvgCwAC6gwAAjkS4Et_njaNR8IUMB8E\",\"file_unique_id\":\"AgAD6gwAAjkS4Es\",\"duration\":123,\"title\":\"Way Back Home\",\"file_name\":\"audio.mp3\",\"mime_type\":\"audio/mpeg\",\"file_size\":2957092}}}";

        let file = std::path::PathBuf::from("./frankenstein_logo.png");
        let params = SendVideoParams::builder()
            .chat_id(275808073)
            .video(file)
            .build();

        let _m = mockito::mock("POST", "/sendVideo")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.send_video(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_animation_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2770,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618737593,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"audio\":{\"file_id\":\"CQACAgIAAxkDAAIK0mB7-bnnewABfdaFKK4NzVLQ7BvgCwAC6gwAAjkS4Et_njaNR8IUMB8E\",\"file_unique_id\":\"AgAD6gwAAjkS4Es\",\"duration\":123,\"title\":\"Way Back Home\",\"file_name\":\"audio.mp3\",\"mime_type\":\"audio/mpeg\",\"file_size\":2957092}}}";

        let file = std::path::PathBuf::from("./frankenstein_logo.png");
        let params = SendAnimationParams::builder()
            .chat_id(275808073)
            .animation(file)
            .build();

        let _m = mockito::mock("POST", "/sendAnimation")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.send_animation(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_voice_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2770,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618737593,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"audio\":{\"file_id\":\"CQACAgIAAxkDAAIK0mB7-bnnewABfdaFKK4NzVLQ7BvgCwAC6gwAAjkS4Et_njaNR8IUMB8E\",\"file_unique_id\":\"AgAD6gwAAjkS4Es\",\"duration\":123,\"title\":\"Way Back Home\",\"file_name\":\"audio.mp3\",\"mime_type\":\"audio/mpeg\",\"file_size\":2957092}}}";
        let file = std::path::PathBuf::from("./frankenstein_logo.png");
        let params = SendVoiceParams::builder()
            .chat_id(275808073)
            .voice(file)
            .build();
        let _m = mockito::mock("POST", "/sendVoice")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.send_voice(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_video_note_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2770,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618737593,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"audio\":{\"file_id\":\"CQACAgIAAxkDAAIK0mB7-bnnewABfdaFKK4NzVLQ7BvgCwAC6gwAAjkS4Et_njaNR8IUMB8E\",\"file_unique_id\":\"AgAD6gwAAjkS4Es\",\"duration\":123,\"title\":\"Way Back Home\",\"file_name\":\"audio.mp3\",\"mime_type\":\"audio/mpeg\",\"file_size\":2957092}}}";
        let file = std::path::PathBuf::from("./frankenstein_logo.png");
        let params = SendVideoNoteParams::builder()
            .chat_id(275808073)
            .video_note(file)
            .build();
        let _m = mockito::mock("POST", "/sendVideoNote")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.send_video_note(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn set_chat_photo_success() {
        let response_string = "{\"ok\":true,\"result\":true}";

        let file = InputFile {
            path: std::path::PathBuf::from("./frankenstein_logo.png"),
        };
        let params = SetChatPhotoParams::builder()
            .chat_id(275808073)
            .photo(file)
            .build();
        let _m = mockito::mock("POST", "/setChatPhoto")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.set_chat_photo(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn set_chat_title_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = SetChatTitleParams::builder()
            .chat_id(-1001368460856)
            .title("Frankenstein")
            .build();

        let _m = mockito::mock("POST", "/setChatTitle")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.set_chat_title(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn set_chat_description_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = SetChatDescriptionParams::builder()
            .chat_id(-1001368460856)
            .description("Frankenstein group")
            .build();

        let _m = mockito::mock("POST", "/setChatDescription")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.set_chat_description(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn pin_chat_message_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = PinChatMessageParams::builder()
            .chat_id(275808073)
            .message_id(2766)
            .build();

        let _m = mockito::mock("POST", "/pinChatMessage")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.pin_chat_message(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn unpin_chat_message_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = UnpinChatMessageParams::builder().chat_id(275808073).build();

        let _m = mockito::mock("POST", "/unpinChatMessage")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.unpin_chat_message(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn leave_chat_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = LeaveChatParams::builder().chat_id(-1001368460856).build();

        let _m = mockito::mock("POST", "/leaveChat")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.leave_chat(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn get_chat_success() {
        let response_string = "{\"ok\":true,\"result\":{\"id\":-1001368460856,\"type\":\"supergroup\",\"title\":\"Frankenstein\",\"photo\":{\"small_file_id\":\"AQADAgAT-kgrmy4AAwIAA8jhydkW____s1Cm6Dc_w8Ge7QUAAR8E\",\"small_file_unique_id\":\"AQAD-kgrmy4AA57tBQAB\",\"big_file_id\":\"AQADAgAT-kgrmy4AAwMAA8jhydkW____s1Cm6Dc_w8Gg7QUAAR8E\",\"big_file_unique_id\":\"AQAD-kgrmy4AA6DtBQAB\"},\"description\":\"Frankenstein group\",\"invite_link\":\"https://t.me/joinchat/smSXMzNKTwA0ZjFi\",\"permissions\":{\"can_send_messages\":true,\"can_send_polls\":true,\"can_send_other_messages\":true,\"can_add_web_page_previews\":true,\"can_change_info\":true,\"can_invite_users\":true,\"can_pin_messages\":true}}}";
        let params = GetChatParams::builder().chat_id(-1001368460856).build();

        let _m = mockito::mock("POST", "/getChat")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.get_chat(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn get_chat_administrators_success() {
        let response_string = "{\"ok\":true,\"result\":[{\"status\":\"administrator\",\"user\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"can_be_edited\":false,\"is_anonymous\":true,\"can_manage_chat\":true,\"can_delete_messages\":true,\"can_manage_video_chats\":true,\"can_restrict_members\":true,\"can_promote_members\":false,\"can_change_info\":true,\"can_invite_users\":true,\"can_pin_messages\":true},{\"status\":\"creator\",\"user\":{\"id\":275808073,\"is_bot\":false,\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\",\"username\":\"Ayrat555\",\"language_code\":\"en\"},\"is_anonymous\":false}]}";
        let params = GetChatAdministratorsParams::builder()
            .chat_id(-1001368460856)
            .build();

        let _m = mockito::mock("POST", "/getChatAdministrators")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.get_chat_administrators(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn get_chat_members_count_success() {
        let response_string = "{\"ok\":true,\"result\":4}";
        let params = GetChatMemberCountParams::builder()
            .chat_id(-1001368460856)
            .build();

        let _m = mockito::mock("POST", "/getChatMemberCount")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.get_chat_member_count(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn get_chat_member_success() {
        let response_string = "{\"ok\":true,\"result\":{\"status\":\"creator\",\"user\":{\"id\":275808073,\"is_bot\":false,\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\",\"username\":\"Ayrat555\",\"language_code\":\"en\"},\"is_anonymous\":false}}";
        let params = GetChatMemberParams::builder()
            .chat_id(-1001368460856)
            .user_id(275808073_u64)
            .build();

        let _m = mockito::mock("POST", "/getChatMember")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.get_chat_member(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn set_chat_sticker_set_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = SetChatStickerSetParams::builder()
            .chat_id(-1001368460856)
            .sticker_set_name("GBTDPack")
            .build();

        let _m = mockito::mock("POST", "/setChatStickerSet")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.set_chat_sticker_set(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn delete_chat_sticker_set_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = DeleteChatStickerSetParams::builder()
            .chat_id(-1001368460856)
            .build();

        let _m = mockito::mock("POST", "/deleteChatStickerSet")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.delete_chat_sticker_set(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn answer_callback_query_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = AnswerCallbackQueryParams::builder()
            .callback_query_id("id")
            .text("text")
            .build();

        let _m = mockito::mock("POST", "/answerCallbackQuery")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.answer_callback_query(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn set_my_commands_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = SetMyCommandsParams::builder()
            .commands(vec![
                BotCommand::builder()
                    .command("meow")
                    .description("mewo")
                    .build(),
                BotCommand::builder()
                    .command("meow1")
                    .description("mewo1")
                    .build(),
            ])
            .build();

        let _m = mockito::mock("POST", "/setMyCommands")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.set_my_commands(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn set_my_commands_default_scope_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = SetMyCommandsParams::builder()
            .commands(vec![
                BotCommand::builder()
                    .command("meow")
                    .description("mewo")
                    .build(),
                BotCommand::builder()
                    .command("meow1")
                    .description("mewo1")
                    .build(),
            ])
            .scope(BotCommandScope::Default)
            .build();

        let _m = mockito::mock("POST", "/setMyCommands")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.set_my_commands(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn delete_my_commands_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let scope = BotCommandScope::Chat(BotCommandScopeChat {
            chat_id: ChatId::Integer(275808073),
        });
        let params = DeleteMyCommandsParams::builder()
            .scope(scope)
            .language_code("es")
            .build();

        let _m = mockito::mock("POST", "/deleteMyCommands")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.delete_my_commands(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn get_my_commands_success() {
        let response_string = "{\"ok\":true,\"result\":[{\"command\":\"meow\",\"description\":\"mewo\"},{\"command\":\"meow1\",\"description\":\"mewo1\"}]}";

        let params = GetMyCommandsParams::builder().build();
        let _m = mockito::mock("POST", "/getMyCommands")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.get_my_commands(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn get_my_commands_scope_success() {
        let response_string = "{\"ok\":true,\"result\":[]}";

        let scope = BotCommandScope::Chat(BotCommandScopeChat {
            chat_id: ChatId::Integer(275808073),
        });
        let params = GetMyCommandsParams::builder().scope(scope).build();

        let _m = mockito::mock("POST", "/getMyCommands")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.get_my_commands(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn answer_inline_query_success() {
        let response_string = "{\"ok\":true,\"result\":true}";

        let venue_result = InlineQueryResult::Venue(
            InlineQueryResultVenue::builder()
                .id("hey9sdfasdflasdfadsfasd")
                .latitude(40.0)
                .longitude(40.0)
                .title("title")
                .address("address")
                .build(),
        );

        let params = AnswerInlineQueryParams::builder()
            .inline_query_id("inline_query.id()")
            .results(vec![venue_result])
            .build();

        let _m = mockito::mock("POST", "/answerInlineQuery")
            .with_status(200)
            .with_body(response_string)
            .create();

        let api = Api::new_url(mockito::server_url());

        let response = api.answer_inline_query(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn edit_message_text_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2782,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1619158127,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"edit_date\":1619158446,\"text\":\"Hi!!\"}}";

        let params = EditMessageTextParams::builder()
            .text("Hi!!")
            .chat_id(275808073)
            .message_id(2782)
            .build();

        let _m = mockito::mock("POST", "/editMessageText")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.edit_message_text(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn edit_message_caption_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2784,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1619159414,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"edit_date\":1619159461,\"photo\":[{\"file_id\":\"AgACAgIAAxkDAAIK4GCCaaWDayYgzQ-BykVy8LYkW0wzAAL-rzEbRx8RSCnCkWjXtdN9ZNLmny4AAwEAAwIAA20AA1hCAAIfBA\",\"file_unique_id\":\"AQADZNLmny4AA1hCAAI\",\"width\":320,\"height\":320,\"file_size\":19162},{\"file_id\":\"AgACAgIAAxkDAAIK4GCCaaWDayYgzQ-BykVy8LYkW0wzAAL-rzEbRx8RSCnCkWjXtdN9ZNLmny4AAwEAAwIAA3gAA1lCAAIfBA\",\"file_unique_id\":\"AQADZNLmny4AA1lCAAI\",\"width\":800,\"height\":800,\"file_size\":65697},{\"file_id\":\"AgACAgIAAxkDAAIK4GCCaaWDayYgzQ-BykVy8LYkW0wzAAL-rzEbRx8RSCnCkWjXtdN9ZNLmny4AAwEAAwIAA3kAA1pCAAIfBA\",\"file_unique_id\":\"AQADZNLmny4AA1pCAAI\",\"width\":1146,\"height\":1146,\"file_size\":101324}],\"caption\":\"Caption\"}}";

        let params = EditMessageCaptionParams::builder()
            .chat_id(275808073)
            .message_id(2784)
            .caption("Caption")
            .build();

        let _m = mockito::mock("POST", "/editMessageCaption")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.edit_message_caption(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn stop_poll_success() {
        let response_string = "{\"ok\":true,\"result\":{\"id\":\"5195109123171024927\",\"question\":\"are you?\",\"options\":[{\"text\":\"1\",\"voter_count\":1},{\"text\":\"2\",\"voter_count\":0}],\"total_voter_count\":1,\"is_closed\":true,\"is_anonymous\":true,\"type\":\"regular\",\"allows_multiple_answers\":false}}";

        let params = StopPollParams::builder()
            .chat_id(-1001368460856)
            .message_id(495)
            .build();

        let _m = mockito::mock("POST", "/stopPoll")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.stop_poll(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn delete_message_success() {
        let response_string = "{\"ok\":true,\"result\":true}";

        let params = DeleteMessageParams::builder()
            .chat_id(275808073)
            .message_id(2784)
            .build();

        let _m = mockito::mock("POST", "/deleteMessage")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.delete_message(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_sticker_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2788,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1619245784,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"sticker\":{\"file_id\":\"CAACAgIAAxkDAAIK5GCDutgNxc07rqqtjkGWrGskbHfQAAIMEAACRx8ZSKJ6Z5GkdVHcHwQ\",\"file_unique_id\":\"AgADDBAAAkcfGUg\",\"type\":\"regular\",\"width\":512,\"height\":512,\"is_animated\":false,\"is_video\":false,\"thumb\":{\"file_id\":\"AAMCAgADGQMAAgrkYIO62A3FzTuuqq2OQZasayRsd9AAAgwQAAJHHxlIonpnkaR1Udz29bujLgADAQAHbQADzR4AAh8E\",\"file_unique_id\":\"AQAD9vW7oy4AA80eAAI\",\"width\":320,\"height\":320,\"file_size\":19264},\"file_size\":36596}}}";

        let file = std::path::PathBuf::from("./frankenstein_logo.png");
        let params = SendStickerParams::builder()
            .chat_id(275808073)
            .sticker(file)
            .build();

        let _m = mockito::mock("POST", "/sendSticker")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.send_sticker(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn get_sticker_set_success() {
        let response_string = "{\"ok\":true,\"result\":{\"name\":\"unocards\",\"title\":\"UNO Bot\",\"sticker_type\":\"regular\",\"is_animated\":false,\"is_video\":false,\"contains_masks\":false,\"stickers\":[{\"file_id\":\"CAACAgQAAxUAAWCDxAQVJ6X7FGiBD5NyjN5DDvgfAALZAQACX1eZAAEqnpNt3SpG_x8E\",\"file_unique_id\":\"AgAD2QEAAl9XmQAB\",\"type\":\"regular\",\"width\":342,\"height\":512,\"is_animated\":false,\"is_video\":false,\"thumb\":{\"file_id\":\"AAMCBAADFQABYIPEBBUnpfsUaIEPk3KM3kMO-B8AAtkBAAJfV5kAASqek23dKkb_P75BGQAEAQAHbQADBBEAAh8E\",\"file_unique_id\":\"AQADP75BGQAEBBEAAg\",\"width\":85,\"height\":128,\"file_size\":2452},\"emoji\":\"dd\",\"set_name\":\"unocards\",\"file_size\":8898}]}}";

        let params = GetStickerSetParams::builder().name("unocards").build();
        let _m = mockito::mock("POST", "/getStickerSet")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());
        let response = api.get_sticker_set(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_media_group_success() {
        let response_string = "{\"ok\":true,\"result\":[{\"message_id\":510,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1619267462,\"chat\":{\"id\":-1001368460856,\"type\":\"supergroup\",\"title\":\"Frankenstein\"},\"media_group_id\":\"12954139699368426\",\"photo\":[{\"file_id\":\"AgACAgIAAx0EUZEOOAACAf5ghA-GtOaBIP2NOmtXdze-Un7PGgAC_q8xG0cfEUgpwpFo17XTfWTS5p8uAAMBAAMCAANtAANYQgACHwQ\",\"file_unique_id\":\"AQADZNLmny4AA1hCAAI\",\"width\":320,\"height\":320,\"file_size\":19162},{\"file_id\":\"AgACAgIAAx0EUZEOOAACAf5ghA-GtOaBIP2NOmtXdze-Un7PGgAC_q8xG0cfEUgpwpFo17XTfWTS5p8uAAMBAAMCAAN4AANZQgACHwQ\",\"file_unique_id\":\"AQADZNLmny4AA1lCAAI\",\"width\":800,\"height\":800,\"file_size\":65697},{\"file_id\":\"AgACAgIAAx0EUZEOOAACAf5ghA-GtOaBIP2NOmtXdze-Un7PGgAC_q8xG0cfEUgpwpFo17XTfWTS5p8uAAMBAAMCAAN5AANaQgACHwQ\",\"file_unique_id\":\"AQADZNLmny4AA1pCAAI\",\"width\":1146,\"height\":1146,\"file_size\":101324}]},{\"message_id\":511,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1619267462,\"chat\":{\"id\":-1001368460856,\"type\":\"supergroup\",\"title\":\"Frankenstein\"},\"media_group_id\":\"12954139699368426\",\"photo\":[{\"file_id\":\"AgACAgIAAx0EUZEOOAACAf9ghA-GeFo0B7v78UyXoOD9drjEGgAC_q8xG0cfEUgpwpFo17XTfWTS5p8uAAMBAAMCAANtAANYQgACHwQ\",\"file_unique_id\":\"AQADZNLmny4AA1hCAAI\",\"width\":320,\"height\":320,\"file_size\":19162},{\"file_id\":\"AgACAgIAAx0EUZEOOAACAf9ghA-GeFo0B7v78UyXoOD9drjEGgAC_q8xG0cfEUgpwpFo17XTfWTS5p8uAAMBAAMCAAN4AANZQgACHwQ\",\"file_unique_id\":\"AQADZNLmny4AA1lCAAI\",\"width\":800,\"height\":800,\"file_size\":65697},{\"file_id\":\"AgACAgIAAx0EUZEOOAACAf9ghA-GeFo0B7v78UyXoOD9drjEGgAC_q8xG0cfEUgpwpFo17XTfWTS5p8uAAMBAAMCAAN5AANaQgACHwQ\",\"file_unique_id\":\"AQADZNLmny4AA1pCAAI\",\"width\":1146,\"height\":1146,\"file_size\":101324}]}]}";

        let file = std::path::PathBuf::from("./frankenstein_logo.png");

        let photo = InputMediaPhoto::builder().media(file).build();
        let medias = vec![Media::Photo(photo.clone()), Media::Photo(photo)];

        let params = SendMediaGroupParams::builder()
            .chat_id(-1001368460856)
            .media(medias)
            .build();

        let _m = mockito::mock("POST", "/sendMediaGroup")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.send_media_group(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn edit_message_media_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":513,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1619336672,\"chat\":{\"id\":-1001368460856,\"type\":\"supergroup\",\"title\":\"Frankenstein\"},\"edit_date\":1619336788,\"photo\":[{\"file_id\":\"AgACAgIAAx0EUZEOOAACAgFghR5URaBN41jx7VNgLPi29xmfQgAC_q8xG0cfEUgpwpFo17XTfWTS5p8uAAMBAAMCAANtAANYQgACHwQ\",\"file_unique_id\":\"AQADZNLmny4AA1hCAAI\",\"width\":320,\"height\":320,\"file_size\":19162},{\"file_id\":\"AgACAgIAAx0EUZEOOAACAgFghR5URaBN41jx7VNgLPi29xmfQgAC_q8xG0cfEUgpwpFo17XTfWTS5p8uAAMBAAMCAAN4AANZQgACHwQ\",\"file_unique_id\":\"AQADZNLmny4AA1lCAAI\",\"width\":800,\"height\":800,\"file_size\":65697},{\"file_id\":\"AgACAgIAAx0EUZEOOAACAgFghR5URaBN41jx7VNgLPi29xmfQgAC_q8xG0cfEUgpwpFo17XTfWTS5p8uAAMBAAMCAAN5AANaQgACHwQ\",\"file_unique_id\":\"AQADZNLmny4AA1pCAAI\",\"width\":1146,\"height\":1146,\"file_size\":101324}]}}";

        let file = std::path::PathBuf::from("./frankenstein_logo.png");

        let params = EditMessageMediaParams::builder()
            .media(InputMedia::Photo(
                InputMediaPhoto::builder().media(file).build(),
            ))
            .chat_id(-1001368460856)
            .message_id(513)
            .build();

        let _m = mockito::mock("POST", "/editMessageMedia")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.edit_message_media(&params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn returns_decode_error_if_response_can_not_be_decoded() {
        let response_string = "{hey this json is invalid}";
        let params = GetUpdatesParams::builder()
            .allowed_updates(vec![AllowedUpdate::Message])
            .build();

        let _m = mockito::mock("POST", "/getUpdates")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = Api::new_url(mockito::server_url());

        let response = api.get_updates(&params);

        assert_eq!(
            Err(Error::Decode("Error(\"key must be a string\", line: 1, column: 2) : \"{hey this json is invalid}\"".to_string())),
            response
        );
    }
}
