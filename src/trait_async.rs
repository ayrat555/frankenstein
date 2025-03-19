use std::path::PathBuf;

use crate::games::GameHighScore;
use crate::inline_mode::{PreparedInlineMessage, SentWebAppMessage};
use crate::input_file::HasInputFile;
use crate::input_media::{InputMedia, MediaGroupInputMedia};
use crate::payments::StarTransactions;
use crate::response::{MessageOrBool, MethodResponse};
use crate::stickers::{Gifts, Sticker, StickerSet};
use crate::types::{
    BotCommand, BotDescription, BotName, BotShortDescription, BusinessConnection,
    ChatAdministratorRights, ChatFullInfo, ChatInviteLink, ChatMember, File, ForumTopic,
    MenuButton, Message, MessageId, Poll, User, UserChatBoosts, UserProfilePhotos,
};
use crate::updates::{Update, WebhookInfo};

macro_rules! request {
    ($name:ident, $return:ty) => {
        paste::paste! {
            #[allow(async_fn_in_trait)]
            #[doc = "Call the `" $name "` method.\n\nSee <https://core.telegram.org/bots/api#" $name:lower ">."]
            #[inline(always)]
            async fn [<$name:snake>] (
                &self,
                params: &crate::methods::[<$name:camel Params>],
            ) -> Result<MethodResponse<$return>, Self::Error> {
                self.request(stringify!($name), Some(params)).await
            }
        }
    }
}

/// request no body
macro_rules! request_nb {
    ($name:ident, $return:ty) => {
        paste::paste! {
            #[allow(async_fn_in_trait)]
            #[doc = "Call the `" $name "` method.\n\nSee <https://core.telegram.org/bots/api#" $name:lower ">."]
            #[inline(always)]
            async fn [<$name:snake>] (
                &self,
            ) -> Result<MethodResponse<$return>, Self::Error> {
                let params: Option<()> = None;
                self.request(stringify!($name), params).await
            }
        }
    }
}

/// request with some properties utilizing [`HasInputFile`]
macro_rules! request_f {
    ($name:ident, $return:ty, $($fileproperty:ident),+) => {
        paste::paste! {
            #[allow(async_fn_in_trait)]
            #[doc = "Call the `" $name "` method.\n\nSee <https://core.telegram.org/bots/api#" $name:lower ">."]
            async fn [<$name:snake>] (
                &self,
                params: &crate::methods::[<$name:camel Params>],
            ) -> Result<MethodResponse<$return>, Self::Error> {
                let mut files = Vec::new();
                $(
                    if let Some(path) = params.$fileproperty.clone_path() {
                        files.push((stringify!($fileproperty), path));
                    }
                )+
                self.request_with_possible_form_data(stringify!($name), params, files).await
            }
        }
    }
}

// Wasm target need not be `Send` because it is single-threaded
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub trait AsyncTelegramApi
where
    Self: Sync,
{
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
    request_f!(sendPhoto, Message, photo);
    request_f!(sendAudio, Message, audio, thumbnail);

    async fn send_media_group(
        &self,
        params: &crate::methods::SendMediaGroupParams,
    ) -> Result<MethodResponse<Vec<Message>>, Self::Error> {
        let mut files = Vec::new();

        macro_rules! replace_attach {
            ($base:ident. $property:ident) => {
                if let Some(file) = $base.$property.replace_attach_dyn(|| files.len()) {
                    files.push(file);
                }
            };
        }

        let mut params = params.clone();
        for media in &mut params.media {
            match media {
                MediaGroupInputMedia::Audio(audio) => {
                    replace_attach!(audio.media);
                    replace_attach!(audio.thumbnail);
                }
                MediaGroupInputMedia::Document(document) => {
                    replace_attach!(document.media);
                }
                MediaGroupInputMedia::Photo(photo) => {
                    replace_attach!(photo.media);
                }
                MediaGroupInputMedia::Video(video) => {
                    replace_attach!(video.media);
                    replace_attach!(video.cover);
                    replace_attach!(video.thumbnail);
                }
            }
        }

        let files_with_str_names = files
            .iter()
            .map(|(key, path)| (key.as_str(), path.clone()))
            .collect();

        self.request_with_possible_form_data("sendMediaGroup", &params, files_with_str_names)
            .await
    }

    request_f!(sendDocument, Message, document, thumbnail);
    request_f!(sendVideo, Message, video, cover, thumbnail);
    request_f!(sendAnimation, Message, animation, thumbnail);
    request_f!(sendVoice, Message, voice);
    request_f!(sendVideoNote, Message, video_note, thumbnail);
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
    request!(getFile, File);
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

    async fn set_chat_photo(
        &self,
        params: &crate::methods::SetChatPhotoParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let photo = &params.photo;
        self.request_with_form_data("setChatPhoto", params, vec![("photo", photo.path.clone())])
            .await
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

    async fn edit_message_media(
        &self,
        params: &crate::methods::EditMessageMediaParams,
    ) -> Result<MethodResponse<MessageOrBool>, Self::Error> {
        let mut files = Vec::new();

        macro_rules! replace_attach {
            ($base:ident. $property:ident) => {{
                const NAME: &str = concat!(stringify!($base), "_", stringify!($property));
                if let Some(file) = $base.$property.replace_attach(NAME) {
                    files.push((NAME, file));
                }
            }};
        }

        let mut params = params.clone();
        match &mut params.media {
            InputMedia::Animation(animation) => {
                replace_attach!(animation.media);
                replace_attach!(animation.thumbnail);
            }
            InputMedia::Document(document) => {
                replace_attach!(document.media);
                replace_attach!(document.thumbnail);
            }
            InputMedia::Audio(audio) => {
                replace_attach!(audio.media);
                replace_attach!(audio.thumbnail);
            }
            InputMedia::Photo(photo) => {
                replace_attach!(photo.media);
            }
            InputMedia::Video(video) => {
                replace_attach!(video.media);
                replace_attach!(video.cover);
                replace_attach!(video.thumbnail);
            }
        }

        self.request_with_possible_form_data("editMessageMedia", &params, files)
            .await
    }

    request!(editMessageReplyMarkup, MessageOrBool);
    request!(stopPoll, Poll);
    request!(deleteMessage, bool);
    request!(deleteMessages, bool);
    request_f!(sendSticker, Message, sticker);
    request!(getStickerSet, StickerSet);

    async fn upload_sticker_file(
        &self,
        params: &crate::methods::UploadStickerFileParams,
    ) -> Result<MethodResponse<File>, Self::Error> {
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
        params: &crate::methods::CreateNewStickerSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let mut files = Vec::new();

        let mut params = params.clone();
        for (index, sticker) in params.stickers.iter_mut().enumerate() {
            if let Some(file) = sticker.sticker.replace_attach_dyn(|| index) {
                files.push(file);
            }
        }

        let files_with_str_names = files
            .iter()
            .map(|(key, path)| (key.as_str(), path.clone()))
            .collect();

        self.request_with_possible_form_data("createNewStickerSet", &params, files_with_str_names)
            .await
    }

    request!(getCustomEmojiStickers, Vec<Sticker>);

    async fn add_sticker_to_set(
        &self,
        params: &crate::methods::AddStickerToSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let mut files = Vec::new();
        if let Some(file) = params.sticker.sticker.clone_path() {
            files.push(("sticker", file));
        }
        self.request_with_possible_form_data("addStickerToSet", params, files)
            .await
    }

    request!(setStickerPositionInSet, bool);
    request!(deleteStickerFromSet, bool);
    request!(replaceStickerInSet, bool);
    request!(setStickerEmojiList, bool);
    request!(setStickerKeywords, bool);
    request!(setStickerMaskPosition, bool);
    request!(setStickerSetTitle, bool);
    request_f!(setStickerSetThumbnail, bool, thumbnail);
    request!(setCustomEmojiStickerSetThumbnail, bool);
    request!(deleteStickerSet, bool);
    request_nb!(getAvailableGifts, Gifts);
    request!(sendGift, bool);
    request!(verifyUser, bool);
    request!(verifyChat, bool);
    request!(removeUserVerification, bool);
    request!(removeChatVerification, bool);
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
    request!(setPassportDataErrors, bool);

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

    async fn request<Params, Output>(
        &self,
        method: &str,
        params: Option<Params>,
    ) -> Result<Output, Self::Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug + std::marker::Send,
        Output: serde::de::DeserializeOwned;

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
