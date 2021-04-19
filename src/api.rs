use crate::api_params::AnimationEnum;
use crate::api_params::AudioEnum;
use crate::api_params::CopyMessageParams;
use crate::api_params::CreateChatInviteLinkParams;
use crate::api_params::DeleteChatPhotoParams;
use crate::api_params::DeleteWebhookParams;
use crate::api_params::DocumentEnum;
use crate::api_params::EditChatInviteLinkParams;
use crate::api_params::EditMessageLiveLocationParams;
use crate::api_params::ExportChatInviteLinkParams;
use crate::api_params::ForwardMessageParams;
use crate::api_params::GetFileParams;
use crate::api_params::GetUpdatesParams;
use crate::api_params::GetUserProfilePhotosParams;
use crate::api_params::KickChatMemberParams;
use crate::api_params::PhotoEnum;
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
use crate::api_params::SendMessageParams;
use crate::api_params::SendPhotoParams;
use crate::api_params::SendPollParams;
use crate::api_params::SendVenueParams;
use crate::api_params::SendVideoNoteParams;
use crate::api_params::SendVideoParams;
use crate::api_params::SendVoiceParams;
use crate::api_params::SetChatAdministratorCustomTitleParams;
use crate::api_params::SetChatDescriptionParams;
use crate::api_params::SetChatPermissionsParams;
use crate::api_params::SetChatPhotoParams;
use crate::api_params::SetChatTitleParams;
use crate::api_params::SetWebhookParams;
use crate::api_params::StopMessageLiveLocationParams;
use crate::api_params::UnbanChatMemberParams;
use crate::api_params::VideoEnum;
use crate::api_params::VideoNoteEnum;
use crate::api_params::VoiceEnum;
use crate::objects::ChatInviteLink;
use crate::objects::File;
use crate::objects::Message;
use crate::objects::MessageId;
use crate::objects::ThumbEnum;
use crate::objects::Update;
use crate::objects::User;
use crate::objects::UserProfilePhotos;
use crate::objects::WebhookInfo;
use multipart::client::lazy::Multipart;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

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
    ) -> Result<ApiResponse<Vec<Update>>, ureq::Error> {
        self.request("getUpdates", Some(params))
    }

    pub fn send_message(
        &self,
        params: SendMessageParams,
    ) -> Result<ApiResponse<Message>, ureq::Error> {
        self.request("sendMessage", Some(params))
    }

    pub fn set_webhook(&self, params: SetWebhookParams) -> Result<ApiResponse<bool>, ureq::Error> {
        self.request("setWebhook", Some(params))
    }

    pub fn delete_webhook(
        &self,
        params: DeleteWebhookParams,
    ) -> Result<ApiResponse<bool>, ureq::Error> {
        self.request("deleteWebhook", Some(params))
    }

    pub fn get_webhook_info(&self) -> Result<ApiResponse<WebhookInfo>, ureq::Error> {
        self.request_without_body("getWebhookInfo")
    }

    pub fn get_me(&self) -> Result<ApiResponse<User>, ureq::Error> {
        self.request_without_body("getMe")
    }

    pub fn log_out(&self) -> Result<ApiResponse<bool>, ureq::Error> {
        self.request_without_body("logOut")
    }

    pub fn close(&self) -> Result<ApiResponse<bool>, ureq::Error> {
        self.request_without_body("close")
    }

    pub fn forward_message(
        &self,
        params: ForwardMessageParams,
    ) -> Result<ApiResponse<Message>, ureq::Error> {
        self.request("forwardMessage", Some(params))
    }

    pub fn copy_message(
        &self,
        params: CopyMessageParams,
    ) -> Result<ApiResponse<MessageId>, ureq::Error> {
        self.request("copyMessage", Some(params))
    }

    pub fn send_photo(&self, params: SendPhotoParams) -> Result<ApiResponse<Message>, ureq::Error> {
        let method_name = "sendPhoto";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let PhotoEnum::InputFileVariant(input_file) = params.photo() {
            files.push(("photo", input_file.path()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    pub fn send_audio(&self, params: SendAudioParams) -> Result<ApiResponse<Message>, ureq::Error> {
        let method_name = "sendAudio";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let AudioEnum::InputFileVariant(input_file) = params.audio() {
            files.push(("audio", input_file.path()));
        }

        if let Some(ThumbEnum::InputFileVariant(input_file)) = params.thumb() {
            files.push(("thumb", input_file.path()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    pub fn send_document(
        &self,
        params: SendDocumentParams,
    ) -> Result<ApiResponse<Message>, ureq::Error> {
        let method_name = "sendDocument";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let DocumentEnum::InputFileVariant(input_file) = params.document() {
            files.push(("document", input_file.path()));
        }

        if let Some(ThumbEnum::InputFileVariant(input_file)) = params.thumb() {
            files.push(("thumb", input_file.path()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    pub fn send_video(&self, params: SendVideoParams) -> Result<ApiResponse<Message>, ureq::Error> {
        let method_name = "sendVideo";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let VideoEnum::InputFileVariant(input_file) = params.video() {
            files.push(("video", input_file.path()));
        }

        if let Some(ThumbEnum::InputFileVariant(input_file)) = params.thumb() {
            files.push(("thumb", input_file.path()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    pub fn send_animation(
        &self,
        params: SendAnimationParams,
    ) -> Result<ApiResponse<Message>, ureq::Error> {
        let method_name = "sendAnimation";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let AnimationEnum::InputFileVariant(input_file) = params.animation() {
            files.push(("animation", input_file.path()));
        }

        if let Some(ThumbEnum::InputFileVariant(input_file)) = params.thumb() {
            files.push(("thumb", input_file.path()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    pub fn send_voice(&self, params: SendVoiceParams) -> Result<ApiResponse<Message>, ureq::Error> {
        let method_name = "sendVoice";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let VoiceEnum::InputFileVariant(input_file) = params.voice() {
            files.push(("voice", input_file.path()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    pub fn send_video_note(
        &self,
        params: SendVideoNoteParams,
    ) -> Result<ApiResponse<Message>, ureq::Error> {
        let method_name = "sendVideoNote";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let VideoNoteEnum::InputFileVariant(input_file) = params.video_note() {
            files.push(("video_note", input_file.path()));
        }

        if let Some(ThumbEnum::InputFileVariant(input_file)) = params.thumb() {
            files.push(("thumb", input_file.path()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    pub fn send_location(
        &self,
        params: SendLocationParams,
    ) -> Result<ApiResponse<Message>, ureq::Error> {
        self.request("sendLocation", Some(params))
    }

    pub fn edit_message_live_location(
        &self,
        params: EditMessageLiveLocationParams,
    ) -> Result<MessageLiveLocationResponse, ureq::Error> {
        self.request("editMessageLiveLocation", Some(params))
    }

    pub fn stop_message_live_location(
        &self,
        params: StopMessageLiveLocationParams,
    ) -> Result<MessageLiveLocationResponse, ureq::Error> {
        self.request("stopMessageLiveLocation", Some(params))
    }

    pub fn send_venue(&self, params: SendVenueParams) -> Result<ApiResponse<Message>, ureq::Error> {
        self.request("sendVenue", Some(params))
    }

    pub fn send_contact(
        &self,
        params: SendContactParams,
    ) -> Result<ApiResponse<Message>, ureq::Error> {
        self.request("sendContact", Some(params))
    }

    pub fn send_poll(&self, params: SendPollParams) -> Result<ApiResponse<Message>, ureq::Error> {
        self.request("sendPoll", Some(params))
    }

    pub fn send_dice(&self, params: SendDiceParams) -> Result<ApiResponse<Message>, ureq::Error> {
        self.request("sendDice", Some(params))
    }

    pub fn send_chat_action(
        &self,
        params: SendChatActionParams,
    ) -> Result<ApiResponse<bool>, ureq::Error> {
        self.request("sendChatAction", Some(params))
    }

    pub fn get_user_profile_photos(
        &self,
        params: GetUserProfilePhotosParams,
    ) -> Result<ApiResponse<UserProfilePhotos>, ureq::Error> {
        self.request("getUserProfilePhotos", Some(params))
    }

    pub fn get_file(&self, params: GetFileParams) -> Result<ApiResponse<File>, ureq::Error> {
        self.request("getFile", Some(params))
    }

    pub fn kick_chat_member(
        &self,
        params: KickChatMemberParams,
    ) -> Result<ApiResponse<bool>, ureq::Error> {
        self.request("kickChatMember", Some(params))
    }

    pub fn unban_chat_member(
        &self,
        params: UnbanChatMemberParams,
    ) -> Result<ApiResponse<bool>, ureq::Error> {
        self.request("unbanChatMember", Some(params))
    }

    pub fn restrict_chat_member(
        &self,
        params: RestrictChatMemberParams,
    ) -> Result<ApiResponse<bool>, ureq::Error> {
        self.request("restrictChatMember", Some(params))
    }

    pub fn promote_chat_member(
        &self,
        params: PromoteChatMemberParams,
    ) -> Result<ApiResponse<bool>, ureq::Error> {
        self.request("promoteChatMember", Some(params))
    }

    pub fn set_chat_administrator_custom_title(
        &self,
        params: SetChatAdministratorCustomTitleParams,
    ) -> Result<ApiResponse<bool>, ureq::Error> {
        self.request("setChatAdministratorCustomTitle", Some(params))
    }

    pub fn set_chat_permissions(
        &self,
        params: SetChatPermissionsParams,
    ) -> Result<ApiResponse<bool>, ureq::Error> {
        self.request("setChatPermissions", Some(params))
    }

    pub fn export_chat_invite_link(
        &self,
        params: ExportChatInviteLinkParams,
    ) -> Result<ApiResponse<String>, ureq::Error> {
        self.request("exportChatInviteLink", Some(params))
    }

    pub fn create_chat_invite_link(
        &self,
        params: CreateChatInviteLinkParams,
    ) -> Result<ApiResponse<ChatInviteLink>, ureq::Error> {
        self.request("createChatInviteLink", Some(params))
    }

    pub fn edit_chat_invite_link(
        &self,
        params: EditChatInviteLinkParams,
    ) -> Result<ApiResponse<ChatInviteLink>, ureq::Error> {
        self.request("editChatInviteLink", Some(params))
    }

    pub fn revoke_chat_invite_link(
        &self,
        params: RevokeChatInviteLinkParams,
    ) -> Result<ApiResponse<ChatInviteLink>, ureq::Error> {
        self.request("revokeChatInviteLink", Some(params))
    }

    pub fn set_chat_photo(
        &self,
        params: SetChatPhotoParams,
    ) -> Result<ApiResponse<bool>, ureq::Error> {
        let photo = params.photo();

        self.request_with_form_data("setChatPhoto", params, vec![("photo", photo.path())])
    }

    pub fn delete_chat_photo(
        &self,
        params: DeleteChatPhotoParams,
    ) -> Result<ApiResponse<bool>, ureq::Error> {
        self.request("deleteChatPhoto", Some(params))
    }

    pub fn set_chat_title(
        &self,
        params: SetChatTitleParams,
    ) -> Result<ApiResponse<bool>, ureq::Error> {
        self.request("setChatTitle", Some(params))
    }

    pub fn set_chat_description(
        &self,
        params: SetChatDescriptionParams,
    ) -> Result<ApiResponse<bool>, ureq::Error> {
        self.request("setChatDescription", Some(params))
    }

    pub fn pin_chat_message(
        &self,
        params: PinChatMessageParams,
    ) -> Result<ApiResponse<bool>, ureq::Error> {
        self.request("pinChatMessage", Some(params))
    }

    fn request_without_body<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
    ) -> Result<T, ureq::Error> {
        let params: Option<()> = None;

        self.request(method, params)
    }

    fn request<T1: serde::ser::Serialize, T2: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: Option<T1>,
    ) -> Result<T2, ureq::Error> {
        let url = format!("{}/{}", self.api_url, method);

        let prepared_request = ureq::post(&url).set("Content-Type", "application/json");

        let response = match params {
            None => prepared_request.call()?,
            Some(data) => {
                let json = serde_json::to_string(&data).unwrap();

                prepared_request.send_string(&json)?
            }
        };

        let parsed_response: T2 = serde_json::from_reader(response.into_reader()).unwrap();

        Ok(parsed_response)
    }

    fn request_with_possible_form_data<
        T1: serde::ser::Serialize,
        T2: serde::de::DeserializeOwned,
    >(
        &self,
        method_name: &str,
        params: T1,
        files: Vec<(&str, PathBuf)>,
    ) -> Result<T2, ureq::Error> {
        if files.len() > 0 {
            self.request_with_form_data(method_name, params, files)
        } else {
            self.request(method_name, Some(params))
        }
    }

    fn request_with_form_data<T1: serde::ser::Serialize, T2: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: T1,
        files: Vec<(&str, PathBuf)>,
    ) -> Result<T2, ureq::Error> {
        let json_string = serde_json::to_string(&params).unwrap();
        let json_struct: Value = serde_json::from_str(&json_string).unwrap();
        let file_keys: Vec<&str> = files.iter().map(|(key, _)| *key).collect();
        let files_with_names: Vec<(&str, Option<&str>, PathBuf)> = files
            .iter()
            .map(|(key, path)| (*key, path.file_name().unwrap().to_str(), path.clone()))
            .collect();

        let mut form = Multipart::new();
        for (key, val) in json_struct.as_object().unwrap().iter() {
            if !file_keys.contains(&key.as_str()) {
                let val = match val {
                    &Value::String(ref val) => format!("{}", val),
                    etc => format!("{}", etc),
                };

                form.add_text(key, val);
            }
        }

        for (parameter_name, file_name, file_path) in files_with_names {
            let file = std::fs::File::open(&file_path).unwrap();
            let file_extension = file_path.extension().and_then(|s| s.to_str()).unwrap_or("");
            let mime = mime_guess::from_ext(&file_extension).first_or_octet_stream();

            form.add_stream(parameter_name, file, file_name, Some(mime));
        }

        let url = format!("{}/{}", self.api_url, method);
        let form_data = form.prepare().unwrap();
        let response = ureq::post(&url)
            .set(
                "Content-Type",
                &format!("multipart/form-data; boundary={}", form_data.boundary()),
            )
            .send(form_data)?;

        let parsed_response: T2 = serde_json::from_reader(response.into_reader()).unwrap();

        Ok(parsed_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_params::*;
    use crate::objects::*;

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

        let _m = mockito::mock("POST", "/getUpdates")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

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
        let _m = mockito::mock("POST", "/sendMessage")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.send_message(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_message_failure() {
        let response_string =
            "{\"ok\":false,\"description\":\"Bad Request: chat not found\",\"error_code\":400}";
        let params = SendMessageParams::new(ChatIdEnum::IsizeVariant(1), "Hello!".to_string());
        let _m = mockito::mock("POST", "/sendMessage")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.send_message(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn set_webhook_success() {
        let response_string =
            "{\"ok\":true,\"result\":true,\"description\":\"Webhook is already deleted\"}";
        let params = SetWebhookParams::new("".to_string());
        let _m = mockito::mock("POST", "/setWebhook")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.set_webhook(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn delete_webhook_success() {
        let response_string =
            "{\"ok\":true,\"result\":true,\"description\":\"Webhook is already deleted\"}";
        let params = DeleteWebhookParams::new();
        let _m = mockito::mock("POST", "/deleteWebhook")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.delete_webhook(params).unwrap();

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
        let api = API::new_url(mockito::server_url());

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
        let api = API::new_url(mockito::server_url());

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
        let api = API::new_url(mockito::server_url());

        let response = api.log_out().unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn close_failure() {
        let response_string = "{\"ok\":false,\"description\":\"Unauthorized\",\"error_code\":401}";

        let _m = mockito::mock("POST", "/close")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.close().unwrap();

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
        let api = API::new_url(mockito::server_url());

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

        let _m = mockito::mock("POST", "/forwardMessage")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

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

        let _m = mockito::mock("POST", "/copyMessage")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.copy_message(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_location_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2750,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618382060,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"location\":{\"longitude\":6.949981,\"latitude\":49.700002}}}";
        let params = SendLocationParams::new(ChatIdEnum::IsizeVariant(275808073), 49.7, 6.95);

        let _m = mockito::mock("POST", "/sendLocation")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

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

        let _m = mockito::mock("POST", "/editMessageLiveLocation")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

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

        let _m = mockito::mock("POST", "/stopMessageLiveLocation")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

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

        let _m = mockito::mock("POST", "/sendVenue")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

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

        let _m = mockito::mock("POST", "/sendContact")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

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

        let _m = mockito::mock("POST", "/sendPoll")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.send_poll(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_dice_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2757,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618467133,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"dice\":{\"emoji\":\"🎲\",\"value\":5}}}";
        let params = SendDiceParams::new(ChatIdEnum::IsizeVariant(275808073));

        let _m = mockito::mock("POST", "/sendDice")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.send_dice(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_chat_action_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params =
            SendChatActionParams::new(ChatIdEnum::IsizeVariant(275808073), "typing".to_string());

        let _m = mockito::mock("POST", "/sendChatAction")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.send_chat_action(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn get_user_profile_photos_success() {
        let response_string = "{\"ok\":true,\"result\":{\"total_count\":3,\"photos\":[[{\"file_id\":\"AgACAgIAAxUAAWB332IlzabFGWzaMrOdQ4ODVLyaAAKypzEbSX9wEEzMxT7F-grc3UA5DwAEAQADAgADYQADg0kCAAEfBA\",\"file_unique_id\":\"AQAD3UA5DwAEg0kCAAE\",\"width\":160,\"height\":160,\"file_size\":8068},{\"file_id\":\"AgACAgIAAxUAAWB332IlzabFGWzaMrOdQ4ODVLyaAAKypzEbSX9wEEzMxT7F-grc3UA5DwAEAQADAgADYgADhEkCAAEfBA\",\"file_unique_id\":\"AQAD3UA5DwAEhEkCAAE\",\"width\":320,\"height\":320,\"file_size\":22765},{\"file_id\":\"AgACAgIAAxUAAWB332IlzabFGWzaMrOdQ4ODVLyaAAKypzEbSX9wEEzMxT7F-grc3UA5DwAEAQADAgADYwADhUkCAAEfBA\",\"file_unique_id\":\"AQAD3UA5DwAEhUkCAAE\",\"width\":640,\"height\":640,\"file_size\":65663}],[{\"file_id\":\"AgACAgIAAxUAAWB332JpnZNv9ZNeZeIt1FFCdOroAAKwpzEbSX9wEOb5okUMX3tVSRdLDQAEAQADAgADYQADZj0KAAEfBA\",\"file_unique_id\":\"AQADSRdLDQAEZj0KAAE\",\"width\":160,\"height\":160,\"file_size\":13459},{\"file_id\":\"AgACAgIAAxUAAWB332JpnZNv9ZNeZeIt1FFCdOroAAKwpzEbSX9wEOb5okUMX3tVSRdLDQAEAQADAgADYgADZz0KAAEfBA\",\"file_unique_id\":\"AQADSRdLDQAEZz0KAAE\",\"width\":320,\"height\":320,\"file_size\":41243},{\"file_id\":\"AgACAgIAAxUAAWB332JpnZNv9ZNeZeIt1FFCdOroAAKwpzEbSX9wEOb5okUMX3tVSRdLDQAEAQADAgADYwADaD0KAAEfBA\",\"file_unique_id\":\"AQADSRdLDQAEaD0KAAE\",\"width\":640,\"height\":640,\"file_size\":114427}],[{\"file_id\":\"AgACAgIAAxUAAWB332ISVowq4pXLx3y1o-7WQteeAAKvpzEbSX9wEBlOkdDjqlYW1Du3DQAEAQADAgADYQADdkwAAh8E\",\"file_unique_id\":\"AQAD1Du3DQAEdkwAAg\",\"width\":160,\"height\":160,\"file_size\":6631},{\"file_id\":\"AgACAgIAAxUAAWB332ISVowq4pXLx3y1o-7WQteeAAKvpzEbSX9wEBlOkdDjqlYW1Du3DQAEAQADAgADYgADd0wAAh8E\",\"file_unique_id\":\"AQAD1Du3DQAEd0wAAg\",\"width\":320,\"height\":320,\"file_size\":20495},{\"file_id\":\"AgACAgIAAxUAAWB332ISVowq4pXLx3y1o-7WQteeAAKvpzEbSX9wEBlOkdDjqlYW1Du3DQAEAQADAgADYwADeEwAAh8E\",\"file_unique_id\":\"AQAD1Du3DQAEeEwAAg\",\"width\":640,\"height\":640,\"file_size\":54395}]]}}";
        let params = GetUserProfilePhotosParams::new(275808073);

        let _m = mockito::mock("POST", "/getUserProfilePhotos")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

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

        let _m = mockito::mock("POST", "/getFile")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.get_file(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn kick_chat_member_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = KickChatMemberParams::new(ChatIdEnum::IsizeVariant(-1001368460856), 275808073);

        let _m = mockito::mock("POST", "/kickChatMember")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.kick_chat_member(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn unban_chat_member_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params =
            UnbanChatMemberParams::new(ChatIdEnum::IsizeVariant(-1001368460856), 275808072);

        let _m = mockito::mock("POST", "/unbanChatMember")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.unban_chat_member(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn restrict_chat_member_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let mut perm = ChatPermissions::new();
        perm.set_can_add_web_page_previews(Some(true));
        let params = RestrictChatMemberParams::new(
            ChatIdEnum::IsizeVariant(-1001368460856),
            275808073,
            perm,
        );

        let _m = mockito::mock("POST", "/restrictChatMember")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.restrict_chat_member(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn promote_chat_member_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let mut params =
            PromoteChatMemberParams::new(ChatIdEnum::IsizeVariant(-1001368460856), 275808073);
        params.set_can_change_info(Some(true));

        let _m = mockito::mock("POST", "/promoteChatMember")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.promote_chat_member(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn set_chat_administrator_custom_title_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = SetChatAdministratorCustomTitleParams::new(
            ChatIdEnum::IsizeVariant(-1001368460856),
            275808073,
            "King".to_string(),
        );

        let _m = mockito::mock("POST", "/setChatAdministratorCustomTitle")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.set_chat_administrator_custom_title(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn set_chat_permissions_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let mut perm = ChatPermissions::new();
        perm.set_can_add_web_page_previews(Some(true));

        let params = SetChatPermissionsParams::new(ChatIdEnum::IsizeVariant(-1001368460856), perm);

        let _m = mockito::mock("POST", "/setChatPermissions")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.set_chat_permissions(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn export_chat_invite_link_success() {
        let response_string = "{\"ok\":true,\"result\":\"https://t.me/joinchat/txIUDwjfk7M2ODEy\"}";

        let params = ExportChatInviteLinkParams::new(ChatIdEnum::IsizeVariant(-1001368460856));

        let _m = mockito::mock("POST", "/exportChatInviteLink")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.export_chat_invite_link(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn create_chat_invite_link_success() {
        let response_string = "{\"ok\":true,\"result\":{\"invite_link\":\"https://t.me/joinchat/yFEnSaeiQm83ZTNi\",\"creator\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"is_primary\":false,\"is_revoked\":false}}";

        let params = CreateChatInviteLinkParams::new(ChatIdEnum::IsizeVariant(-1001368460856));

        let _m = mockito::mock("POST", "/createChatInviteLink")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.create_chat_invite_link(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn edit_chat_invite_link_success() {
        let response_string = "{\"ok\":true,\"result\":{\"invite_link\":\"https://t.me/joinchat/O458bA8hQ0MzNmQy\",\"creator\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"is_primary\":false,\"is_revoked\":false}}";

        let params = EditChatInviteLinkParams::new(
            ChatIdEnum::IsizeVariant(-1001368460856),
            "https://t.me/joinchat/O458bA8hQ0MzNmQy".to_string(),
        );

        let _m = mockito::mock("POST", "/editChatInviteLink")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.edit_chat_invite_link(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn revoke_chat_invite_link_success() {
        let response_string = "{\"ok\":true,\"result\":{\"invite_link\":\"https://t.me/joinchat/O458bA8hQ0MzNmQy\",\"creator\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"is_primary\":false,\"is_revoked\":true}}";

        let params = RevokeChatInviteLinkParams::new(
            ChatIdEnum::IsizeVariant(-1001368460856),
            "https://t.me/joinchat/O458bA8hQ0MzNmQy".to_string(),
        );

        let _m = mockito::mock("POST", "/revokeChatInviteLink")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.revoke_chat_invite_link(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn delete_chat_photo_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = DeleteChatPhotoParams::new(ChatIdEnum::IsizeVariant(-1001368460856));

        let _m = mockito::mock("POST", "/deleteChatPhoto")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.delete_chat_photo(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_photo_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2763,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618730180,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"photo\":[{\"file_id\":\"AgACAgIAAxkDAAIKy2B73MQXIhoDDmLXjcUjgqGf-m8bAALjsDEbORLgS-s4BkBzcC5DYvIBny4AAwEAAwIAA20AA0U3AwABHwQ\",\"file_unique_id\":\"AQADYvIBny4AA0U3AwAB\",\"width\":320,\"height\":320,\"file_size\":19968},{\"file_id\":\"AgACAgIAAxkDAAIKy2B73MQXIhoDDmLXjcUjgqGf-m8bAALjsDEbORLgS-s4BkBzcC5DYvIBny4AAwEAAwIAA3gAA0Y3AwABHwQ\",\"file_unique_id\":\"AQADYvIBny4AA0Y3AwAB\",\"width\":799,\"height\":800,\"file_size\":63581},{\"file_id\":\"AgACAgIAAxkDAAIKy2B73MQXIhoDDmLXjcUjgqGf-m8bAALjsDEbORLgS-s4BkBzcC5DYvIBny4AAwEAAwIAA3kAA0M3AwABHwQ\",\"file_unique_id\":\"AQADYvIBny4AA0M3AwAB\",\"width\":847,\"height\":848,\"file_size\":63763}]}}";
        let params = SendPhotoParams::new(
            ChatIdEnum::IsizeVariant(275808073),
            PhotoEnum::InputFileVariant(InputFile::new(std::path::PathBuf::from(
                "./frankenstein_logo.png",
            ))),
        );

        let _m = mockito::mock("POST", "/sendPhoto")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.send_photo(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_audio_file_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2766,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618735176,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"audio\":{\"file_id\":\"CQACAgIAAxkDAAIKzmB78EjK-iOHo-HKC-M6p4r0jGdmAALkDAACORLgS5co1z0uFAKgHwQ\",\"file_unique_id\":\"AgAD5AwAAjkS4Es\",\"duration\":123,\"title\":\"Way Back Home\",\"file_name\":\"audio.mp3\",\"mime_type\":\"audio/mpeg\",\"file_size\":2957092}}}";
        let params = SendAudioParams::new(
            ChatIdEnum::IsizeVariant(275808073),
            AudioEnum::InputFileVariant(InputFile::new(std::path::PathBuf::from(
                "./frankenstein_logo.png",
            ))),
        );
        let _m = mockito::mock("POST", "/sendAudio")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.send_audio(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_audio_file_with_thumb_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2766,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618735176,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"audio\":{\"file_id\":\"CQACAgIAAxkDAAIKzmB78EjK-iOHo-HKC-M6p4r0jGdmAALkDAACORLgS5co1z0uFAKgHwQ\",\"file_unique_id\":\"AgAD5AwAAjkS4Es\",\"duration\":123,\"title\":\"Way Back Home\",\"file_name\":\"audio.mp3\",\"mime_type\":\"audio/mpeg\",\"file_size\":2957092}}}";
        let mut params = SendAudioParams::new(
            ChatIdEnum::IsizeVariant(275808073),
            AudioEnum::InputFileVariant(InputFile::new(std::path::PathBuf::from(
                "./frankenstein_logo.png",
            ))),
        );
        params.set_thumb(Some(ThumbEnum::InputFileVariant(InputFile::new(
            std::path::PathBuf::from("./frankenstein_logo.png"),
        ))));
        let _m = mockito::mock("POST", "/sendAudio")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.send_audio(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_audio_file_id_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2769,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618735333,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"audio\":{\"file_id\":\"CQACAgIAAxkDAAIK0WB78OUFavWx6fjzCQ_d5qnu_R7mAALkDAACORLgS5co1z0uFAKgHwQ\",\"file_unique_id\":\"AgAD5AwAAjkS4Es\",\"duration\":123,\"title\":\"Way Back Home\",\"file_name\":\"audio.mp3\",\"mime_type\":\"audio/mpeg\",\"file_size\":2957092}}}";
        let params = SendAudioParams::new(
            ChatIdEnum::IsizeVariant(275808073),
            AudioEnum::StringVariant(
                "CQACAgIAAxkDAAIKzmB78EjK-iOHo-HKC-M6p4r0jGdmAALkDAACORLgS5co1z0uFAKgHwQ"
                    .to_string(),
            ),
        );
        let _m = mockito::mock("POST", "/sendAudio")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.send_audio(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_document_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2770,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618737593,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"audio\":{\"file_id\":\"CQACAgIAAxkDAAIK0mB7-bnnewABfdaFKK4NzVLQ7BvgCwAC6gwAAjkS4Et_njaNR8IUMB8E\",\"file_unique_id\":\"AgAD6gwAAjkS4Es\",\"duration\":123,\"title\":\"Way Back Home\",\"file_name\":\"audio.mp3\",\"mime_type\":\"audio/mpeg\",\"file_size\":2957092}}}";
        let params = SendDocumentParams::new(
            ChatIdEnum::IsizeVariant(275808073),
            DocumentEnum::InputFileVariant(InputFile::new(std::path::PathBuf::from(
                "./frankenstein_logo.png",
            ))),
        );
        let _m = mockito::mock("POST", "/sendDocument")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.send_document(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_video_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2770,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618737593,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"audio\":{\"file_id\":\"CQACAgIAAxkDAAIK0mB7-bnnewABfdaFKK4NzVLQ7BvgCwAC6gwAAjkS4Et_njaNR8IUMB8E\",\"file_unique_id\":\"AgAD6gwAAjkS4Es\",\"duration\":123,\"title\":\"Way Back Home\",\"file_name\":\"audio.mp3\",\"mime_type\":\"audio/mpeg\",\"file_size\":2957092}}}";
        let params = SendVideoParams::new(
            ChatIdEnum::IsizeVariant(275808073),
            VideoEnum::InputFileVariant(InputFile::new(std::path::PathBuf::from(
                "./frankenstein_logo.png",
            ))),
        );
        let _m = mockito::mock("POST", "/sendVideo")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.send_video(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_animation_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2770,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618737593,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"audio\":{\"file_id\":\"CQACAgIAAxkDAAIK0mB7-bnnewABfdaFKK4NzVLQ7BvgCwAC6gwAAjkS4Et_njaNR8IUMB8E\",\"file_unique_id\":\"AgAD6gwAAjkS4Es\",\"duration\":123,\"title\":\"Way Back Home\",\"file_name\":\"audio.mp3\",\"mime_type\":\"audio/mpeg\",\"file_size\":2957092}}}";
        let params = SendAnimationParams::new(
            ChatIdEnum::IsizeVariant(275808073),
            AnimationEnum::InputFileVariant(InputFile::new(std::path::PathBuf::from(
                "./frankenstein_logo.png",
            ))),
        );
        let _m = mockito::mock("POST", "/sendAnimation")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.send_animation(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_voice_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2770,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618737593,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"audio\":{\"file_id\":\"CQACAgIAAxkDAAIK0mB7-bnnewABfdaFKK4NzVLQ7BvgCwAC6gwAAjkS4Et_njaNR8IUMB8E\",\"file_unique_id\":\"AgAD6gwAAjkS4Es\",\"duration\":123,\"title\":\"Way Back Home\",\"file_name\":\"audio.mp3\",\"mime_type\":\"audio/mpeg\",\"file_size\":2957092}}}";
        let params = SendVoiceParams::new(
            ChatIdEnum::IsizeVariant(275808073),
            VoiceEnum::InputFileVariant(InputFile::new(std::path::PathBuf::from(
                "./frankenstein_logo.png",
            ))),
        );
        let _m = mockito::mock("POST", "/sendVoice")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.send_voice(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn send_video_note_success() {
        let response_string = "{\"ok\":true,\"result\":{\"message_id\":2770,\"from\":{\"id\":1276618370,\"is_bot\":true,\"first_name\":\"test_el_bot\",\"username\":\"el_mon_test_bot\"},\"date\":1618737593,\"chat\":{\"id\":275808073,\"type\":\"private\",\"username\":\"Ayrat555\",\"first_name\":\"Ayrat\",\"last_name\":\"Badykov\"},\"audio\":{\"file_id\":\"CQACAgIAAxkDAAIK0mB7-bnnewABfdaFKK4NzVLQ7BvgCwAC6gwAAjkS4Et_njaNR8IUMB8E\",\"file_unique_id\":\"AgAD6gwAAjkS4Es\",\"duration\":123,\"title\":\"Way Back Home\",\"file_name\":\"audio.mp3\",\"mime_type\":\"audio/mpeg\",\"file_size\":2957092}}}";
        let params = SendVideoNoteParams::new(
            ChatIdEnum::IsizeVariant(275808073),
            VideoNoteEnum::InputFileVariant(InputFile::new(std::path::PathBuf::from(
                "./frankenstein_logo.png",
            ))),
        );
        let _m = mockito::mock("POST", "/sendVideoNote")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.send_video_note(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn set_chat_photo_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = SetChatPhotoParams::new(
            ChatIdEnum::IsizeVariant(-1001368460856),
            InputFile::new(std::path::PathBuf::from("./frankenstein_logo.png")),
        );

        let _m = mockito::mock("POST", "/setChatPhoto")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.set_chat_photo(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn set_chat_title_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = SetChatTitleParams::new(
            ChatIdEnum::IsizeVariant(-1001368460856),
            "Frankenstein".to_string(),
        );

        let _m = mockito::mock("POST", "/setChatTitle")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.set_chat_title(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn set_chat_description_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let mut params = SetChatDescriptionParams::new(ChatIdEnum::IsizeVariant(-1001368460856));
        params.set_description(Some("Frankenstein group".to_string()));

        let _m = mockito::mock("POST", "/setChatDescription")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.set_chat_description(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }

    #[test]
    fn pin_chat_message_success() {
        let response_string = "{\"ok\":true,\"result\":true}";
        let params = PinChatMessageParams::new(ChatIdEnum::IsizeVariant(275808073), 2766);

        let _m = mockito::mock("POST", "/pinChatMessage")
            .with_status(200)
            .with_body(response_string)
            .create();
        let api = API::new_url(mockito::server_url());

        let response = api.pin_chat_message(params).unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(response_string, json);
    }
}
