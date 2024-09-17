//! Parameters to Telegram API methods.

use crate::macros::builder;
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
use macro_rules_attribute::apply;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

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

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BotCommandScopeChat {
    pub chat_id: ChatId,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BotCommandScopeChatAdministrators {
    pub chat_id: ChatId,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BotCommandScopeChatMember {
    pub chat_id: ChatId,
    pub user_id: u64,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InputFile {
    pub path: PathBuf,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetWebhookParams {
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<InputFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<AllowedUpdate>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_token: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteWebhookParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendMessageParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

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
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ForwardMessageParams {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub from_chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    pub message_id: i32,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ForwardMessagesParams {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub from_chat_id: ChatId,

    pub message_ids: Vec<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CopyMessageParams {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub from_chat_id: ChatId,

    pub message_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
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

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CopyMessagesParams {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub from_chat_id: ChatId,

    pub message_ids: Vec<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_caption: Option<bool>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendPhotoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub photo: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendAudioParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub audio: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<FileUpload>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendDocumentParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub document: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<FileUpload>,

    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendVideoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub video: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<FileUpload>,

    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendAnimationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub animation: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<FileUpload>,

    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendVoiceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub voice: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendVideoNoteParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub video_note: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<FileUpload>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendPaidMediaParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    pub chat_id: ChatId,

    pub star_count: u32,

    pub media: Vec<InputPaidMedia>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
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

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendMediaGroupParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub media: Vec<Media>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendLocationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

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
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EditMessageLiveLocationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
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

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StopMessageLiveLocationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendVenueParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub latitude: f64,

    pub longitude: f64,

    pub title: String,

    pub address: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendContactParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub phone_number: String,

    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendPollParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

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
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendDiceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendChatActionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub action: ChatAction,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetMessageReactionParams {
    pub chat_id: ChatId,

    pub message_id: i32,

    pub reaction: Vec<ReactionType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_big: Option<bool>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetUserProfilePhotosParams {
    pub user_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetFileParams {
    pub file_id: String,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BanChatMemberParams {
    pub chat_id: ChatId,

    pub user_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_messages: Option<bool>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UnbanChatMemberParams {
    pub chat_id: ChatId,

    pub user_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_if_banned: Option<bool>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RestrictChatMemberParams {
    pub chat_id: ChatId,

    pub user_id: u64,

    pub permissions: ChatPermissions,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_independent_chat_permissions: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<u64>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromoteChatMemberParams {
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

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetChatAdministratorCustomTitleParams {
    pub chat_id: ChatId,

    pub user_id: u64,

    pub custom_title: String,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BanChatSenderChatParams {
    pub chat_id: ChatId,

    pub sender_chat_id: i64,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UnbanChatSenderChatParams {
    pub chat_id: ChatId,

    pub sender_chat_id: i64,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetChatPermissionsParams {
    pub chat_id: ChatId,

    pub permissions: ChatPermissions,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_independent_chat_permissions: Option<bool>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExportChatInviteLinkParams {
    pub chat_id: ChatId,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateChatInviteLinkParams {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EditChatInviteLinkParams {
    pub chat_id: ChatId,

    pub invite_link: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateChatSubscriptionInviteLinkParams {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    pub subscription_period: u32,

    pub subscription_price: u16,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EditChatSubscriptionInviteLinkParams {
    pub chat_id: ChatId,

    pub invite_link: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RevokeChatInviteLinkParams {
    pub chat_id: ChatId,

    pub invite_link: String,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApproveChatJoinRequestParams {
    pub chat_id: ChatId,

    pub user_id: u64,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeclineChatJoinRequestParams {
    pub chat_id: ChatId,

    pub user_id: u64,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetChatPhotoParams {
    pub chat_id: ChatId,

    pub photo: InputFile,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteChatPhotoParams {
    pub chat_id: ChatId,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetChatTitleParams {
    pub chat_id: ChatId,

    pub title: String,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetChatDescriptionParams {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PinChatMessageParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    pub chat_id: ChatId,

    pub message_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UnpinChatMessageParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UnpinAllChatMessagesParams {
    pub chat_id: ChatId,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LeaveChatParams {
    pub chat_id: ChatId,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetChatParams {
    pub chat_id: ChatId,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetChatAdministratorsParams {
    pub chat_id: ChatId,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetChatMemberCountParams {
    pub chat_id: ChatId,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetChatMemberParams {
    pub chat_id: ChatId,

    pub user_id: u64,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetChatStickerSetParams {
    pub chat_id: ChatId,

    pub sticker_set_name: String,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteChatStickerSetParams {
    pub chat_id: ChatId,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateForumTopicParams {
    pub chat_id: ChatId,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EditForumTopicParams {
    pub chat_id: ChatId,

    pub message_thread_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CloseForumTopicParams {
    pub chat_id: ChatId,

    pub message_thread_id: i32,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReopenForumTopicParams {
    pub chat_id: ChatId,

    pub message_thread_id: i32,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteForumTopicParams {
    pub chat_id: ChatId,

    pub message_thread_id: i32,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UnpinAllForumTopicMessagesParams {
    pub chat_id: ChatId,

    pub message_thread_id: i32,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EditGeneralForumTopicParams {
    pub chat_id: ChatId,

    pub name: String,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CloseGeneralForumTopicParams {
    pub chat_id: ChatId,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReopenGeneralForumTopicParams {
    pub chat_id: ChatId,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HideGeneralForumTopicParams {
    pub chat_id: ChatId,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UnhideGeneralForumTopicParams {
    pub chat_id: ChatId,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnswerCallbackQueryParams {
    pub callback_query_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<u32>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetUserChatBoostsParams {
    pub chat_id: ChatId,

    pub user_id: u64,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetBusinessConnectionParams {
    pub business_connection_id: String,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetMyCommandsParams {
    pub commands: Vec<BotCommand>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetMyNameParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetMyNameParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetMyDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetMyDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetMyShortDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetMyShortDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetMyCommandsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteMyCommandsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EditMessageTextParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

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

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EditMessageCaptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
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

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EditMessageMediaParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    pub media: InputMedia,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EditMessageReplyMarkupParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StopPollParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    pub chat_id: ChatId,

    pub message_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteMessageParams {
    pub chat_id: ChatId,

    pub message_id: i32,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteMessagesParams {
    pub chat_id: ChatId,

    pub message_ids: Vec<i32>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendStickerParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub sticker: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetStickerSetParams {
    pub name: String,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UploadStickerFileParams {
    pub user_id: u64,

    pub sticker: InputFile,

    pub sticker_format: StickerFormat,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateNewStickerSetParams {
    pub user_id: u64,

    pub name: String,

    pub title: String,

    pub stickers: Vec<InputSticker>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_type: Option<StickerType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetCustomEmojiStickersParams {
    pub custom_emoji_ids: Vec<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AddStickerToSetParams {
    pub user_id: u64,

    pub name: String,

    pub sticker: InputSticker,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetStickerPositionInSetParams {
    pub sticker: String,

    pub position: u32,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteStickerFromSetParams {
    pub sticker: String,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReplaceStickerInSetParams {
    pub user_id: u64,

    pub name: String,

    pub old_sticker: String,

    pub sticker: InputSticker,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetStickerEmojiListParams {
    pub sticker: String,

    pub emoji_list: Vec<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetStickerKeywordsParams {
    pub sticker: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetStickerMaskPositionParams {
    pub sticker: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetStickerSetTitleParams {
    pub name: String,

    pub title: String,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetStickerSetThumbnailParams {
    pub name: String,

    pub user_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<FileUpload>,

    pub format: StickerFormat,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetCustomEmojiStickerSetThumbnailParams {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteStickerSetParams {
    pub name: String,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnswerInlineQueryParams {
    pub inline_query_id: String,

    pub results: Vec<InlineQueryResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<InlineQueryResultsButton>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InlineQueryResultsButton {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendInvoiceParams {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub title: String,

    pub description: String,

    pub payload: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,

    pub currency: String,

    pub prices: Vec<LabeledPrice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<u32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateInvoiceLinkParams {
    pub title: String,

    pub description: String,

    pub payload: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,

    pub currency: String,

    pub prices: Vec<LabeledPrice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<u32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
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

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnswerShippingQueryParams {
    pub shipping_query_id: String,

    pub ok: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<ShippingOption>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnswerPreCheckoutQueryParams {
    pub pre_checkout_query_id: String,

    pub ok: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetStarTransactionsParams {
    offset: u32,

    limit: u32,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RefundStarPaymentParams {
    pub user_id: u64,

    pub telegram_payment_charge_id: String,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetPassportDataErrorsParams {
    pub user_id: u64,

    pub errors: Vec<PassportElementError>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendGameParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    pub chat_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    pub game_short_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
    pub inline_message_id: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetGameHighScoresParams {
    pub user_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InputMediaPhoto {
    pub media: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
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

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InputMediaVideo {
    pub media: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<FileUpload>,

    #[serde(skip_serializing_if = "Option::is_none")]
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

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InputMediaAnimation {
    pub media: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<FileUpload>,

    #[serde(skip_serializing_if = "Option::is_none")]
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

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InputMediaAudio {
    pub media: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<FileUpload>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InputMediaDocument {
    pub media: FileUpload,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<FileUpload>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetMyDefaultAdministratorRightsParams {
    pub rights: ChatAdministratorRights,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetMyDefaultAdministratorRightsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}
#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnswerWebAppQueryParams {
    pub web_app_query_id: String,

    pub result: InlineQueryResult,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetChatMenuButtonParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_button: Option<MenuButton>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetChatMenuButtonParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UnpinAllGeneralForumTopicMessagesParams {
    pub chat_id: ChatId,
}

#[apply(builder!)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplyParameters {
    pub message_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_position: Option<u32>,
}
