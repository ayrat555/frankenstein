//! Parameters to Telegram API methods.

use std::path::PathBuf;

use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::objects::{
    AllowedUpdate, BotCommand, ChatAdministratorRights, ChatPermissions, ForceReply,
    InlineKeyboardMarkup, InlineQueryResultArticle, InlineQueryResultAudio,
    InlineQueryResultCachedAudio, InlineQueryResultCachedDocument, InlineQueryResultCachedGif,
    InlineQueryResultCachedMpeg4Gif, InlineQueryResultCachedPhoto, InlineQueryResultCachedSticker,
    InlineQueryResultCachedVideo, InlineQueryResultCachedVoice, InlineQueryResultContact,
    InlineQueryResultDocument, InlineQueryResultGame, InlineQueryResultGif,
    InlineQueryResultLocation, InlineQueryResultMpeg4Gif, InlineQueryResultPhoto,
    InlineQueryResultVenue, InlineQueryResultVideo, InlineQueryResultVoice, InputPaidMedia,
    InputPollOption, InputSticker, LabeledPrice, LinkPreviewOptions, MaskPosition, MenuButton,
    MessageEntity, PassportElementErrorDataField, PassportElementErrorFile,
    PassportElementErrorFiles, PassportElementErrorFrontSide, PassportElementErrorReverseSide,
    PassportElementErrorSelfie, PassportElementErrorTranslationFile,
    PassportElementErrorTranslationFiles, PassportElementErrorUnspecified, PollType, ReactionType,
    ReplyKeyboardMarkup, ReplyKeyboardRemove, ShippingOption, StickerFormat, StickerType,
    WebAppInfo,
};
use crate::ParseMode;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum FileUpload {
    InputFile(InputFile),
    String(String),
}

impl From<PathBuf> for FileUpload {
    fn from(path: PathBuf) -> Self {
        let input_file = InputFile { path };

        Self::InputFile(input_file)
    }
}

impl From<InputFile> for FileUpload {
    fn from(file: InputFile) -> Self {
        Self::InputFile(file)
    }
}

impl From<String> for FileUpload {
    fn from(file: String) -> Self {
        Self::String(file)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum InlineQueryResult {
    #[serde(rename = "audio")]
    CachedAudio(InlineQueryResultCachedAudio),
    #[serde(rename = "document")]
    CachedDocument(InlineQueryResultCachedDocument),
    #[serde(rename = "gif")]
    CachedGif(InlineQueryResultCachedGif),
    #[serde(rename = "mpeg4_gif")]
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    #[serde(rename = "photo")]
    CachedPhoto(InlineQueryResultCachedPhoto),
    #[serde(rename = "sticker")]
    CachedSticker(InlineQueryResultCachedSticker),
    #[serde(rename = "video")]
    CachedVideo(InlineQueryResultCachedVideo),
    #[serde(rename = "voice")]
    CachedVoice(InlineQueryResultCachedVoice),
    #[serde(rename = "article")]
    Article(InlineQueryResultArticle),
    #[serde(rename = "audio")]
    Audio(InlineQueryResultAudio),
    #[serde(rename = "contract")]
    Contact(InlineQueryResultContact),
    #[serde(rename = "game")]
    Game(InlineQueryResultGame),
    #[serde(rename = "document")]
    Document(InlineQueryResultDocument),
    #[serde(rename = "gif")]
    Gif(InlineQueryResultGif),
    #[serde(rename = "location")]
    Location(InlineQueryResultLocation),
    #[serde(rename = "mpeg4_gif")]
    Mpeg4Gif(InlineQueryResultMpeg4Gif),
    #[serde(rename = "photo")]
    Photo(InlineQueryResultPhoto),
    #[serde(rename = "venue")]
    Venue(InlineQueryResultVenue),
    #[serde(rename = "video")]
    Video(InlineQueryResultVideo),
    #[serde(rename = "voice")]
    Voice(InlineQueryResultVoice),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum InputMedia {
    #[serde(rename = "animation")]
    Animation(InputMediaAnimation),
    #[serde(rename = "document")]
    Document(InputMediaDocument),
    #[serde(rename = "audio")]
    Audio(InputMediaAudio),
    #[serde(rename = "photo")]
    Photo(InputMediaPhoto),
    #[serde(rename = "video")]
    Video(InputMediaVideo),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "source")]
pub enum PassportElementError {
    #[serde(rename = "data")]
    DataField(PassportElementErrorDataField),
    #[serde(rename = "front_side")]
    FrontSide(PassportElementErrorFrontSide),
    #[serde(rename = "reverse_side")]
    ReverseSide(PassportElementErrorReverseSide),
    #[serde(rename = "selfie")]
    Selfie(PassportElementErrorSelfie),
    #[serde(rename = "file")]
    File(PassportElementErrorFile),
    #[serde(rename = "files")]
    Files(PassportElementErrorFiles),
    #[serde(rename = "translation_file")]
    TranslationFile(PassportElementErrorTranslationFile),
    #[serde(rename = "translation_files")]
    TranslationFiles(PassportElementErrorTranslationFiles),
    #[serde(rename = "unspecified")]
    Unspecified(PassportElementErrorUnspecified),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ChatId {
    Integer(i64),
    String(String),
}

impl From<i64> for ChatId {
    fn from(id: i64) -> Self {
        Self::Integer(id)
    }
}

impl From<String> for ChatId {
    fn from(id: String) -> Self {
        Self::String(id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChatAction {
    Typing,
    UploadPhoto,
    RecordVideo,
    UploadVideo,
    RecordVoice,
    UploadVoice,
    UploadDocument,
    ChooseSticker,
    FindLocation,
    RecordVideoNote,
    UploadVideoNote,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Media {
    Audio(InputMediaAudio),
    Document(InputMediaDocument),
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BotCommandScope {
    Default,
    AllPrivateChats,
    AllGroupChats,
    AllChatAdministrators,
    Chat(BotCommandScopeChat),
    ChatAdministrators(BotCommandScopeChatAdministrators),
    ChatMember(BotCommandScopeChatMember),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BotCommandScopeChat {
    #[builder(into)]
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BotCommandScopeChatAdministrators {
    #[builder(into)]
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BotCommandScopeChatMember {
    #[builder(into)]
    pub chat_id: ChatId,
    pub user_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InputFile {
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GetUpdatesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetWebhookParams {
    #[builder(into)]
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<InputFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub ip_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<AllowedUpdate>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub secret_token: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct DeleteWebhookParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SendMessageParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    #[builder(into)]
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ForwardMessageParams {
    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    #[builder(into)]
    pub from_chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    pub message_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ForwardMessagesParams {
    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    #[builder(into)]
    pub from_chat_id: ChatId,

    pub message_ids: Vec<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct CopyMessageParams {
    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    #[builder(into)]
    pub from_chat_id: ChatId,

    pub message_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct CopyMessagesParams {
    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    #[builder(into)]
    pub from_chat_id: ChatId,

    pub message_ids: Vec<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_caption: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SendPhotoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    #[builder(into)]
    pub photo: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SendAudioParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    #[builder(into)]
    pub audio: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub performer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub thumbnail: Option<FileUpload>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SendDocumentParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    #[builder(into)]
    pub document: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub thumbnail: Option<FileUpload>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SendVideoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    #[builder(into)]
    pub video: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub thumbnail: Option<FileUpload>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SendAnimationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    #[builder(into)]
    pub animation: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub thumbnail: Option<FileUpload>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SendVoiceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    #[builder(into)]
    pub voice: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SendVideoNoteParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    #[builder(into)]
    pub video_note: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub thumbnail: Option<FileUpload>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SendPaidMediaParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat_id: ChatId,

    pub star_count: u32,

    pub media: Vec<InputPaidMedia>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub payload: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SendMediaGroupParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub media: Vec<Media>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct SendLocationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub latitude: f64,

    pub longitude: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct EditMessageLiveLocationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub inline_message_id: Option<String>,

    pub latitude: f64,

    pub longitude: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct StopMessageLiveLocationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub inline_message_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct SendVenueParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub latitude: f64,

    pub longitude: f64,

    #[builder(into)]
    pub title: String,

    #[builder(into)]
    pub address: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub foursquare_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub foursquare_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub google_place_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub google_place_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SendContactParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    #[builder(into)]
    pub phone_number: String,

    #[builder(into)]
    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub vcard: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SendPollParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    #[builder(into)]
    pub question: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_entities: Option<Vec<MessageEntity>>,

    pub options: Vec<InputPollOption>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<PollType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_multiple_answers: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub explanation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_closed: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SendDiceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub emoji: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SendChatActionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub action: ChatAction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetMessageReactionParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub message_id: i32,

    pub reaction: Vec<ReactionType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_big: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GetUserProfilePhotosParams {
    pub user_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GetFileParams {
    #[builder(into)]
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BanChatMemberParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub user_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_messages: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct UnbanChatMemberParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub user_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_if_banned: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct RestrictChatMemberParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub user_id: u64,

    pub permissions: ChatPermissions,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_independent_chat_permissions: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PromoteChatMemberParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub user_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_chat: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_stories: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_stories: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_stories: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_video_chats: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetChatAdministratorCustomTitleParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub user_id: u64,

    #[builder(into)]
    pub custom_title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BanChatSenderChatParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub sender_chat_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct UnbanChatSenderChatParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub sender_chat_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetChatPermissionsParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub permissions: ChatPermissions,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_independent_chat_permissions: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ExportChatInviteLinkParams {
    #[builder(into)]
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct CreateChatInviteLinkParams {
    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct EditChatInviteLinkParams {
    #[builder(into)]
    pub chat_id: ChatId,

    #[builder(into)]
    pub invite_link: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct CreateChatSubscriptionInviteLinkParams {
    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub name: Option<String>,

    pub subscription_period: u32,

    pub subscription_price: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct EditChatSubscriptionInviteLinkParams {
    #[builder(into)]
    pub chat_id: ChatId,

    #[builder(into)]
    pub invite_link: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct RevokeChatInviteLinkParams {
    #[builder(into)]
    pub chat_id: ChatId,

    #[builder(into)]
    pub invite_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ApproveChatJoinRequestParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub user_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct DeclineChatJoinRequestParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub user_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetChatPhotoParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub photo: InputFile,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct DeleteChatPhotoParams {
    #[builder(into)]
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetChatTitleParams {
    #[builder(into)]
    pub chat_id: ChatId,

    #[builder(into)]
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetChatDescriptionParams {
    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PinChatMessageParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat_id: ChatId,

    pub message_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct UnpinChatMessageParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct UnpinAllChatMessagesParams {
    #[builder(into)]
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct LeaveChatParams {
    #[builder(into)]
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GetChatParams {
    #[builder(into)]
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GetChatAdministratorsParams {
    #[builder(into)]
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GetChatMemberCountParams {
    #[builder(into)]
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GetChatMemberParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub user_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetChatStickerSetParams {
    #[builder(into)]
    pub chat_id: ChatId,

    #[builder(into)]
    pub sticker_set_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct DeleteChatStickerSetParams {
    #[builder(into)]
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct CreateForumTopicParams {
    #[builder(into)]
    pub chat_id: ChatId,

    #[builder(into)]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct EditForumTopicParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub message_thread_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct CloseForumTopicParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub message_thread_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ReopenForumTopicParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub message_thread_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct DeleteForumTopicParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub message_thread_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct UnpinAllForumTopicMessagesParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub message_thread_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct EditGeneralForumTopicParams {
    #[builder(into)]
    pub chat_id: ChatId,

    #[builder(into)]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct CloseGeneralForumTopicParams {
    #[builder(into)]
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ReopenGeneralForumTopicParams {
    #[builder(into)]
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct HideGeneralForumTopicParams {
    #[builder(into)]
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct UnhideGeneralForumTopicParams {
    #[builder(into)]
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct AnswerCallbackQueryParams {
    #[builder(into)]
    pub callback_query_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GetUserChatBoostsParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub user_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GetBusinessConnectionParams {
    #[builder(into)]
    pub business_connection_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetMyCommandsParams {
    pub commands: Vec<BotCommand>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetMyNameParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GetMyNameParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetMyDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GetMyDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetMyShortDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub short_description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GetMyShortDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GetMyCommandsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct DeleteMyCommandsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct EditMessageTextParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub inline_message_id: Option<String>,

    #[builder(into)]
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct EditMessageCaptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub inline_message_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct EditMessageMediaParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub inline_message_id: Option<String>,

    pub media: InputMedia,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct EditMessageReplyMarkupParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub inline_message_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct StopPollParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat_id: ChatId,

    pub message_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct DeleteMessageParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub message_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct DeleteMessagesParams {
    #[builder(into)]
    pub chat_id: ChatId,

    pub message_ids: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SendStickerParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    #[builder(into)]
    pub sticker: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub emoji: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GetStickerSetParams {
    #[builder(into)]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct UploadStickerFileParams {
    pub user_id: u64,

    pub sticker: InputFile,

    pub sticker_format: StickerFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct CreateNewStickerSetParams {
    pub user_id: u64,

    #[builder(into)]
    pub name: String,

    #[builder(into)]
    pub title: String,

    pub stickers: Vec<InputSticker>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_type: Option<StickerType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GetCustomEmojiStickersParams {
    pub custom_emoji_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct AddStickerToSetParams {
    pub user_id: u64,

    #[builder(into)]
    pub name: String,

    pub sticker: InputSticker,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetStickerPositionInSetParams {
    #[builder(into)]
    pub sticker: String,

    pub position: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct DeleteStickerFromSetParams {
    #[builder(into)]
    pub sticker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ReplaceStickerInSetParams {
    pub user_id: u64,

    #[builder(into)]
    pub name: String,

    #[builder(into)]
    pub old_sticker: String,

    pub sticker: InputSticker,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetStickerEmojiListParams {
    #[builder(into)]
    pub sticker: String,

    pub emoji_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetStickerKeywordsParams {
    #[builder(into)]
    pub sticker: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct SetStickerMaskPositionParams {
    #[builder(into)]
    pub sticker: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetStickerSetTitleParams {
    #[builder(into)]
    pub name: String,

    #[builder(into)]
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetStickerSetThumbnailParams {
    #[builder(into)]
    pub name: String,

    pub user_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub thumbnail: Option<FileUpload>,

    pub format: StickerFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetCustomEmojiStickerSetThumbnailParams {
    #[builder(into)]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct DeleteStickerSetParams {
    #[builder(into)]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct AnswerInlineQueryParams {
    #[builder(into)]
    pub inline_query_id: String,

    pub results: Vec<InlineQueryResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub next_offset: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<InlineQueryResultsButton>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InlineQueryResultsButton {
    #[builder(into)]
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub start_parameter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SendInvoiceParams {
    #[builder(into)]
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    #[builder(into)]
    pub title: String,

    #[builder(into)]
    pub description: String,

    #[builder(into)]
    pub payload: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub provider_token: Option<String>,

    #[builder(into)]
    pub currency: String,

    pub prices: Vec<LabeledPrice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<u32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub start_parameter: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub provider_data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub photo_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct CreateInvoiceLinkParams {
    #[builder(into)]
    pub title: String,

    #[builder(into)]
    pub description: String,

    #[builder(into)]
    pub payload: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub provider_token: Option<String>,

    #[builder(into)]
    pub currency: String,

    pub prices: Vec<LabeledPrice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<u32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub provider_data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub photo_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct AnswerShippingQueryParams {
    #[builder(into)]
    pub shipping_query_id: String,

    pub ok: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<ShippingOption>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct AnswerPreCheckoutQueryParams {
    #[builder(into)]
    pub pre_checkout_query_id: String,

    pub ok: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GetStarTransactionsParams {
    offset: u32,

    limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct RefundStarPaymentParams {
    pub user_id: u64,

    pub telegram_payment_charge_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetPassportDataErrorsParams {
    pub user_id: u64,

    pub errors: Vec<PassportElementError>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SendGameParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    pub chat_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    #[builder(into)]
    pub game_short_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetGameScoreParams {
    pub user_id: u64,

    pub score: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_edit_message: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub inline_message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GetGameHighScoresParams {
    pub user_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub inline_message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InputMediaPhoto {
    #[builder(into)]
    pub media: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InputMediaVideo {
    #[builder(into)]
    pub media: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub thumbnail: Option<FileUpload>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InputMediaAnimation {
    #[builder(into)]
    pub media: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub thumbnail: Option<FileUpload>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InputMediaAudio {
    #[builder(into)]
    pub media: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub thumbnail: Option<FileUpload>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub performer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InputMediaDocument {
    #[builder(into)]
    pub media: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub thumbnail: Option<FileUpload>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetMyDefaultAdministratorRightsParams {
    pub rights: ChatAdministratorRights,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GetMyDefaultAdministratorRightsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct AnswerWebAppQueryParams {
    #[builder(into)]
    pub web_app_query_id: String,

    pub result: InlineQueryResult,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SetChatMenuButtonParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_button: Option<MenuButton>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GetChatMenuButtonParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct UnpinAllGeneralForumTopicMessagesParams {
    #[builder(into)]
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ReplyParameters {
    pub message_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub quote: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_position: Option<u32>,
}
