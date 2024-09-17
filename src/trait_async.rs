use std::path::PathBuf;

use async_trait::async_trait;

use crate::api_params::{
    AddStickerToSetParams, AnswerCallbackQueryParams, AnswerInlineQueryParams,
    AnswerPreCheckoutQueryParams, AnswerShippingQueryParams, AnswerWebAppQueryParams,
    ApproveChatJoinRequestParams, BanChatMemberParams, BanChatSenderChatParams,
    CloseForumTopicParams, CloseGeneralForumTopicParams, CopyMessageParams, CopyMessagesParams,
    CreateChatInviteLinkParams, CreateChatSubscriptionInviteLinkParams, CreateForumTopicParams,
    CreateInvoiceLinkParams, CreateNewStickerSetParams, DeclineChatJoinRequestParams,
    DeleteChatPhotoParams, DeleteChatStickerSetParams, DeleteForumTopicParams, DeleteMessageParams,
    DeleteMessagesParams, DeleteMyCommandsParams, DeleteStickerFromSetParams,
    DeleteStickerSetParams, DeleteWebhookParams, EditChatInviteLinkParams,
    EditChatSubscriptionInviteLinkParams, EditForumTopicParams, EditGeneralForumTopicParams,
    EditMessageCaptionParams, EditMessageLiveLocationParams, EditMessageMediaParams,
    EditMessageReplyMarkupParams, EditMessageTextParams, ExportChatInviteLinkParams, FileUpload,
    ForwardMessageParams, ForwardMessagesParams, GetBusinessConnectionParams,
    GetChatAdministratorsParams, GetChatMemberCountParams, GetChatMemberParams,
    GetChatMenuButtonParams, GetChatParams, GetCustomEmojiStickersParams, GetFileParams,
    GetGameHighScoresParams, GetMyCommandsParams, GetMyDefaultAdministratorRightsParams,
    GetMyDescriptionParams, GetMyNameParams, GetMyShortDescriptionParams,
    GetStarTransactionsParams, GetStickerSetParams, GetUpdatesParams, GetUserChatBoostsParams,
    GetUserProfilePhotosParams, HideGeneralForumTopicParams, InputMedia, LeaveChatParams, Media,
    PinChatMessageParams, PromoteChatMemberParams, RefundStarPaymentParams, ReopenForumTopicParams,
    ReopenGeneralForumTopicParams, ReplaceStickerInSetParams, RestrictChatMemberParams,
    RevokeChatInviteLinkParams, SendAnimationParams, SendAudioParams, SendChatActionParams,
    SendContactParams, SendDiceParams, SendDocumentParams, SendGameParams, SendInvoiceParams,
    SendLocationParams, SendMediaGroupParams, SendMessageParams, SendPaidMediaParams,
    SendPhotoParams, SendPollParams, SendStickerParams, SendVenueParams, SendVideoNoteParams,
    SendVideoParams, SendVoiceParams, SetChatAdministratorCustomTitleParams,
    SetChatDescriptionParams, SetChatMenuButtonParams, SetChatPermissionsParams,
    SetChatPhotoParams, SetChatStickerSetParams, SetChatTitleParams,
    SetCustomEmojiStickerSetThumbnailParams, SetGameScoreParams, SetMessageReactionParams,
    SetMyCommandsParams, SetMyDefaultAdministratorRightsParams, SetMyDescriptionParams,
    SetMyNameParams, SetMyShortDescriptionParams, SetStickerEmojiListParams,
    SetStickerKeywordsParams, SetStickerMaskPositionParams, SetStickerPositionInSetParams,
    SetStickerSetThumbnailParams, SetStickerSetTitleParams, SetWebhookParams,
    StopMessageLiveLocationParams, StopPollParams, UnbanChatMemberParams,
    UnbanChatSenderChatParams, UnhideGeneralForumTopicParams, UnpinAllChatMessagesParams,
    UnpinAllForumTopicMessagesParams, UnpinAllGeneralForumTopicMessagesParams,
    UnpinChatMessageParams, UploadStickerFileParams,
};
use crate::objects::{
    BotCommand, BotDescription, BotName, BotShortDescription, BusinessConnection,
    ChatAdministratorRights, ChatFullInfo, ChatInviteLink, ChatMember, File as FileObject,
    ForumTopic, GameHighScore, InputSticker, MenuButton, Message, MessageId, Poll,
    SentWebAppMessage, StarTransactions, Sticker, StickerSet, Update, User, UserChatBoosts,
    UserProfilePhotos, WebhookInfo,
};
use crate::response::{MessageOrBool, MethodResponse};

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

    async fn forward_messages(
        &self,
        params: &ForwardMessagesParams,
    ) -> Result<MethodResponse<Vec<MessageId>>, Self::Error> {
        self.request("forwardMessages", Some(params)).await
    }

    async fn copy_message(
        &self,
        params: &CopyMessageParams,
    ) -> Result<MethodResponse<MessageId>, Self::Error> {
        self.request("copyMessage", Some(params)).await
    }

    async fn copy_messages(
        &self,
        params: &CopyMessagesParams,
    ) -> Result<MethodResponse<Vec<MessageId>>, Self::Error> {
        self.request("copyMessages", Some(params)).await
    }

    async fn send_photo(
        &self,
        params: &SendPhotoParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendPhoto";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let FileUpload::InputFile(input_file) = &params.photo {
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

        if let FileUpload::InputFile(input_file) = &params.audio {
            files.push(("audio", input_file.path.clone()));
        }

        if let Some(FileUpload::InputFile(input_file)) = &params.thumbnail {
            files.push(("thumbnail", input_file.path.clone()));
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

                    if let FileUpload::InputFile(input_file) = &audio.media {
                        let name = format!("file{file_idx}");
                        let attach_name = format!("attach://{name}");
                        file_idx += 1;

                        new_audio.media = FileUpload::String(attach_name);

                        files.push((name, input_file.path.clone()));
                    };

                    if let Some(FileUpload::InputFile(input_file)) = &audio.thumbnail {
                        let name = format!("file{file_idx}");
                        let attach_name = format!("attach://{name}");
                        file_idx += 1;

                        new_audio.thumbnail = Some(FileUpload::String(attach_name));

                        files.push((name, input_file.path.clone()));
                    };

                    new_medias.push(Media::Audio(new_audio));
                }

                Media::Document(document) => {
                    let mut new_document = document.clone();

                    if let FileUpload::InputFile(input_file) = &document.media {
                        let name = format!("file{file_idx}");
                        let attach_name = format!("attach://{name}");
                        file_idx += 1;

                        new_document.media = FileUpload::String(attach_name);

                        files.push((name, input_file.path.clone()));
                    };

                    new_medias.push(Media::Document(new_document));
                }
                Media::Photo(photo) => {
                    let mut new_photo = photo.clone();

                    if let FileUpload::InputFile(input_file) = &photo.media {
                        let name = format!("file{file_idx}");
                        let attach_name = format!("attach://{name}");
                        file_idx += 1;

                        new_photo.media = FileUpload::String(attach_name);

                        files.push((name, input_file.path.clone()));
                    };

                    new_medias.push(Media::Photo(new_photo));
                }

                Media::Video(video) => {
                    let mut new_video = video.clone();

                    if let FileUpload::InputFile(input_file) = &video.media {
                        let name = format!("file{file_idx}");
                        let attach_name = format!("attach://{name}");
                        file_idx += 1;

                        new_video.media = FileUpload::String(attach_name);

                        files.push((name, input_file.path.clone()));
                    };

                    if let Some(FileUpload::InputFile(input_file)) = &video.thumbnail {
                        let name = format!("file{file_idx}");
                        let attach_name = format!("attach://{name}");
                        file_idx += 1;

                        new_video.thumbnail = Some(FileUpload::String(attach_name));

                        files.push((name, input_file.path.clone()));
                    };

                    new_medias.push(Media::Video(new_video));
                }
            };
        }

        let mut new_params = params.clone();
        new_params.media = new_medias;

        let files_with_str_names = files
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

        if let FileUpload::InputFile(input_file) = &params.document {
            files.push(("document", input_file.path.clone()));
        }

        if let Some(FileUpload::InputFile(input_file)) = &params.thumbnail {
            files.push(("thumbnail", input_file.path.clone()));
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

        if let FileUpload::InputFile(input_file) = &params.video {
            files.push(("video", input_file.path.clone()));
        }

        if let Some(FileUpload::InputFile(input_file)) = &params.thumbnail {
            files.push(("thumbnail", input_file.path.clone()));
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

        if let FileUpload::InputFile(input_file) = &params.animation {
            files.push(("animation", input_file.path.clone()));
        }

        if let Some(FileUpload::InputFile(input_file)) = &params.thumbnail {
            files.push(("thumbnail", input_file.path.clone()));
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

        if let FileUpload::InputFile(input_file) = &params.voice {
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

        if let FileUpload::InputFile(input_file) = &params.video_note {
            files.push(("video_note", input_file.path.clone()));
        }

        if let Some(FileUpload::InputFile(input_file)) = &params.thumbnail {
            files.push(("thumbnail", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
            .await
    }

    async fn send_paid_media(
        &self,
        params: &SendPaidMediaParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        self.request("sendPaidMedia", Some(params)).await
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
    ) -> Result<MethodResponse<MessageOrBool>, Self::Error> {
        self.request("editMessageLiveLocation", Some(params)).await
    }

    async fn stop_message_live_location(
        &self,
        params: &StopMessageLiveLocationParams,
    ) -> Result<MethodResponse<MessageOrBool>, Self::Error> {
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

    async fn set_message_reaction(
        &self,
        params: &SetMessageReactionParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setMessageReaction", Some(params)).await
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

    async fn create_chat_subscription_invite_link(
        &self,
        params: &CreateChatSubscriptionInviteLinkParams,
    ) -> Result<MethodResponse<ChatInviteLink>, Self::Error> {
        self.request("createChatSubscriptionInviteLink", Some(params))
            .await
    }

    async fn edit_chat_subscription_invite_link(
        &self,
        params: &EditChatSubscriptionInviteLinkParams,
    ) -> Result<MethodResponse<ChatInviteLink>, Self::Error> {
        self.request("editChatSubscriptionInviteLink", Some(params))
            .await
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

    async fn unpin_all_chat_messages(
        &self,
        params: &UnpinAllChatMessagesParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("unpinAllChatMessages", Some(params)).await
    }

    async fn leave_chat(
        &self,
        params: &LeaveChatParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("leaveChat", Some(params)).await
    }

    async fn get_chat(
        &self,
        params: &GetChatParams,
    ) -> Result<MethodResponse<ChatFullInfo>, Self::Error> {
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

    async fn get_forum_topic_icon_stickers(
        &self,
    ) -> Result<MethodResponse<Vec<Sticker>>, Self::Error> {
        self.request_without_body("getForumTopicIconStickers").await
    }

    async fn create_forum_topic(
        &self,
        params: &CreateForumTopicParams,
    ) -> Result<MethodResponse<ForumTopic>, Self::Error> {
        self.request("createForumTopic", Some(params)).await
    }

    async fn edit_forum_topic(
        &self,
        params: &EditForumTopicParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("editForumTopic", Some(params)).await
    }

    async fn close_forum_topic(
        &self,
        params: &CloseForumTopicParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("closeForumTopic", Some(params)).await
    }

    async fn reopen_forum_topic(
        &self,
        params: &ReopenForumTopicParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("reopenForumTopic", Some(params)).await
    }

    async fn delete_forum_topic(
        &self,
        params: &DeleteForumTopicParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("deleteForumTopic", Some(params)).await
    }

    async fn unpin_all_forum_topic_messages(
        &self,
        params: &UnpinAllForumTopicMessagesParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("unpinAllForumTopicMessages", Some(params))
            .await
    }

    async fn edit_general_forum_topic(
        &self,
        params: &EditGeneralForumTopicParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("editGeneralForumTopic", Some(params)).await
    }

    async fn close_general_forum_topic(
        &self,
        params: &CloseGeneralForumTopicParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("closeGeneralForumTopic", Some(params)).await
    }

    async fn reopen_general_forum_topic(
        &self,
        params: &ReopenGeneralForumTopicParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("reopenGeneralForumTopic", Some(params)).await
    }

    async fn hide_general_forum_topic(
        &self,
        params: &HideGeneralForumTopicParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("hideGeneralForumTopic", Some(params)).await
    }

    async fn unhide_general_forum_topic(
        &self,
        params: &UnhideGeneralForumTopicParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("unhideGeneralForumTopic", Some(params)).await
    }

    async fn answer_callback_query(
        &self,
        params: &AnswerCallbackQueryParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("answerCallbackQuery", Some(params)).await
    }

    async fn get_user_chat_boosts(
        &self,
        params: &GetUserChatBoostsParams,
    ) -> Result<MethodResponse<UserChatBoosts>, Self::Error> {
        self.request("getUserChatBoosts", Some(params)).await
    }

    async fn get_business_connection(
        &self,
        params: &GetBusinessConnectionParams,
    ) -> Result<MethodResponse<BusinessConnection>, Self::Error> {
        self.request("getBusinessConnection", Some(params)).await
    }

    async fn get_my_commands(
        &self,
        params: &GetMyCommandsParams,
    ) -> Result<MethodResponse<Vec<BotCommand>>, Self::Error> {
        self.request("getMyCommands", Some(params)).await
    }

    async fn set_my_commands(
        &self,
        params: &SetMyCommandsParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setMyCommands", Some(params)).await
    }

    async fn delete_my_commands(
        &self,
        params: &DeleteMyCommandsParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("deleteMyCommands", Some(params)).await
    }

    async fn set_my_name(
        &self,
        params: &SetMyNameParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setMyName", Some(params)).await
    }

    async fn get_my_name(
        &self,
        params: &GetMyNameParams,
    ) -> Result<MethodResponse<BotName>, Self::Error> {
        self.request("getMyName", Some(params)).await
    }

    async fn set_my_description(
        &self,
        params: &SetMyDescriptionParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setMyDescription", Some(params)).await
    }

    async fn get_my_description(
        &self,
        params: &GetMyDescriptionParams,
    ) -> Result<MethodResponse<BotDescription>, Self::Error> {
        self.request("getMyDescription", Some(params)).await
    }

    async fn set_my_short_description(
        &self,
        params: &SetMyShortDescriptionParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setMyShortDescription", Some(params)).await
    }

    async fn get_my_short_description(
        &self,
        params: &GetMyShortDescriptionParams,
    ) -> Result<MethodResponse<BotShortDescription>, Self::Error> {
        self.request("getMyShortDescription", Some(params)).await
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
    ) -> Result<MethodResponse<MessageOrBool>, Self::Error> {
        self.request("editMessageText", Some(params)).await
    }

    async fn edit_message_caption(
        &self,
        params: &EditMessageCaptionParams,
    ) -> Result<MethodResponse<MessageOrBool>, Self::Error> {
        self.request("editMessageCaption", Some(params)).await
    }

    async fn edit_message_media(
        &self,
        params: &EditMessageMediaParams,
    ) -> Result<MethodResponse<MessageOrBool>, Self::Error> {
        let method_name = "editMessageMedia";
        let mut files: Vec<(String, PathBuf)> = vec![];

        let new_media = match &params.media {
            InputMedia::Animation(animation) => {
                let mut new_animation = animation.clone();

                if let FileUpload::InputFile(input_file) = &animation.media {
                    let name = "animation".to_string();
                    let attach_name = format!("attach://{name}");

                    new_animation.media = FileUpload::String(attach_name);

                    files.push((name, input_file.path.clone()));
                };

                if let Some(FileUpload::InputFile(input_file)) = &animation.thumbnail {
                    let name = "animation_thumb".to_string();
                    let attach_name = format!("attach://{name}");

                    new_animation.thumbnail = Some(FileUpload::String(attach_name));

                    files.push((name, input_file.path.clone()));
                };

                InputMedia::Animation(new_animation)
            }
            InputMedia::Document(document) => {
                let mut new_document = document.clone();

                if let FileUpload::InputFile(input_file) = &document.media {
                    let name = "document".to_string();
                    let attach_name = format!("attach://{name}");

                    new_document.media = FileUpload::String(attach_name);

                    files.push((name, input_file.path.clone()));
                };

                if let Some(FileUpload::InputFile(input_file)) = &document.thumbnail {
                    let name = "document_thumb".to_string();
                    let attach_name = format!("attach://{name}");

                    new_document.thumbnail = Some(FileUpload::String(attach_name));

                    files.push((name, input_file.path.clone()));
                };

                InputMedia::Document(new_document)
            }
            InputMedia::Audio(audio) => {
                let mut new_audio = audio.clone();

                if let FileUpload::InputFile(input_file) = &audio.media {
                    let name = "audio".to_string();
                    let attach_name = format!("attach://{name}");

                    new_audio.media = FileUpload::String(attach_name);

                    files.push((name, input_file.path.clone()));
                };

                if let Some(FileUpload::InputFile(input_file)) = &audio.thumbnail {
                    let name = "audio_thumb".to_string();
                    let attach_name = format!("attach://{name}");

                    new_audio.thumbnail = Some(FileUpload::String(attach_name));

                    files.push((name, input_file.path.clone()));
                };

                InputMedia::Audio(new_audio)
            }
            InputMedia::Photo(photo) => {
                let mut new_photo = photo.clone();

                if let FileUpload::InputFile(input_file) = &photo.media {
                    let name = "photo".to_string();
                    let attach_name = format!("attach://{name}");

                    new_photo.media = FileUpload::String(attach_name);

                    files.push((name, input_file.path.clone()));
                };

                InputMedia::Photo(new_photo)
            }
            InputMedia::Video(video) => {
                let mut new_video = video.clone();

                if let FileUpload::InputFile(input_file) = &video.media {
                    let name = "video".to_string();
                    let attach_name = format!("attach://{name}");

                    new_video.media = FileUpload::String(attach_name);

                    files.push((name, input_file.path.clone()));
                };

                if let Some(FileUpload::InputFile(input_file)) = &video.thumbnail {
                    let name = "video_thumb".to_string();
                    let attach_name = format!("attach://{name}");

                    new_video.thumbnail = Some(FileUpload::String(attach_name));

                    files.push((name, input_file.path.clone()));
                };

                InputMedia::Video(new_video)
            }
        };

        let mut new_params = params.clone();
        new_params.media = new_media;

        let files_with_str_names = files
            .iter()
            .map(|(key, path)| (key.as_str(), path.clone()))
            .collect();

        self.request_with_possible_form_data(method_name, &new_params, files_with_str_names)
            .await
    }

    async fn edit_message_reply_markup(
        &self,
        params: &EditMessageReplyMarkupParams,
    ) -> Result<MethodResponse<MessageOrBool>, Self::Error> {
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

    async fn delete_messages(
        &self,
        params: &DeleteMessagesParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("deleteMessages", Some(params)).await
    }

    async fn send_sticker(
        &self,
        params: &SendStickerParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendSticker";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let FileUpload::InputFile(input_file) = &params.sticker {
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
    ) -> Result<MethodResponse<FileObject>, Self::Error> {
        let sticker = &params.sticker;

        self.request_with_form_data(
            "uploadStickerFile",
            params,
            vec![("sticker", sticker.path.clone())],
        )
        .await
    }

    async fn create_new_sticker_set(
        &self,
        params: &CreateNewStickerSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let method_name = "createNewStickerSet";
        let mut new_stickers: Vec<InputSticker> = vec![];
        let mut files: Vec<(String, PathBuf)> = vec![];
        let mut file_idx = 0;

        for sticker in &params.stickers {
            let mut new_sticker = sticker.clone();

            if let FileUpload::InputFile(input_file) = &sticker.sticker {
                let name = format!("file{file_idx}");
                let attach_name = format!("attach://{name}");
                file_idx += 1;

                new_sticker.sticker = FileUpload::String(attach_name);

                files.push((name, input_file.path.clone()));
            };

            new_stickers.push(new_sticker);
        }

        let mut new_params = params.clone();
        new_params.stickers = new_stickers;

        let files_with_str_names = files
            .iter()
            .map(|(key, path)| (key.as_str(), path.clone()))
            .collect();

        self.request_with_possible_form_data(method_name, &new_params, files_with_str_names)
            .await
    }

    async fn get_custom_emoji_stickers(
        &self,
        params: &GetCustomEmojiStickersParams,
    ) -> Result<MethodResponse<Vec<Sticker>>, Self::Error> {
        self.request("getCustomEmojiStickers", Some(params)).await
    }
    async fn add_sticker_to_set(
        &self,
        params: &AddStickerToSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let method_name = "addStickerToSet";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let FileUpload::InputFile(input_file) = &params.sticker.sticker {
            files.push(("sticker", input_file.path.clone()));
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

    async fn replace_sticker_in_set(
        &self,
        params: &ReplaceStickerInSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("replaceStickerInSet", Some(params)).await
    }

    async fn delete_sticker_from_set(
        &self,
        params: &DeleteStickerFromSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("deleteStickerFromSet", Some(params)).await
    }

    async fn set_sticker_emoji_list(
        &self,
        params: &SetStickerEmojiListParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setStickerEmojiList", Some(params)).await
    }

    async fn set_sticker_keywords(
        &self,
        params: &SetStickerKeywordsParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setStickerKeywords", Some(params)).await
    }

    async fn set_sticker_mask_position(
        &self,
        params: &SetStickerMaskPositionParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setStickerMaskPosition", Some(params)).await
    }

    async fn set_sticker_set_title(
        &self,
        params: &SetStickerSetTitleParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setStickerSetTitle", Some(params)).await
    }

    async fn set_sticker_set_thumb(
        &self,
        params: &SetStickerSetThumbnailParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let method_name = "setStickerSetThumbnail";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let Some(FileUpload::InputFile(input_file)) = &params.thumbnail {
            files.push(("thumbnail", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
            .await
    }

    async fn set_custom_emoji_sticker_set_thumbnail(
        &self,
        params: &SetCustomEmojiStickerSetThumbnailParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setCustomEmojiStickerSetThumbnail", Some(params))
            .await
    }

    async fn delete_sticker_set(
        &self,
        params: &DeleteStickerSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("deleteStickerSet", Some(params)).await
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

    async fn get_star_transactions(
        &self,
        params: &GetStarTransactionsParams,
    ) -> Result<MethodResponse<StarTransactions>, Self::Error> {
        self.request("getStarTransactions", Some(params)).await
    }

    async fn refund_star_payment(
        &self,
        params: &RefundStarPaymentParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("refundStarPayment", Some(params)).await
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
    ) -> Result<MethodResponse<MessageOrBool>, Self::Error> {
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
        params: &SetChatMenuButtonParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("setChatMenuButton", Some(params)).await
    }

    async fn get_chat_menu_button(
        &self,
        params: &GetChatMenuButtonParams,
    ) -> Result<MethodResponse<MenuButton>, Self::Error> {
        self.request("getChatMenuButton", Some(params)).await
    }

    async fn unpin_all_general_forum_topic_messages(
        &self,
        params: &UnpinAllGeneralForumTopicMessagesParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        self.request("unpinAllGeneralForumTopicMessages", Some(params))
            .await
    }

    async fn request_without_body<Output>(&self, method: &str) -> Result<Output, Self::Error>
    where
        Output: serde::de::DeserializeOwned,
    {
        let params: Option<()> = None;
        self.request(method, params).await
    }

    async fn request<Params, Output>(
        &self,
        method: &str,
        params: Option<Params>,
    ) -> Result<Output, Self::Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug + std::marker::Send,
        Output: serde::de::DeserializeOwned;

    async fn request_with_possible_form_data<Params, Output>(
        &self,
        method_name: &str,
        params: Params,
        files: Vec<(&str, PathBuf)>,
    ) -> Result<Output, Self::Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug + std::marker::Send,
        Output: serde::de::DeserializeOwned,
    {
        if files.is_empty() {
            self.request(method_name, Some(params)).await
        } else {
            self.request_with_form_data(method_name, params, files)
                .await
        }
    }

    async fn request_with_form_data<Params, Output>(
        &self,
        method: &str,
        params: Params,
        files: Vec<(&str, PathBuf)>,
    ) -> Result<Output, Self::Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug + std::marker::Send,
        Output: serde::de::DeserializeOwned;
}
