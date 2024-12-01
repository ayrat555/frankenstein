use std::path::PathBuf;

use crate::api_params::{
    AddStickerToSetParams, CreateNewStickerSetParams, EditMessageMediaParams, FileUpload,
    InputMedia, Media, SendAnimationParams, SendAudioParams, SendDocumentParams,
    SendMediaGroupParams, SendPhotoParams, SendStickerParams, SendVideoNoteParams, SendVideoParams,
    SendVoiceParams, SetChatPhotoParams, SetStickerSetThumbnailParams, UploadStickerFileParams,
};
use crate::objects::{
    BotCommand, BotDescription, BotName, BotShortDescription, BusinessConnection,
    ChatAdministratorRights, ChatFullInfo, ChatInviteLink, ChatMember, File as FileObject,
    ForumTopic, GameHighScore, InputSticker, MenuButton, Message, MessageId, Poll,
    PreparedInlineMessage, SentWebAppMessage, StarTransactions, Sticker, StickerSet, Update, User,
    UserChatBoosts, UserProfilePhotos, WebhookInfo,
};
use crate::response::{MessageOrBool, MethodResponse};

macro_rules! request {
    ($name:ident, $return:ty) => {
        paste::paste! {
            #[doc = "Call the `" $name "` method.\n\nSee <https://core.telegram.org/bots/api#" $name:lower ">."]
            #[inline(always)]
            fn [<$name:snake>] (
                &self,
                params: &crate::api_params::[<$name:camel Params>],
            ) -> Result<MethodResponse<$return>, Self::Error> {
                self.request(stringify!($name), Some(params))
            }
        }
    }
}

/// request no body
macro_rules! request_nb {
    ($name:ident, $return:ty) => {
        paste::paste! {
            #[doc = "Call the `" $name "` method.\n\nSee <https://core.telegram.org/bots/api#" $name:lower ">."]
            #[inline(always)]
            fn [<$name:snake>] (
                &self,
            ) -> Result<MethodResponse<$return>, Self::Error> {
                let params: Option<()> = None;
                self.request(stringify!($name), params)
            }
        }
    }
}

pub trait TelegramApi {
    type Error;

    request!(getUpdates, Vec<Update>);
    request!(sendMessage, Message);
    request!(setWebhook, bool);
    request!(deleteWebhook, bool);
    request_nb!(getWebhookInfo, WebhookInfo);
    request_nb!(getMe, User);
    request_nb!(logOut, bool);
    request_nb!(close, bool);
    request!(forwardMessage, Message);
    request!(forwardMessages, Vec<MessageId>);
    request!(copyMessage, MessageId);
    request!(copyMessages, Vec<MessageId>);

    fn send_photo(&self, params: &SendPhotoParams) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendPhoto";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let FileUpload::InputFile(input_file) = &params.photo {
            files.push(("photo", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    fn send_audio(&self, params: &SendAudioParams) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendAudio";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let FileUpload::InputFile(input_file) = &params.audio {
            files.push(("audio", input_file.path.clone()));
        }

        if let Some(FileUpload::InputFile(input_file)) = &params.thumbnail {
            files.push(("thumbnail", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    fn send_media_group(
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
    }

    fn send_document(
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
    }

    fn send_video(&self, params: &SendVideoParams) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendVideo";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let FileUpload::InputFile(input_file) = &params.video {
            files.push(("video", input_file.path.clone()));
        }

        if let Some(FileUpload::InputFile(input_file)) = &params.thumbnail {
            files.push(("thumbnail", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    fn send_animation(
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
    }

    fn send_voice(&self, params: &SendVoiceParams) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendVoice";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let FileUpload::InputFile(input_file) = &params.voice {
            files.push(("voice", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    fn send_video_note(
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
    }

    request!(sendPaidMedia, Message);
    request!(sendLocation, Message);
    request!(editMessageLiveLocation, MessageOrBool);
    request!(stopMessageLiveLocation, MessageOrBool);
    request!(sendVenue, Message);
    request!(sendContact, Message);
    request!(sendPoll, Message);
    request!(sendDice, Message);
    request!(sendChatAction, bool);
    request!(setMessageReaction, bool);
    request!(getUserProfilePhotos, UserProfilePhotos);
    request!(setUserEmojiStatus, bool);
    request!(getFile, FileObject);
    request!(banChatMember, bool);
    request!(unbanChatMember, bool);
    request!(restrictChatMember, bool);
    request!(promoteChatMember, bool);
    request!(setChatAdministratorCustomTitle, bool);
    request!(banChatSenderChat, bool);
    request!(unbanChatSenderChat, bool);
    request!(setChatPermissions, bool);
    request!(exportChatInviteLink, String);
    request!(createChatInviteLink, ChatInviteLink);
    request!(editChatInviteLink, ChatInviteLink);
    request!(createChatSubscriptionInviteLink, ChatInviteLink);
    request!(editChatSubscriptionInviteLink, ChatInviteLink);
    request!(revokeChatInviteLink, ChatInviteLink);
    request!(approveChatJoinRequest, bool);
    request!(declineChatJoinRequest, bool);

    fn set_chat_photo(
        &self,
        params: &SetChatPhotoParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let photo = &params.photo;

        self.request_with_form_data("setChatPhoto", params, vec![("photo", photo.path.clone())])
    }

    request!(deleteChatPhoto, bool);
    request!(setChatTitle, bool);
    request!(setChatDescription, bool);
    request!(pinChatMessage, bool);
    request!(unpinChatMessage, bool);
    request!(unpinAllChatMessages, bool);
    request!(leaveChat, bool);
    request!(getChat, ChatFullInfo);
    request!(getChatAdministrators, Vec<ChatMember>);
    request!(getChatMemberCount, u32);
    request!(getChatMember, ChatMember);
    request!(setChatStickerSet, bool);
    request!(deleteChatStickerSet, bool);
    request_nb!(getForumTopicIconStickers, Vec<Sticker>);
    request!(createForumTopic, ForumTopic);
    request!(editForumTopic, bool);
    request!(closeForumTopic, bool);
    request!(reopenForumTopic, bool);
    request!(deleteForumTopic, bool);
    request!(unpinAllForumTopicMessages, bool);
    request!(editGeneralForumTopic, bool);
    request!(closeGeneralForumTopic, bool);
    request!(reopenGeneralForumTopic, bool);
    request!(hideGeneralForumTopic, bool);
    request!(unhideGeneralForumTopic, bool);
    request!(answerCallbackQuery, bool);
    request!(getUserChatBoosts, UserChatBoosts);
    request!(getBusinessConnection, BusinessConnection);
    request!(getMyCommands, Vec<BotCommand>);
    request!(setMyCommands, bool);
    request!(deleteMyCommands, bool);
    request!(setMyName, bool);
    request!(getMyName, BotName);
    request!(setMyDescription, bool);
    request!(getMyDescription, BotDescription);
    request!(setMyShortDescription, bool);
    request!(getMyShortDescription, BotShortDescription);
    request!(answerInlineQuery, bool);
    request!(editMessageText, MessageOrBool);
    request!(editMessageCaption, MessageOrBool);

    fn edit_message_media(
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
    }

    request!(editMessageReplyMarkup, MessageOrBool);
    request!(stopPoll, Poll);
    request!(deleteMessage, bool);
    request!(deleteMessages, bool);

    fn send_sticker(
        &self,
        params: &SendStickerParams,
    ) -> Result<MethodResponse<Message>, Self::Error> {
        let method_name = "sendSticker";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let FileUpload::InputFile(input_file) = &params.sticker {
            files.push(("sticker", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    request!(getStickerSet, StickerSet);

    fn upload_sticker_file(
        &self,
        params: &UploadStickerFileParams,
    ) -> Result<MethodResponse<FileObject>, Self::Error> {
        let sticker = &params.sticker;

        self.request_with_form_data(
            "uploadStickerFile",
            params,
            vec![("sticker", sticker.path.clone())],
        )
    }

    fn create_new_sticker_set(
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
    }

    request!(getCustomEmojiStickers, Vec<Sticker>);

    fn add_sticker_to_set(
        &self,
        params: &AddStickerToSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let method_name = "addStickerToSet";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let FileUpload::InputFile(input_file) = &params.sticker.sticker {
            files.push(("sticker", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    request!(setStickerPositionInSet, bool);
    request!(deleteStickerFromSet, bool);
    request!(replaceStickerInSet, bool);
    request!(setStickerEmojiList, bool);
    request!(setStickerKeywords, bool);
    request!(setStickerMaskPosition, bool);
    request!(setStickerSetTitle, bool);

    fn set_sticker_set_thumbnail(
        &self,
        params: &SetStickerSetThumbnailParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let method_name = "setStickerSetThumbnail";
        let mut files: Vec<(&str, PathBuf)> = vec![];

        if let Some(FileUpload::InputFile(input_file)) = &params.thumbnail {
            files.push(("thumbnail", input_file.path.clone()));
        }

        self.request_with_possible_form_data(method_name, params, files)
    }

    request!(setCustomEmojiStickerSetThumbnail, bool);
    request!(deleteStickerSet, bool);
    request!(sendInvoice, Message);
    request!(createInvoiceLink, String);
    request!(answerShippingQuery, bool);
    request!(answerPreCheckoutQuery, bool);
    request!(getStarTransactions, StarTransactions);
    request!(refundStarPayment, bool);
    request!(editUserStarSubscription, bool);
    request!(sendGame, Message);
    request!(setGameScore, MessageOrBool);
    request!(getGameHighScores, Vec<GameHighScore>);
    request!(setMyDefaultAdministratorRights, bool);
    request!(getMyDefaultAdministratorRights, ChatAdministratorRights);
    request!(answerWebAppQuery, SentWebAppMessage);
    request!(savePreparedInlineMessage, PreparedInlineMessage);
    request!(setChatMenuButton, bool);
    request!(getChatMenuButton, MenuButton);
    request!(unpinAllGeneralForumTopicMessages, bool);

    fn request_with_possible_form_data<Params, Output>(
        &self,
        method_name: &str,
        params: &Params,
        files: Vec<(&str, PathBuf)>,
    ) -> Result<Output, Self::Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug,
        Output: serde::de::DeserializeOwned,
    {
        if files.is_empty() {
            self.request(method_name, Some(params))
        } else {
            self.request_with_form_data(method_name, params, files)
        }
    }

    fn request_with_form_data<Params, Output>(
        &self,
        method: &str,
        params: Params,
        files: Vec<(&str, PathBuf)>,
    ) -> Result<Output, Self::Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug,
        Output: serde::de::DeserializeOwned;

    fn request<Params, Output>(
        &self,
        method: &str,
        params: Option<Params>,
    ) -> Result<Output, Self::Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug,
        Output: serde::de::DeserializeOwned;
}
