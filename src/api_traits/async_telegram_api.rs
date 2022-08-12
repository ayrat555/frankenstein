use super::EditMessageResponse;
use super::MethodResponse;
use crate::api_params::AddStickerToSetParams;
use crate::api_params::AnswerCallbackQueryParams;
use crate::api_params::AnswerInlineQueryParams;
use crate::api_params::AnswerPreCheckoutQueryParams;
use crate::api_params::AnswerShippingQueryParams;
use crate::api_params::AnswerWebAppQueryParams;
use crate::api_params::ApproveChatJoinRequestParams;
use crate::api_params::BanChatMemberParams;
use crate::api_params::BanChatSenderChatParams;
use crate::api_params::CopyMessageParams;
use crate::api_params::CreateChatInviteLinkParams;
use crate::api_params::CreateInvoiceLinkParams;
use crate::api_params::CreateNewStickerSetParams;
use crate::api_params::DeclineChatJoinRequestParams;
use crate::api_params::DeleteChatPhotoParams;
use crate::api_params::DeleteChatStickerSetParams;
use crate::api_params::DeleteMessageParams;
use crate::api_params::DeleteMyCommandsParams;
use crate::api_params::DeleteStickerFromSetParams;
use crate::api_params::DeleteWebhookParams;
use crate::api_params::EditChatInviteLinkParams;
use crate::api_params::EditMessageCaptionParams;
use crate::api_params::EditMessageLiveLocationParams;
use crate::api_params::EditMessageMediaParams;
use crate::api_params::EditMessageReplyMarkupParams;
use crate::api_params::EditMessageTextParams;
use crate::api_params::ExportChatInviteLinkParams;
use crate::api_params::File;
use crate::api_params::ForwardMessageParams;
use crate::api_params::GetChatAdministratorsParams;
use crate::api_params::GetChatMemberCountParams;
use crate::api_params::GetChatMemberParams;
use crate::api_params::GetChatMenuButtonParams;
use crate::api_params::GetChatParams;
use crate::api_params::GetCustomEmojiStickersParams;
use crate::api_params::GetFileParams;
use crate::api_params::GetGameHighScoresParams;
use crate::api_params::GetMyCommandsParams;
use crate::api_params::GetMyDefaultAdministratorRightsParams;
use crate::api_params::GetStickerSetParams;
use crate::api_params::GetUpdatesParams;
use crate::api_params::GetUserProfilePhotosParams;
use crate::api_params::InputMedia;
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
use crate::api_params::SendGameParams;
use crate::api_params::SendInvoiceParams;
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
use crate::api_params::SetChatMenuButtonParams;
use crate::api_params::SetChatPermissionsParams;
use crate::api_params::SetChatPhotoParams;
use crate::api_params::SetChatStickerSetParams;
use crate::api_params::SetChatTitleParams;
use crate::api_params::SetGameScoreParams;
use crate::api_params::SetMyCommandsParams;
use crate::api_params::SetMyDefaultAdministratorRightsParams;
use crate::api_params::SetStickerPositionInSetParams;
use crate::api_params::SetStickerSetThumbParams;
use crate::api_params::SetWebhookParams;
use crate::api_params::StopMessageLiveLocationParams;
use crate::api_params::StopPollParams;
use crate::api_params::UnbanChatMemberParams;
use crate::api_params::UnbanChatSenderChatParams;
use crate::api_params::UnpinChatMessageParams;
use crate::api_params::UploadStickerFileParams;
use crate::objects::BotCommand;
use crate::objects::Chat;
use crate::objects::ChatAdministratorRights;
use crate::objects::ChatInviteLink;
use crate::objects::ChatMember;
use crate::objects::File as FileObject;
use crate::objects::GameHighScore;
use crate::objects::MenuButton;
use crate::objects::Message;
use crate::objects::MessageId;
use crate::objects::Poll;
use crate::objects::SentWebAppMessage;
use crate::objects::StickerSet;
use crate::objects::Update;
use crate::objects::User;
use crate::objects::UserProfilePhotos;
use crate::objects::WebhookInfo;
use async_trait::async_trait;
use std::path::PathBuf;

#[async_trait]
pub trait AsyncTelegramApi {
    type Error;

    async fn get_updates(
        &self,
        params: &GetUpdatesParams,
    ) -> Result<MethodResponse<Vec<Update>>, Self::Error> {
        self.request("getUpdates", Some(params)).await
    }

    async fn send_message(
        &self,
        params: &SendMessageParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        self.request("sendMessage", Some(params)).await
    }

    async fn set_webhook(
        &self,
        params: &SetWebhookParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setWebhook", Some(params)).await
    }

    async fn delete_webhook(
        &self,
        params: &DeleteWebhookParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("deleteWebhook", Some(params)).await
    }

    async fn get_webhook_info(&self) -> Result<MethodResponse<WebhookInfo>, Self::Error> {
        self.request_without_body("getWebhookInfo").await
    }

    async fn get_me(&self) -> Result<MethodResponse<User>, Self::Error> {
        self.request_without_body("getMe").await
    }

    async fn log_out(&self) -> Result<MethodResponse<bool>, Self::Error> {
        self.request_without_body("logOut").await
    }

    async fn close(&self) -> Result<MethodResponse<bool>, Self::Error> {
        self.request_without_body("close").await
    }

    async fn forward_message(
        &self,
        params: &ForwardMessageParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        self.request("forwardMessage", Some(params)).await
    }

    async fn copy_message(
        &self,
        params: &CopyMessageParams,
    ) -> Result<MethodResponse<MessageId>, Self::Error> {
        self.request("copyMessage", Some(params)).await
    }

    async fn send_photo(
        &self,
        params: &SendPhotoParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendPhoto";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let File::InputFile(input_file) = &params.photo {
            files.push(("photo", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
            .await
    }

    async fn send_audio(
        &self,
        params: &SendAudioParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendAudio";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let File::InputFile(input_file) = &params.audio {
            files.push(("audio", input_file.path.clone()));
        }

        if let Some(File::InputFile(input_file)) = &params.thumb {
            files.push(("thumb", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
            .await
    }

    async fn send_media_group(
        &self,
        params: &SendMediaGroupParams,
    ) -> Result<MethodResponse<Vec<Message>>, Self::Error> {
        let method_name = "sendMediaGroup";
        let mut files: Vec<(String, PathBuf)> = vec![];
        let mut new_medias: Vec<Media> = vec![];
        let mut file_idx = 0;

        for media in &params.media {
            match media {
                Media::Audio(audio) => {
                    let mut new_audio = audio.clone();

                    if let File::InputFile(input_file) = &audio.media {
                        let name = format!("file{}", file_idx);
                        let attach_name = format!("attach://{}", name);
                        file_idx += 1;

                        new_audio.media = File::String(attach_name);

                        files.push((name, input_file.path.clone()));
                    };

                    if let Some(File::InputFile(input_file)) = &audio.thumb {
                        let name = format!("file{}", file_idx);
                        let attach_name = format!("attach://{}", name);
                        file_idx += 1;

                        new_audio.thumb = Some(File::String(attach_name));

                        files.push((name, input_file.path.clone()));
                    };

                    new_medias.push(Media::Audio(new_audio));
                }

                Media::Document(document) => {
                    let mut new_document = document.clone();

                    if let File::InputFile(input_file) = &document.media {
                        let name = format!("file{}", file_idx);
                        let attach_name = format!("attach://{}", name);
                        file_idx += 1;

                        new_document.media = File::String(attach_name);

                        files.push((name, input_file.path.clone()));
                    };

                    new_medias.push(Media::Document(new_document));
                }
                Media::Photo(photo) => {
                    let mut new_photo = photo.clone();

                    if let File::InputFile(input_file) = &photo.media {
                        let name = format!("file{}", file_idx);
                        let attach_name = format!("attach://{}", name);
                        file_idx += 1;

                        new_photo.media = File::String(attach_name);

                        files.push((name, input_file.path.clone()));
                    };

                    new_medias.push(Media::Photo(new_photo));
                }

                Media::Video(video) => {
                    let mut new_video = video.clone();

                    if let File::InputFile(input_file) = &video.media {
                        let name = format!("file{}", file_idx);
                        let attach_name = format!("attach://{}", name);
                        file_idx += 1;

                        new_video.media = File::String(attach_name);

                        files.push((name, input_file.path.clone()));
                    };

                    if let Some(File::InputFile(input_file)) = &video.thumb {
                        let name = format!("file{}", file_idx);
                        let attach_name = format!("attach://{}", name);
                        file_idx += 1;

                        new_video.thumb = Some(File::String(attach_name));

                        files.push((name, input_file.path.clone()));
                    };

                    new_medias.push(Media::Video(new_video));
                }
            };
        }

        let mut new_params = params.clone();
        new_params.media = new_medias;

        let files_with_str_names: Vec<(&str, PathBuf)> = files
            .iter()
            .map(|(key, path)| (key.as_str(), path.clone()))
            .collect();

        self.request_with_possible_form_data(method_name, &new_params, files_with_str_names)
            .await
    }

    async fn send_document(
        &self,
        params: &SendDocumentParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendDocument";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let File::InputFile(input_file) = &params.document {
            files.push(("document", input_file.path.clone()));
        }

        if let Some(File::InputFile(input_file)) = &params.thumb {
            files.push(("thumb", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
            .await
    }

    async fn send_video(
        &self,
        params: &SendVideoParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendVideo";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let File::InputFile(input_file) = &params.video {
            files.push(("video", input_file.path.clone()));
        }

        if let Some(File::InputFile(input_file)) = &params.thumb {
            files.push(("thumb", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
            .await
    }

    async fn send_animation(
        &self,
        params: &SendAnimationParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendAnimation";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let File::InputFile(input_file) = &params.animation {
            files.push(("animation", input_file.path.clone()));
        }

        if let Some(File::InputFile(input_file)) = &params.thumb {
            files.push(("thumb", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
            .await
    }

    async fn send_voice(
        &self,
        params: &SendVoiceParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendVoice";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let File::InputFile(input_file) = &params.voice {
            files.push(("voice", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
            .await
    }

    async fn send_video_note(
        &self,
        params: &SendVideoNoteParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendVideoNote";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let File::InputFile(input_file) = &params.video_note {
            files.push(("video_note", input_file.path.clone()));
        }

        if let Some(File::InputFile(input_file)) = &params.thumb {
            files.push(("thumb", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
            .await
    }

    async fn send_location(
        &self,
        params: &SendLocationParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        self.request("sendLocation", Some(params)).await
    }

    async fn edit_message_live_location(
        &self,
        params: &EditMessageLiveLocationParams,
    ) -> Result<EditMessageResponse, Self::Error> {
        self.request("editMessageLiveLocation", Some(params)).await
    }

    async fn stop_message_live_location(
        &self,
        params: &StopMessageLiveLocationParams,
    ) -> Result<EditMessageResponse, Self::Error> {
        self.request("stopMessageLiveLocation", Some(params)).await
    }

    async fn send_venue(
        &self,
        params: &SendVenueParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        self.request("sendVenue", Some(params)).await
    }

    async fn send_contact(
        &self,
        params: &SendContactParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        self.request("sendContact", Some(params)).await
    }

    async fn send_poll(
        &self,
        params: &SendPollParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        self.request("sendPoll", Some(params)).await
    }

    async fn send_dice(
        &self,
        params: &SendDiceParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        self.request("sendDice", Some(params)).await
    }

    async fn send_chat_action(
        &self,
        params: &SendChatActionParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("sendChatAction", Some(params)).await
    }

    async fn get_user_profile_photos(
        &self,
        params: &GetUserProfilePhotosParams,
    ) -> Result<MethodResponse<UserProfilePhotos>, Self::Error> {
        self.request("getUserProfilePhotos", Some(params)).await
    }

    async fn get_file(
        &self,
        params: &GetFileParams,
    ) -> Result<MethodResponse<FileObject>, Self::Error> {
        self.request("getFile", Some(params)).await
    }

    async fn ban_chat_member(
        &self,
        params: &BanChatMemberParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("banChatMember", Some(params)).await
    }

    async fn unban_chat_member(
        &self,
        params: &UnbanChatMemberParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("unbanChatMember", Some(params)).await
    }

    async fn restrict_chat_member(
        &self,
        params: &RestrictChatMemberParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("restrictChatMember", Some(params)).await
    }

    async fn promote_chat_member(
        &self,
        params: &PromoteChatMemberParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("promoteChatMember", Some(params)).await
    }

    async fn set_chat_administrator_custom_title(
        &self,
        params: &SetChatAdministratorCustomTitleParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setChatAdministratorCustomTitle", Some(params))
            .await
    }

    async fn ban_chat_sender_chat(
        &self,
        params: &BanChatSenderChatParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("banChatSenderChat", Some(params)).await
    }

    async fn unban_chat_sender_chat(
        &self,
        params: &UnbanChatSenderChatParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("unbanChatSenderChat", Some(params)).await
    }

    async fn set_chat_permissions(
        &self,
        params: &SetChatPermissionsParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setChatPermissions", Some(params)).await
    }

    async fn export_chat_invite_link(
        &self,
        params: &ExportChatInviteLinkParams,
    ) -> Result<MethodResponse<String>, Self::Error> {
        self.request("exportChatInviteLink", Some(params)).await
    }

    async fn create_chat_invite_link(
        &self,
        params: &CreateChatInviteLinkParams,
    ) -> Result<MethodResponse<ChatInviteLink>, Self::Error> {
        self.request("createChatInviteLink", Some(params)).await
    }

    async fn edit_chat_invite_link(
        &self,
        params: &EditChatInviteLinkParams,
    ) -> Result<MethodResponse<ChatInviteLink>, Self::Error> {
        self.request("editChatInviteLink", Some(params)).await
    }

    async fn revoke_chat_invite_link(
        &self,
        params: &RevokeChatInviteLinkParams,
    ) -> Result<MethodResponse<ChatInviteLink>, Self::Error> {
        self.request("revokeChatInviteLink", Some(params)).await
    }

    async fn approve_chat_join_request(
        &self,
        params: &ApproveChatJoinRequestParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("approveChatJoinRequest", Some(params)).await
    }

    async fn decline_chat_join_request(
        &self,
        params: &DeclineChatJoinRequestParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("declineChatJoinRequest", Some(params)).await
    }

    async fn set_chat_photo(
        &self,
        params: &SetChatPhotoParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let photo = &params.photo;

        self.request_with_form_data("setChatPhoto", params, vec![("photo", photo.path.clone())])
            .await
    }

    async fn delete_chat_photo(
        &self,
        params: &DeleteChatPhotoParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("deleteChatPhoto", Some(params)).await
    }

    async fn set_chat_title(
        &self,
        params: &SetChatTitleParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setChatTitle", Some(params)).await
    }

    async fn set_chat_description(
        &self,
        params: &SetChatDescriptionParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setChatDescription", Some(params)).await
    }

    async fn pin_chat_message(
        &self,
        params: &PinChatMessageParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("pinChatMessage", Some(params)).await
    }

    async fn unpin_chat_message(
        &self,
        params: &UnpinChatMessageParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("unpinChatMessage", Some(params)).await
    }

    async fn leave_chat(
        &self,
        params: &LeaveChatParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("leaveChat", Some(params)).await
    }

    async fn get_chat(&self, params: &GetChatParams) -> Result<MethodResponse<Chat>, Self::Error> {
        self.request("getChat", Some(params)).await
    }

    async fn get_chat_administrators(
        &self,
        params: &GetChatAdministratorsParams,
    ) -> Result<MethodResponse<Vec<ChatMember>>, Self::Error> {
        self.request("getChatAdministrators", Some(params)).await
    }

    async fn get_chat_member_count(
        &self,
        params: &GetChatMemberCountParams,
    ) -> Result<MethodResponse<u32>, Self::Error> {
        self.request("getChatMemberCount", Some(params)).await
    }

    async fn get_chat_member(
        &self,
        params: &GetChatMemberParams,
    ) -> Result<MethodResponse<ChatMember>, Self::Error> {
        self.request("getChatMember", Some(params)).await
    }

    async fn set_chat_sticker_set(
        &self,
        params: &SetChatStickerSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setChatStickerSet", Some(params)).await
    }

    async fn delete_chat_sticker_set(
        &self,
        params: &DeleteChatStickerSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("deleteChatStickerSet", Some(params)).await
    }

    async fn answer_callback_query(
        &self,
        params: &AnswerCallbackQueryParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("answerCallbackQuery", Some(params)).await
    }

    async fn set_my_commands(
        &self,
        params: &SetMyCommandsParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setMyCommands", Some(params)).await
    }

    async fn get_my_commands(
        &self,
        params: &GetMyCommandsParams,
    ) -> Result<MethodResponse<Vec<BotCommand>>, Self::Error> {
        self.request("getMyCommands", Some(params)).await
    }

    async fn delete_my_commands(
        &self,
        params: &DeleteMyCommandsParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("deleteMyCommands", Some(params)).await
    }

    async fn answer_inline_query(
        &self,
        params: &AnswerInlineQueryParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("answerInlineQuery", Some(params)).await
    }

    async fn edit_message_text(
        &self,
        params: &EditMessageTextParams,
    ) -> Result<EditMessageResponse, Self::Error> {
        self.request("editMessageText", Some(params)).await
    }

    async fn edit_message_caption(
        &self,
        params: &EditMessageCaptionParams,
    ) -> Result<EditMessageResponse, Self::Error> {
        self.request("editMessageCaption", Some(params)).await
    }

    async fn edit_message_media(
        &self,
        params: &EditMessageMediaParams,
    ) -> Result<EditMessageResponse, Self::Error> {
        let method_name = "editMessageMedia";
        let mut files: Vec<(String, PathBuf)> = vec![];

        let new_media = match &params.media {
            InputMedia::Animation(animation) => {
                let mut new_animation = animation.clone();

                if let File::InputFile(input_file) = &animation.media {
                    let name = "animation".to_string();
                    let attach_name = format!("attach://{}", name);

                    new_animation.media = File::String(attach_name);

                    files.push((name, input_file.path.clone()));
                };

                if let Some(File::InputFile(input_file)) = &animation.thumb {
                    let name = "animation_thumb".to_string();
                    let attach_name = format!("attach://{}", name);

                    new_animation.thumb = Some(File::String(attach_name));

                    files.push((name, input_file.path.clone()));
                };

                InputMedia::Animation(new_animation)
            }
            InputMedia::Document(document) => {
                let mut new_document = document.clone();

                if let File::InputFile(input_file) = &document.media {
                    let name = "document".to_string();
                    let attach_name = format!("attach://{}", name);

                    new_document.media = File::String(attach_name);

                    files.push((name, input_file.path.clone()));
                };

                if let Some(File::InputFile(input_file)) = &document.thumb {
                    let name = "document_thumb".to_string();
                    let attach_name = format!("attach://{}", name);

                    new_document.thumb = Some(File::String(attach_name));

                    files.push((name, input_file.path.clone()));
                };

                InputMedia::Document(new_document)
            }
            InputMedia::Audio(audio) => {
                let mut new_audio = audio.clone();

                if let File::InputFile(input_file) = &audio.media {
                    let name = "audio".to_string();
                    let attach_name = format!("attach://{}", name);

                    new_audio.media = File::String(attach_name);

                    files.push((name, input_file.path.clone()));
                };

                if let Some(File::InputFile(input_file)) = &audio.thumb {
                    let name = "audio_thumb".to_string();
                    let attach_name = format!("attach://{}", name);

                    new_audio.thumb = Some(File::String(attach_name));

                    files.push((name, input_file.path.clone()));
                };

                InputMedia::Audio(new_audio)
            }
            InputMedia::Photo(photo) => {
                let mut new_photo = photo.clone();

                if let File::InputFile(input_file) = &photo.media {
                    let name = "photo".to_string();
                    let attach_name = format!("attach://{}", name);

                    new_photo.media = File::String(attach_name);

                    files.push((name, input_file.path.clone()));
                };

                InputMedia::Photo(new_photo)
            }
            InputMedia::Video(video) => {
                let mut new_video = video.clone();

                if let File::InputFile(input_file) = &video.media {
                    let name = "video".to_string();
                    let attach_name = format!("attach://{}", name);

                    new_video.media = File::String(attach_name);

                    files.push((name, input_file.path.clone()));
                };

                if let Some(File::InputFile(input_file)) = &video.thumb {
                    let name = "video_thumb".to_string();
                    let attach_name = format!("attach://{}", name);

                    new_video.thumb = Some(File::String(attach_name));

                    files.push((name, input_file.path.clone()));
                };

                InputMedia::Video(new_video)
            }
        };

        let mut new_params = params.clone();
        new_params.media = new_media;

        let files_with_str_names: Vec<(&str, PathBuf)> = files
            .iter()
            .map(|(key, path)| (key.as_str(), path.clone()))
            .collect();

        self.request_with_possible_form_data(method_name, &new_params, files_with_str_names)
            .await
    }

    async fn edit_message_reply_markup(
        &self,
        params: &EditMessageReplyMarkupParams,
    ) -> Result<EditMessageResponse, Self::Error> {
        self.request("editMessageReplyMarkup", Some(params)).await
    }

    async fn stop_poll(
        &self,
        params: &StopPollParams,
    ) -> Result<MethodResponse<Poll>, Self::Error> {
        self.request("stopPoll", Some(params)).await
    }

    async fn delete_message(
        &self,
        params: &DeleteMessageParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("deleteMessage", Some(params)).await
    }

    async fn send_sticker(
        &self,
        params: &SendStickerParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendSticker";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let File::InputFile(input_file) = &params.sticker {
            files.push(("sticker", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
            .await
    }

    async fn get_sticker_set(
        &self,
        params: &GetStickerSetParams,
    ) -> Result<MethodResponse<StickerSet>, Self::Error> {
        self.request("getStickerSet", Some(params)).await
    }

    async fn upload_sticker_file(
        &self,
        params: &UploadStickerFileParams,
    ) -> Result<MethodResponse<File>, Self::Error> {
        let sticker = &params.png_sticker;

        self.request_with_form_data(
            "uploadStickerFile",
            params,
            vec![("png_sticker", sticker.path.clone())],
        )
        .await
    }

    async fn create_new_sticker_set(
        &self,
        params: &CreateNewStickerSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let method_name = "createNewStickerSet";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let Some(File::InputFile(input_file)) = &params.png_sticker {
            files.push(("png_sticker", input_file.path.clone()));
        }

        if let Some(input_file) = &params.tgs_sticker {
            files.push(("tgs_sticker", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
            .await
    }

    async fn get_custom_emoji_stickers(
        &self,
        params: &GetCustomEmojiStickersParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("getCustomEmojiStickers", Some(params)).await
    }
    async fn add_sticker_to_set(
        &self,
        params: &AddStickerToSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let method_name = "addStickerToSet";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let Some(File::InputFile(input_file)) = &params.png_sticker {
            files.push(("png_sticker", input_file.path.clone()));
        }

        if let Some(input_file) = &params.tgs_sticker {
            files.push(("tgs_sticker", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
            .await
    }

    async fn set_sticker_position_in_set(
        &self,
        params: &SetStickerPositionInSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setStickerPositionInSet", Some(params)).await
    }

    async fn delete_sticker_from_set(
        &self,
        params: &DeleteStickerFromSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("deleteStickerFromSet", Some(params)).await
    }

    async fn set_sticker_set_thumb(
        &self,
        params: &SetStickerSetThumbParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let method_name = "setStickerSetThumb";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let Some(File::InputFile(input_file)) = &params.thumb {
            files.push(("thumb", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
            .await
    }

    async fn send_invoice(
        &self,
        params: &SendInvoiceParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        self.request("sendInvoice", Some(params)).await
    }

    async fn create_invoice_link(
        &self,
        params: &CreateInvoiceLinkParams,
    ) -> Result<MethodResponse<String>, Self::Error> {
        self.request("createInvoiceLink", Some(params)).await
    }

    async fn answer_shipping_query(
        &self,
        params: &AnswerShippingQueryParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("answerShippingQuery", Some(params)).await
    }

    async fn answer_pre_checkout_query(
        &self,
        params: &AnswerPreCheckoutQueryParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("answerPreCheckoutQuery", Some(params)).await
    }

    async fn send_game(
        &self,
        params: &SendGameParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        self.request("sendGame", Some(params)).await
    }

    async fn set_game_score(
        &self,
        params: &SetGameScoreParams,
    ) -> Result<EditMessageResponse, Self::Error> {
        self.request("setGameScore", Some(params)).await
    }

    async fn get_game_high_scores(
        &self,
        params: &GetGameHighScoresParams,
    ) -> Result<MethodResponse<Vec<GameHighScore>>, Self::Error> {
        self.request("getGameHighScores", Some(params)).await
    }

    async fn set_my_default_administrator_rights(
        &self,
        params: &SetMyDefaultAdministratorRightsParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setMyDefaultAdministratorRights", Some(params))
            .await
    }

    async fn get_my_default_administrator_rights(
        &self,
        params: &GetMyDefaultAdministratorRightsParams,
    ) -> Result<MethodResponse<ChatAdministratorRights>, Self::Error> {
        self.request("getMyDefaultAdministratorRights", Some(params))
            .await
    }

    async fn answer_web_app_query(
        &self,
        params: &AnswerWebAppQueryParams,
    ) -> Result<MethodResponse<SentWebAppMessage>, Self::Error> {
        self.request("answerWebAppQuery", Some(params)).await
    }

    async fn set_chat_menu_button(
        &self,
        params: SetChatMenuButtonParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setChatMenuButton", Some(params)).await
    }

    async fn get_chat_menu_button(
        &self,
        params: GetChatMenuButtonParams,
    ) -> Result<MethodResponse<MenuButton>, Self::Error> {
        self.request("getChatMenuButton", Some(params)).await
    }

    async fn request_without_body<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
    ) -> Result<T, Self::Error> {
        let params: Option<()> = None;

        self.request(method, params).await
    }

    async fn request<
        T1: serde::ser::Serialize + std::fmt::Debug + std::marker::Send,
        T2: serde::de::DeserializeOwned,
    >(
        &self,
        method: &str,
        params: Option<T1>,
    ) -> Result<T2, Self::Error>;

    async fn request_with_possible_form_data<
        T1: serde::ser::Serialize + std::fmt::Debug + std::marker::Send,
        T2: serde::de::DeserializeOwned,
    >(
        &self,
        method_name: &str,
        params: T1,
        files: Vec<(&str, PathBuf)>,
    ) -> Result<T2, Self::Error> {
        if files.is_empty() {
            self.request(method_name, Some(params)).await
        } else {
            self.request_with_form_data(method_name, params, files)
                .await
        }
    }

    async fn request_with_form_data<
        T1: serde::ser::Serialize + std::fmt::Debug + std::marker::Send,
        T2: serde::de::DeserializeOwned,
    >(
        &self,
        method: &str,
        params: T1,
        files: Vec<(&str, PathBuf)>,
    ) -> Result<T2, Self::Error>;
}
