use crate::objects::{
    BotCommand, ChatPermissions, ForceReply, InlineKeyboardMarkup, InlineQueryResultArticle,
    InlineQueryResultAudio, InlineQueryResultCachedAudio, InlineQueryResultCachedDocument,
    InlineQueryResultCachedGif, InlineQueryResultCachedMpeg4Gif, InlineQueryResultCachedPhoto,
    InlineQueryResultCachedSticker, InlineQueryResultCachedVideo, InlineQueryResultCachedVoice,
    InlineQueryResultContact, InlineQueryResultDocument, InlineQueryResultGame,
    InlineQueryResultGif, InlineQueryResultLocation, InlineQueryResultMpeg4Gif,
    InlineQueryResultPhoto, InlineQueryResultVenue, InlineQueryResultVideo, InlineQueryResultVoice,
    LabeledPrice, MaskPosition, MessageEntity, PassportElementErrorDataField,
    PassportElementErrorFile, PassportElementErrorFiles, PassportElementErrorFrontSide,
    PassportElementErrorReverseSide, PassportElementErrorSelfie,
    PassportElementErrorTranslationFile, PassportElementErrorTranslationFiles,
    PassportElementErrorUnspecified, PollType, ReplyKeyboardMarkup, ReplyKeyboardRemove,
    ShippingOption,
};
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum File {
    InputFile(InputFile),
    String(String),
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ChatId {
    Integer(i64),
    String(String),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ChatAction {
    Typing,
    UploadPhoto,
    RecordVideo,
    UploadVideo,
    RecordVoice,
    UploadVoice,
    UploadDocument,
    FindLocation,
    RecordVideoNote,
    UploadVideoNote,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Media {
    #[serde(rename = "audio")]
    Audio(InputMediaAudio),
    #[serde(rename = "document")]
    Document(InputMediaDocument),
    #[serde(rename = "photo")]
    Photo(InputMediaPhoto),
    #[serde(rename = "video")]
    Video(InputMediaVideo),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum BotCommandScope {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "all_private_chats")]
    AllPrivateChats,
    #[serde(rename = "all_group_chats")]
    AllGroupChats,
    #[serde(rename = "all_chat_administrators")]
    AllChatAdministrators,
    #[serde(rename = "chat")]
    Chat(BotCommandScopeChat),
    #[serde(rename = "chat_administrators")]
    ChatAdministrators(BotCommandScopeChatAdministrators),
    #[serde(rename = "chat_member")]
    ChatMember(BotCommandScopeChatMember),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeChat {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeChatAdministrators {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeChatMember {
    pub chat_id: ChatId,
    pub user_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputFile {
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetUpdatesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetWebhookParams {
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<InputFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteWebhookParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendMessageParams {
    pub chat_id: ChatId,

    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForwardMessageParams {
    pub chat_id: ChatId,

    pub from_chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    pub message_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CopyMessageParams {
    pub chat_id: ChatId,

    pub from_chat_id: ChatId,

    pub message_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendPhotoParams {
    pub chat_id: ChatId,

    pub photo: File,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendAudioParams {
    pub chat_id: ChatId,

    pub audio: File,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendDocumentParams {
    pub chat_id: ChatId,

    pub document: File,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendVideoParams {
    pub chat_id: ChatId,

    pub video: File,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendAnimationParams {
    pub chat_id: ChatId,

    pub animation: File,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendVoiceParams {
    pub chat_id: ChatId,

    pub voice: File,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendVideoNoteParams {
    pub chat_id: ChatId,

    pub video_note: File,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendMediaGroupParams {
    pub chat_id: ChatId,

    pub media: Vec<Media>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendLocationParams {
    pub chat_id: ChatId,

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
    pub reply_to_message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EditMessageLiveLocationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    pub latitude: f64,

    pub longitude: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StopMessageLiveLocationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendVenueParams {
    pub chat_id: ChatId,

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
    pub reply_to_message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendContactParams {
    pub chat_id: ChatId,

    pub phone_number: String,

    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendPollParams {
    pub chat_id: ChatId,

    pub question: String,

    pub options: Vec<String>,

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
    pub explanation_parse_mode: Option<String>,

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
    pub reply_to_message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendDiceParams {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendChatActionParams {
    pub chat_id: ChatId,

    pub action: ChatAction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetUserProfilePhotosParams {
    pub user_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetFileParams {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BanChatMemberParams {
    pub chat_id: ChatId,

    pub user_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_messages: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnbanChatMemberParams {
    pub chat_id: ChatId,

    pub user_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_if_banned: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RestrictChatMemberParams {
    pub chat_id: ChatId,

    pub user_id: u64,

    pub permissions: ChatPermissions,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub can_manage_voice_chats: Option<bool>,

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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetChatAdministratorCustomTitleParams {
    pub chat_id: ChatId,

    pub user_id: u64,

    pub custom_title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetChatPermissionsParams {
    pub chat_id: ChatId,

    pub permissions: ChatPermissions,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportChatInviteLinkParams {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateChatInviteLinkParams {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EditChatInviteLinkParams {
    pub chat_id: ChatId,

    pub invite_link: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RevokeChatInviteLinkParams {
    pub chat_id: ChatId,

    pub invite_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetChatPhotoParams {
    pub chat_id: ChatId,

    pub photo: InputFile,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteChatPhotoParams {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetChatTitleParams {
    pub chat_id: ChatId,

    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetChatDescriptionParams {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PinChatMessageParams {
    pub chat_id: ChatId,

    pub message_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnpinChatMessageParams {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnpinAllChatMessagesParams {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LeaveChatParams {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetChatParams {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetChatAdministratorsParams {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetChatMemberCountParams {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetChatMemberParams {
    pub chat_id: ChatId,

    pub user_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetChatStickerSetParams {
    pub chat_id: ChatId,

    pub sticker_set_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteChatStickerSetParams {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetMyCommandsParams {
    pub commands: Vec<BotCommand>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetMyCommandsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteMyCommandsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EditMessageTextParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EditMessageCaptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EditMessageMediaParams {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EditMessageReplyMarkupParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StopPollParams {
    pub chat_id: ChatId,

    pub message_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteMessageParams {
    pub chat_id: ChatId,

    pub message_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendStickerParams {
    pub chat_id: ChatId,

    pub sticker: File,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetStickerSetParams {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UploadStickerFileParams {
    pub user_id: u64,

    pub png_sticker: InputFile,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateNewStickerSetParams {
    pub user_id: u64,

    pub name: String,

    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub png_sticker: Option<File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgs_sticker: Option<InputFile>,

    pub emojis: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_masks: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AddStickerToSetParams {
    pub user_id: u64,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub png_sticker: Option<File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgs_sticker: Option<InputFile>,

    pub emojis: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetStickerPositionInSetParams {
    pub sticker: String,

    pub position: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteStickerFromSetParams {
    pub sticker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetStickerSetThumbParams {
    pub name: String,

    pub user_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<File>,
}

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
    pub switch_pm_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_parameter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendInvoiceParams {
    pub chat_id: i64,

    pub title: String,

    pub description: String,

    pub payload: String,

    pub provider_token: String,

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
    pub reply_to_message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnswerShippingQueryParams {
    pub shipping_query_id: String,

    pub ok: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<ShippingOption>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnswerPreCheckoutQueryParams {
    pub pre_checkout_query_id: String,

    pub ok: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetPassportDataErrorsParams {
    pub user_id: u64,

    pub errors: Vec<PassportElementError>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendGameParams {
    pub chat_id: i64,

    pub game_short_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetGameHighScoresParams {
    pub user_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputMediaPhoto {
    pub media: File,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputMediaVideo {
    pub media: File,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputMediaAnimation {
    pub media: File,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputMediaAudio {
    pub media: File,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputMediaDocument {
    pub media: File,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,
}

impl GetUpdatesParams {
    pub fn new() -> Self {
        Self {
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }

    pub fn set_offset(&mut self, offset: Option<u32>) {
        self.offset = offset;
    }

    pub fn set_limit(&mut self, limit: Option<u32>) {
        self.limit = limit;
    }

    pub fn set_timeout(&mut self, timeout: Option<u32>) {
        self.timeout = timeout;
    }

    pub fn set_allowed_updates(&mut self, allowed_updates: Option<Vec<String>>) {
        self.allowed_updates = allowed_updates;
    }

    pub fn offset(&self) -> Option<u32> {
        self.offset
    }

    pub fn limit(&self) -> Option<u32> {
        self.limit
    }

    pub fn timeout(&self) -> Option<u32> {
        self.timeout
    }

    pub fn allowed_updates(&self) -> Option<Vec<String>> {
        self.allowed_updates.clone()
    }
}

impl SetWebhookParams {
    pub fn new(url: String) -> Self {
        Self {
            url,
            certificate: None,
            ip_address: None,
            max_connections: None,
            allowed_updates: None,
            drop_pending_updates: None,
        }
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn set_certificate(&mut self, certificate: Option<InputFile>) {
        self.certificate = certificate;
    }

    pub fn set_ip_address(&mut self, ip_address: Option<String>) {
        self.ip_address = ip_address;
    }

    pub fn set_max_connections(&mut self, max_connections: Option<u32>) {
        self.max_connections = max_connections;
    }

    pub fn set_allowed_updates(&mut self, allowed_updates: Option<Vec<String>>) {
        self.allowed_updates = allowed_updates;
    }

    pub fn set_drop_pending_updates(&mut self, drop_pending_updates: Option<bool>) {
        self.drop_pending_updates = drop_pending_updates;
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn certificate(&self) -> Option<InputFile> {
        self.certificate.clone()
    }

    pub fn ip_address(&self) -> Option<String> {
        self.ip_address.clone()
    }

    pub fn max_connections(&self) -> Option<u32> {
        self.max_connections
    }

    pub fn allowed_updates(&self) -> Option<Vec<String>> {
        self.allowed_updates.clone()
    }

    pub fn drop_pending_updates(&self) -> Option<bool> {
        self.drop_pending_updates
    }
}

impl DeleteWebhookParams {
    pub fn new() -> Self {
        Self {
            drop_pending_updates: None,
        }
    }

    pub fn set_drop_pending_updates(&mut self, drop_pending_updates: Option<bool>) {
        self.drop_pending_updates = drop_pending_updates;
    }

    pub fn drop_pending_updates(&self) -> Option<bool> {
        self.drop_pending_updates
    }
}

impl SendMessageParams {
    pub fn new(chat_id: ChatId, text: String) -> Self {
        Self {
            chat_id,
            text,
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn set_parse_mode(&mut self, parse_mode: Option<String>) {
        self.parse_mode = parse_mode;
    }

    pub fn set_entities(&mut self, entities: Option<Vec<MessageEntity>>) {
        self.entities = entities;
    }

    pub fn set_disable_web_page_preview(&mut self, disable_web_page_preview: Option<bool>) {
        self.disable_web_page_preview = disable_web_page_preview;
    }

    pub fn set_disable_notification(&mut self, disable_notification: Option<bool>) {
        self.disable_notification = disable_notification;
    }

    pub fn set_reply_to_message_id(&mut self, reply_to_message_id: Option<i32>) {
        self.reply_to_message_id = reply_to_message_id;
    }

    pub fn set_allow_sending_without_reply(&mut self, allow_sending_without_reply: Option<bool>) {
        self.allow_sending_without_reply = allow_sending_without_reply;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<ReplyMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn parse_mode(&self) -> Option<String> {
        self.parse_mode.clone()
    }

    pub fn entities(&self) -> Option<Vec<MessageEntity>> {
        self.entities.clone()
    }

    pub fn disable_web_page_preview(&self) -> Option<bool> {
        self.disable_web_page_preview
    }

    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }

    pub fn reply_to_message_id(&self) -> Option<i32> {
        self.reply_to_message_id
    }

    pub fn allow_sending_without_reply(&self) -> Option<bool> {
        self.allow_sending_without_reply
    }

    pub fn reply_markup(&self) -> Option<ReplyMarkup> {
        self.reply_markup.clone()
    }
}

impl ForwardMessageParams {
    pub fn new(chat_id: ChatId, from_chat_id: ChatId, message_id: i32) -> Self {
        Self {
            chat_id,
            from_chat_id,
            message_id,
            disable_notification: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_from_chat_id(&mut self, from_chat_id: ChatId) {
        self.from_chat_id = from_chat_id;
    }

    pub fn set_message_id(&mut self, message_id: i32) {
        self.message_id = message_id;
    }

    pub fn set_disable_notification(&mut self, disable_notification: Option<bool>) {
        self.disable_notification = disable_notification;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn from_chat_id(&self) -> ChatId {
        self.from_chat_id.clone()
    }

    pub fn message_id(&self) -> i32 {
        self.message_id
    }

    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }
}

impl CopyMessageParams {
    pub fn new(chat_id: ChatId, from_chat_id: ChatId, message_id: i32) -> Self {
        Self {
            chat_id,
            from_chat_id,
            message_id,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_from_chat_id(&mut self, from_chat_id: ChatId) {
        self.from_chat_id = from_chat_id;
    }

    pub fn set_message_id(&mut self, message_id: i32) {
        self.message_id = message_id;
    }

    pub fn set_caption(&mut self, caption: Option<String>) {
        self.caption = caption;
    }

    pub fn set_parse_mode(&mut self, parse_mode: Option<String>) {
        self.parse_mode = parse_mode;
    }

    pub fn set_caption_entities(&mut self, caption_entities: Option<Vec<MessageEntity>>) {
        self.caption_entities = caption_entities;
    }

    pub fn set_disable_notification(&mut self, disable_notification: Option<bool>) {
        self.disable_notification = disable_notification;
    }

    pub fn set_reply_to_message_id(&mut self, reply_to_message_id: Option<i32>) {
        self.reply_to_message_id = reply_to_message_id;
    }

    pub fn set_allow_sending_without_reply(&mut self, allow_sending_without_reply: Option<bool>) {
        self.allow_sending_without_reply = allow_sending_without_reply;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<ReplyMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn from_chat_id(&self) -> ChatId {
        self.from_chat_id.clone()
    }

    pub fn message_id(&self) -> i32 {
        self.message_id
    }

    pub fn caption(&self) -> Option<String> {
        self.caption.clone()
    }

    pub fn parse_mode(&self) -> Option<String> {
        self.parse_mode.clone()
    }

    pub fn caption_entities(&self) -> Option<Vec<MessageEntity>> {
        self.caption_entities.clone()
    }

    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }

    pub fn reply_to_message_id(&self) -> Option<i32> {
        self.reply_to_message_id
    }

    pub fn allow_sending_without_reply(&self) -> Option<bool> {
        self.allow_sending_without_reply
    }

    pub fn reply_markup(&self) -> Option<ReplyMarkup> {
        self.reply_markup.clone()
    }
}

impl SendPhotoParams {
    pub fn new(chat_id: ChatId, photo: File) -> Self {
        Self {
            chat_id,
            photo,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_photo(&mut self, photo: File) {
        self.photo = photo;
    }

    pub fn set_caption(&mut self, caption: Option<String>) {
        self.caption = caption;
    }

    pub fn set_parse_mode(&mut self, parse_mode: Option<String>) {
        self.parse_mode = parse_mode;
    }

    pub fn set_caption_entities(&mut self, caption_entities: Option<Vec<MessageEntity>>) {
        self.caption_entities = caption_entities;
    }

    pub fn set_disable_notification(&mut self, disable_notification: Option<bool>) {
        self.disable_notification = disable_notification;
    }

    pub fn set_reply_to_message_id(&mut self, reply_to_message_id: Option<i32>) {
        self.reply_to_message_id = reply_to_message_id;
    }

    pub fn set_allow_sending_without_reply(&mut self, allow_sending_without_reply: Option<bool>) {
        self.allow_sending_without_reply = allow_sending_without_reply;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<ReplyMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn photo(&self) -> File {
        self.photo.clone()
    }

    pub fn caption(&self) -> Option<String> {
        self.caption.clone()
    }

    pub fn parse_mode(&self) -> Option<String> {
        self.parse_mode.clone()
    }

    pub fn caption_entities(&self) -> Option<Vec<MessageEntity>> {
        self.caption_entities.clone()
    }

    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }

    pub fn reply_to_message_id(&self) -> Option<i32> {
        self.reply_to_message_id
    }

    pub fn allow_sending_without_reply(&self) -> Option<bool> {
        self.allow_sending_without_reply
    }

    pub fn reply_markup(&self) -> Option<ReplyMarkup> {
        self.reply_markup.clone()
    }
}

impl SendAudioParams {
    pub fn new(chat_id: ChatId, audio: File) -> Self {
        Self {
            chat_id,
            audio,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            duration: None,
            performer: None,
            title: None,
            thumb: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_audio(&mut self, audio: File) {
        self.audio = audio;
    }

    pub fn set_caption(&mut self, caption: Option<String>) {
        self.caption = caption;
    }

    pub fn set_parse_mode(&mut self, parse_mode: Option<String>) {
        self.parse_mode = parse_mode;
    }

    pub fn set_caption_entities(&mut self, caption_entities: Option<Vec<MessageEntity>>) {
        self.caption_entities = caption_entities;
    }

    pub fn set_duration(&mut self, duration: Option<u32>) {
        self.duration = duration;
    }

    pub fn set_performer(&mut self, performer: Option<String>) {
        self.performer = performer;
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    pub fn set_thumb(&mut self, thumb: Option<File>) {
        self.thumb = thumb;
    }

    pub fn set_disable_notification(&mut self, disable_notification: Option<bool>) {
        self.disable_notification = disable_notification;
    }

    pub fn set_reply_to_message_id(&mut self, reply_to_message_id: Option<i32>) {
        self.reply_to_message_id = reply_to_message_id;
    }

    pub fn set_allow_sending_without_reply(&mut self, allow_sending_without_reply: Option<bool>) {
        self.allow_sending_without_reply = allow_sending_without_reply;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<ReplyMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn audio(&self) -> File {
        self.audio.clone()
    }

    pub fn caption(&self) -> Option<String> {
        self.caption.clone()
    }

    pub fn parse_mode(&self) -> Option<String> {
        self.parse_mode.clone()
    }

    pub fn caption_entities(&self) -> Option<Vec<MessageEntity>> {
        self.caption_entities.clone()
    }

    pub fn duration(&self) -> Option<u32> {
        self.duration
    }

    pub fn performer(&self) -> Option<String> {
        self.performer.clone()
    }

    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }

    pub fn thumb(&self) -> Option<File> {
        self.thumb.clone()
    }

    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }

    pub fn reply_to_message_id(&self) -> Option<i32> {
        self.reply_to_message_id
    }

    pub fn allow_sending_without_reply(&self) -> Option<bool> {
        self.allow_sending_without_reply
    }

    pub fn reply_markup(&self) -> Option<ReplyMarkup> {
        self.reply_markup.clone()
    }
}

impl SendDocumentParams {
    pub fn new(chat_id: ChatId, document: File) -> Self {
        Self {
            chat_id,
            document,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_content_type_detection: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_document(&mut self, document: File) {
        self.document = document;
    }

    pub fn set_thumb(&mut self, thumb: Option<File>) {
        self.thumb = thumb;
    }

    pub fn set_caption(&mut self, caption: Option<String>) {
        self.caption = caption;
    }

    pub fn set_parse_mode(&mut self, parse_mode: Option<String>) {
        self.parse_mode = parse_mode;
    }

    pub fn set_caption_entities(&mut self, caption_entities: Option<Vec<MessageEntity>>) {
        self.caption_entities = caption_entities;
    }

    pub fn set_disable_content_type_detection(
        &mut self,
        disable_content_type_detection: Option<bool>,
    ) {
        self.disable_content_type_detection = disable_content_type_detection;
    }

    pub fn set_disable_notification(&mut self, disable_notification: Option<bool>) {
        self.disable_notification = disable_notification;
    }

    pub fn set_reply_to_message_id(&mut self, reply_to_message_id: Option<i32>) {
        self.reply_to_message_id = reply_to_message_id;
    }

    pub fn set_allow_sending_without_reply(&mut self, allow_sending_without_reply: Option<bool>) {
        self.allow_sending_without_reply = allow_sending_without_reply;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<ReplyMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn document(&self) -> File {
        self.document.clone()
    }

    pub fn thumb(&self) -> Option<File> {
        self.thumb.clone()
    }

    pub fn caption(&self) -> Option<String> {
        self.caption.clone()
    }

    pub fn parse_mode(&self) -> Option<String> {
        self.parse_mode.clone()
    }

    pub fn caption_entities(&self) -> Option<Vec<MessageEntity>> {
        self.caption_entities.clone()
    }

    pub fn disable_content_type_detection(&self) -> Option<bool> {
        self.disable_content_type_detection
    }

    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }

    pub fn reply_to_message_id(&self) -> Option<i32> {
        self.reply_to_message_id
    }

    pub fn allow_sending_without_reply(&self) -> Option<bool> {
        self.allow_sending_without_reply
    }

    pub fn reply_markup(&self) -> Option<ReplyMarkup> {
        self.reply_markup.clone()
    }
}

impl SendVideoParams {
    pub fn new(chat_id: ChatId, video: File) -> Self {
        Self {
            chat_id,
            video,
            duration: None,
            width: None,
            height: None,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            supports_streaming: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_video(&mut self, video: File) {
        self.video = video;
    }

    pub fn set_duration(&mut self, duration: Option<u32>) {
        self.duration = duration;
    }

    pub fn set_width(&mut self, width: Option<u32>) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: Option<u32>) {
        self.height = height;
    }

    pub fn set_thumb(&mut self, thumb: Option<File>) {
        self.thumb = thumb;
    }

    pub fn set_caption(&mut self, caption: Option<String>) {
        self.caption = caption;
    }

    pub fn set_parse_mode(&mut self, parse_mode: Option<String>) {
        self.parse_mode = parse_mode;
    }

    pub fn set_caption_entities(&mut self, caption_entities: Option<Vec<MessageEntity>>) {
        self.caption_entities = caption_entities;
    }

    pub fn set_supports_streaming(&mut self, supports_streaming: Option<bool>) {
        self.supports_streaming = supports_streaming;
    }

    pub fn set_disable_notification(&mut self, disable_notification: Option<bool>) {
        self.disable_notification = disable_notification;
    }

    pub fn set_reply_to_message_id(&mut self, reply_to_message_id: Option<i32>) {
        self.reply_to_message_id = reply_to_message_id;
    }

    pub fn set_allow_sending_without_reply(&mut self, allow_sending_without_reply: Option<bool>) {
        self.allow_sending_without_reply = allow_sending_without_reply;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<ReplyMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn video(&self) -> File {
        self.video.clone()
    }

    pub fn duration(&self) -> Option<u32> {
        self.duration
    }

    pub fn width(&self) -> Option<u32> {
        self.width
    }

    pub fn height(&self) -> Option<u32> {
        self.height
    }

    pub fn thumb(&self) -> Option<File> {
        self.thumb.clone()
    }

    pub fn caption(&self) -> Option<String> {
        self.caption.clone()
    }

    pub fn parse_mode(&self) -> Option<String> {
        self.parse_mode.clone()
    }

    pub fn caption_entities(&self) -> Option<Vec<MessageEntity>> {
        self.caption_entities.clone()
    }

    pub fn supports_streaming(&self) -> Option<bool> {
        self.supports_streaming
    }

    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }

    pub fn reply_to_message_id(&self) -> Option<i32> {
        self.reply_to_message_id
    }

    pub fn allow_sending_without_reply(&self) -> Option<bool> {
        self.allow_sending_without_reply
    }

    pub fn reply_markup(&self) -> Option<ReplyMarkup> {
        self.reply_markup.clone()
    }
}

impl SendAnimationParams {
    pub fn new(chat_id: ChatId, animation: File) -> Self {
        Self {
            chat_id,
            animation,
            duration: None,
            width: None,
            height: None,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_animation(&mut self, animation: File) {
        self.animation = animation;
    }

    pub fn set_duration(&mut self, duration: Option<u32>) {
        self.duration = duration;
    }

    pub fn set_width(&mut self, width: Option<u32>) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: Option<u32>) {
        self.height = height;
    }

    pub fn set_thumb(&mut self, thumb: Option<File>) {
        self.thumb = thumb;
    }

    pub fn set_caption(&mut self, caption: Option<String>) {
        self.caption = caption;
    }

    pub fn set_parse_mode(&mut self, parse_mode: Option<String>) {
        self.parse_mode = parse_mode;
    }

    pub fn set_caption_entities(&mut self, caption_entities: Option<Vec<MessageEntity>>) {
        self.caption_entities = caption_entities;
    }

    pub fn set_disable_notification(&mut self, disable_notification: Option<bool>) {
        self.disable_notification = disable_notification;
    }

    pub fn set_reply_to_message_id(&mut self, reply_to_message_id: Option<i32>) {
        self.reply_to_message_id = reply_to_message_id;
    }

    pub fn set_allow_sending_without_reply(&mut self, allow_sending_without_reply: Option<bool>) {
        self.allow_sending_without_reply = allow_sending_without_reply;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<ReplyMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn animation(&self) -> File {
        self.animation.clone()
    }

    pub fn duration(&self) -> Option<u32> {
        self.duration
    }

    pub fn width(&self) -> Option<u32> {
        self.width
    }

    pub fn height(&self) -> Option<u32> {
        self.height
    }

    pub fn thumb(&self) -> Option<File> {
        self.thumb.clone()
    }

    pub fn caption(&self) -> Option<String> {
        self.caption.clone()
    }

    pub fn parse_mode(&self) -> Option<String> {
        self.parse_mode.clone()
    }

    pub fn caption_entities(&self) -> Option<Vec<MessageEntity>> {
        self.caption_entities.clone()
    }

    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }

    pub fn reply_to_message_id(&self) -> Option<i32> {
        self.reply_to_message_id
    }

    pub fn allow_sending_without_reply(&self) -> Option<bool> {
        self.allow_sending_without_reply
    }

    pub fn reply_markup(&self) -> Option<ReplyMarkup> {
        self.reply_markup.clone()
    }
}

impl SendVoiceParams {
    pub fn new(chat_id: ChatId, voice: File) -> Self {
        Self {
            chat_id,
            voice,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            duration: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_voice(&mut self, voice: File) {
        self.voice = voice;
    }

    pub fn set_caption(&mut self, caption: Option<String>) {
        self.caption = caption;
    }

    pub fn set_parse_mode(&mut self, parse_mode: Option<String>) {
        self.parse_mode = parse_mode;
    }

    pub fn set_caption_entities(&mut self, caption_entities: Option<Vec<MessageEntity>>) {
        self.caption_entities = caption_entities;
    }

    pub fn set_duration(&mut self, duration: Option<u32>) {
        self.duration = duration;
    }

    pub fn set_disable_notification(&mut self, disable_notification: Option<bool>) {
        self.disable_notification = disable_notification;
    }

    pub fn set_reply_to_message_id(&mut self, reply_to_message_id: Option<i32>) {
        self.reply_to_message_id = reply_to_message_id;
    }

    pub fn set_allow_sending_without_reply(&mut self, allow_sending_without_reply: Option<bool>) {
        self.allow_sending_without_reply = allow_sending_without_reply;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<ReplyMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn voice(&self) -> File {
        self.voice.clone()
    }

    pub fn caption(&self) -> Option<String> {
        self.caption.clone()
    }

    pub fn parse_mode(&self) -> Option<String> {
        self.parse_mode.clone()
    }

    pub fn caption_entities(&self) -> Option<Vec<MessageEntity>> {
        self.caption_entities.clone()
    }

    pub fn duration(&self) -> Option<u32> {
        self.duration
    }

    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }

    pub fn reply_to_message_id(&self) -> Option<i32> {
        self.reply_to_message_id
    }

    pub fn allow_sending_without_reply(&self) -> Option<bool> {
        self.allow_sending_without_reply
    }

    pub fn reply_markup(&self) -> Option<ReplyMarkup> {
        self.reply_markup.clone()
    }
}

impl SendVideoNoteParams {
    pub fn new(chat_id: ChatId, video_note: File) -> Self {
        Self {
            chat_id,
            video_note,
            duration: None,
            length: None,
            thumb: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_video_note(&mut self, video_note: File) {
        self.video_note = video_note;
    }

    pub fn set_duration(&mut self, duration: Option<u32>) {
        self.duration = duration;
    }

    pub fn set_length(&mut self, length: Option<u32>) {
        self.length = length;
    }

    pub fn set_thumb(&mut self, thumb: Option<File>) {
        self.thumb = thumb;
    }

    pub fn set_disable_notification(&mut self, disable_notification: Option<bool>) {
        self.disable_notification = disable_notification;
    }

    pub fn set_reply_to_message_id(&mut self, reply_to_message_id: Option<i32>) {
        self.reply_to_message_id = reply_to_message_id;
    }

    pub fn set_allow_sending_without_reply(&mut self, allow_sending_without_reply: Option<bool>) {
        self.allow_sending_without_reply = allow_sending_without_reply;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<ReplyMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn video_note(&self) -> File {
        self.video_note.clone()
    }

    pub fn duration(&self) -> Option<u32> {
        self.duration
    }

    pub fn length(&self) -> Option<u32> {
        self.length
    }

    pub fn thumb(&self) -> Option<File> {
        self.thumb.clone()
    }

    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }

    pub fn reply_to_message_id(&self) -> Option<i32> {
        self.reply_to_message_id
    }

    pub fn allow_sending_without_reply(&self) -> Option<bool> {
        self.allow_sending_without_reply
    }

    pub fn reply_markup(&self) -> Option<ReplyMarkup> {
        self.reply_markup.clone()
    }
}

impl SendMediaGroupParams {
    pub fn new(chat_id: ChatId, media: Vec<Media>) -> Self {
        Self {
            chat_id,
            media,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_media(&mut self, media: Vec<Media>) {
        self.media = media;
    }

    pub fn set_disable_notification(&mut self, disable_notification: Option<bool>) {
        self.disable_notification = disable_notification;
    }

    pub fn set_reply_to_message_id(&mut self, reply_to_message_id: Option<i32>) {
        self.reply_to_message_id = reply_to_message_id;
    }

    pub fn set_allow_sending_without_reply(&mut self, allow_sending_without_reply: Option<bool>) {
        self.allow_sending_without_reply = allow_sending_without_reply;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn media(&self) -> Vec<Media> {
        self.media.clone()
    }

    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }

    pub fn reply_to_message_id(&self) -> Option<i32> {
        self.reply_to_message_id
    }

    pub fn allow_sending_without_reply(&self) -> Option<bool> {
        self.allow_sending_without_reply
    }
}

impl SendLocationParams {
    pub fn new(chat_id: ChatId, latitude: f64, longitude: f64) -> Self {
        Self {
            chat_id,
            latitude,
            longitude,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_latitude(&mut self, latitude: f64) {
        self.latitude = latitude;
    }

    pub fn set_longitude(&mut self, longitude: f64) {
        self.longitude = longitude;
    }

    pub fn set_horizontal_accuracy(&mut self, horizontal_accuracy: Option<f64>) {
        self.horizontal_accuracy = horizontal_accuracy;
    }

    pub fn set_live_period(&mut self, live_period: Option<u32>) {
        self.live_period = live_period;
    }

    pub fn set_heading(&mut self, heading: Option<u16>) {
        self.heading = heading;
    }

    pub fn set_proximity_alert_radius(&mut self, proximity_alert_radius: Option<u32>) {
        self.proximity_alert_radius = proximity_alert_radius;
    }

    pub fn set_disable_notification(&mut self, disable_notification: Option<bool>) {
        self.disable_notification = disable_notification;
    }

    pub fn set_reply_to_message_id(&mut self, reply_to_message_id: Option<i32>) {
        self.reply_to_message_id = reply_to_message_id;
    }

    pub fn set_allow_sending_without_reply(&mut self, allow_sending_without_reply: Option<bool>) {
        self.allow_sending_without_reply = allow_sending_without_reply;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<ReplyMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    pub fn longitude(&self) -> f64 {
        self.longitude
    }

    pub fn horizontal_accuracy(&self) -> Option<f64> {
        self.horizontal_accuracy
    }

    pub fn live_period(&self) -> Option<u32> {
        self.live_period
    }

    pub fn heading(&self) -> Option<u16> {
        self.heading
    }

    pub fn proximity_alert_radius(&self) -> Option<u32> {
        self.proximity_alert_radius
    }

    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }

    pub fn reply_to_message_id(&self) -> Option<i32> {
        self.reply_to_message_id
    }

    pub fn allow_sending_without_reply(&self) -> Option<bool> {
        self.allow_sending_without_reply
    }

    pub fn reply_markup(&self) -> Option<ReplyMarkup> {
        self.reply_markup.clone()
    }
}

impl EditMessageLiveLocationParams {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
        }
    }

    pub fn set_latitude(&mut self, latitude: f64) {
        self.latitude = latitude;
    }

    pub fn set_longitude(&mut self, longitude: f64) {
        self.longitude = longitude;
    }

    pub fn set_chat_id(&mut self, chat_id: Option<ChatId>) {
        self.chat_id = chat_id;
    }

    pub fn set_message_id(&mut self, message_id: Option<i32>) {
        self.message_id = message_id;
    }

    pub fn set_inline_message_id(&mut self, inline_message_id: Option<String>) {
        self.inline_message_id = inline_message_id;
    }

    pub fn set_horizontal_accuracy(&mut self, horizontal_accuracy: Option<f64>) {
        self.horizontal_accuracy = horizontal_accuracy;
    }

    pub fn set_heading(&mut self, heading: Option<u16>) {
        self.heading = heading;
    }

    pub fn set_proximity_alert_radius(&mut self, proximity_alert_radius: Option<u32>) {
        self.proximity_alert_radius = proximity_alert_radius;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<InlineKeyboardMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    pub fn longitude(&self) -> f64 {
        self.longitude
    }

    pub fn chat_id(&self) -> Option<ChatId> {
        self.chat_id.clone()
    }

    pub fn message_id(&self) -> Option<i32> {
        self.message_id
    }

    pub fn inline_message_id(&self) -> Option<String> {
        self.inline_message_id.clone()
    }

    pub fn horizontal_accuracy(&self) -> Option<f64> {
        self.horizontal_accuracy
    }

    pub fn heading(&self) -> Option<u16> {
        self.heading
    }

    pub fn proximity_alert_radius(&self) -> Option<u32> {
        self.proximity_alert_radius
    }

    pub fn reply_markup(&self) -> Option<InlineKeyboardMarkup> {
        self.reply_markup.clone()
    }
}

impl StopMessageLiveLocationParams {
    pub fn new() -> Self {
        Self {
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: Option<ChatId>) {
        self.chat_id = chat_id;
    }

    pub fn set_message_id(&mut self, message_id: Option<i32>) {
        self.message_id = message_id;
    }

    pub fn set_inline_message_id(&mut self, inline_message_id: Option<String>) {
        self.inline_message_id = inline_message_id;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<InlineKeyboardMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> Option<ChatId> {
        self.chat_id.clone()
    }

    pub fn message_id(&self) -> Option<i32> {
        self.message_id
    }

    pub fn inline_message_id(&self) -> Option<String> {
        self.inline_message_id.clone()
    }

    pub fn reply_markup(&self) -> Option<InlineKeyboardMarkup> {
        self.reply_markup.clone()
    }
}

impl SendVenueParams {
    pub fn new(
        chat_id: ChatId,
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
    ) -> Self {
        Self {
            chat_id,
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_latitude(&mut self, latitude: f64) {
        self.latitude = latitude;
    }

    pub fn set_longitude(&mut self, longitude: f64) {
        self.longitude = longitude;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_address(&mut self, address: String) {
        self.address = address;
    }

    pub fn set_foursquare_id(&mut self, foursquare_id: Option<String>) {
        self.foursquare_id = foursquare_id;
    }

    pub fn set_foursquare_type(&mut self, foursquare_type: Option<String>) {
        self.foursquare_type = foursquare_type;
    }

    pub fn set_google_place_id(&mut self, google_place_id: Option<String>) {
        self.google_place_id = google_place_id;
    }

    pub fn set_google_place_type(&mut self, google_place_type: Option<String>) {
        self.google_place_type = google_place_type;
    }

    pub fn set_disable_notification(&mut self, disable_notification: Option<bool>) {
        self.disable_notification = disable_notification;
    }

    pub fn set_reply_to_message_id(&mut self, reply_to_message_id: Option<i32>) {
        self.reply_to_message_id = reply_to_message_id;
    }

    pub fn set_allow_sending_without_reply(&mut self, allow_sending_without_reply: Option<bool>) {
        self.allow_sending_without_reply = allow_sending_without_reply;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<ReplyMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    pub fn longitude(&self) -> f64 {
        self.longitude
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }

    pub fn foursquare_id(&self) -> Option<String> {
        self.foursquare_id.clone()
    }

    pub fn foursquare_type(&self) -> Option<String> {
        self.foursquare_type.clone()
    }

    pub fn google_place_id(&self) -> Option<String> {
        self.google_place_id.clone()
    }

    pub fn google_place_type(&self) -> Option<String> {
        self.google_place_type.clone()
    }

    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }

    pub fn reply_to_message_id(&self) -> Option<i32> {
        self.reply_to_message_id
    }

    pub fn allow_sending_without_reply(&self) -> Option<bool> {
        self.allow_sending_without_reply
    }

    pub fn reply_markup(&self) -> Option<ReplyMarkup> {
        self.reply_markup.clone()
    }
}

impl SendContactParams {
    pub fn new(chat_id: ChatId, phone_number: String, first_name: String) -> Self {
        Self {
            chat_id,
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_phone_number(&mut self, phone_number: String) {
        self.phone_number = phone_number;
    }

    pub fn set_first_name(&mut self, first_name: String) {
        self.first_name = first_name;
    }

    pub fn set_last_name(&mut self, last_name: Option<String>) {
        self.last_name = last_name;
    }

    pub fn set_vcard(&mut self, vcard: Option<String>) {
        self.vcard = vcard;
    }

    pub fn set_disable_notification(&mut self, disable_notification: Option<bool>) {
        self.disable_notification = disable_notification;
    }

    pub fn set_reply_to_message_id(&mut self, reply_to_message_id: Option<i32>) {
        self.reply_to_message_id = reply_to_message_id;
    }

    pub fn set_allow_sending_without_reply(&mut self, allow_sending_without_reply: Option<bool>) {
        self.allow_sending_without_reply = allow_sending_without_reply;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<ReplyMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn phone_number(&self) -> String {
        self.phone_number.clone()
    }

    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn last_name(&self) -> Option<String> {
        self.last_name.clone()
    }

    pub fn vcard(&self) -> Option<String> {
        self.vcard.clone()
    }

    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }

    pub fn reply_to_message_id(&self) -> Option<i32> {
        self.reply_to_message_id
    }

    pub fn allow_sending_without_reply(&self) -> Option<bool> {
        self.allow_sending_without_reply
    }

    pub fn reply_markup(&self) -> Option<ReplyMarkup> {
        self.reply_markup.clone()
    }
}

impl SendPollParams {
    pub fn new(chat_id: ChatId, question: String, options: Vec<String>) -> Self {
        Self {
            chat_id,
            question,
            options,
            is_anonymous: None,
            type_field: None,
            allows_multiple_answers: None,
            correct_option_id: None,
            explanation: None,
            explanation_parse_mode: None,
            explanation_entities: None,
            open_period: None,
            close_date: None,
            is_closed: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_question(&mut self, question: String) {
        self.question = question;
    }

    pub fn set_options(&mut self, options: Vec<String>) {
        self.options = options;
    }

    pub fn set_is_anonymous(&mut self, is_anonymous: Option<bool>) {
        self.is_anonymous = is_anonymous;
    }

    pub fn set_type_field(&mut self, type_field: Option<PollType>) {
        self.type_field = type_field;
    }

    pub fn set_allows_multiple_answers(&mut self, allows_multiple_answers: Option<bool>) {
        self.allows_multiple_answers = allows_multiple_answers;
    }

    pub fn set_correct_option_id(&mut self, correct_option_id: Option<u8>) {
        self.correct_option_id = correct_option_id;
    }

    pub fn set_explanation(&mut self, explanation: Option<String>) {
        self.explanation = explanation;
    }

    pub fn set_explanation_parse_mode(&mut self, explanation_parse_mode: Option<String>) {
        self.explanation_parse_mode = explanation_parse_mode;
    }

    pub fn set_explanation_entities(&mut self, explanation_entities: Option<Vec<MessageEntity>>) {
        self.explanation_entities = explanation_entities;
    }

    pub fn set_open_period(&mut self, open_period: Option<u32>) {
        self.open_period = open_period;
    }

    pub fn set_close_date(&mut self, close_date: Option<u64>) {
        self.close_date = close_date;
    }

    pub fn set_is_closed(&mut self, is_closed: Option<bool>) {
        self.is_closed = is_closed;
    }

    pub fn set_disable_notification(&mut self, disable_notification: Option<bool>) {
        self.disable_notification = disable_notification;
    }

    pub fn set_reply_to_message_id(&mut self, reply_to_message_id: Option<i32>) {
        self.reply_to_message_id = reply_to_message_id;
    }

    pub fn set_allow_sending_without_reply(&mut self, allow_sending_without_reply: Option<bool>) {
        self.allow_sending_without_reply = allow_sending_without_reply;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<ReplyMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn question(&self) -> String {
        self.question.clone()
    }

    pub fn options(&self) -> Vec<String> {
        self.options.clone()
    }

    pub fn is_anonymous(&self) -> Option<bool> {
        self.is_anonymous
    }

    pub fn type_field(&self) -> Option<PollType> {
        self.type_field.clone()
    }

    pub fn allows_multiple_answers(&self) -> Option<bool> {
        self.allows_multiple_answers
    }

    pub fn correct_option_id(&self) -> Option<u8> {
        self.correct_option_id
    }

    pub fn explanation(&self) -> Option<String> {
        self.explanation.clone()
    }

    pub fn explanation_parse_mode(&self) -> Option<String> {
        self.explanation_parse_mode.clone()
    }

    pub fn explanation_entities(&self) -> Option<Vec<MessageEntity>> {
        self.explanation_entities.clone()
    }

    pub fn open_period(&self) -> Option<u32> {
        self.open_period
    }

    pub fn close_date(&self) -> Option<u64> {
        self.close_date
    }

    pub fn is_closed(&self) -> Option<bool> {
        self.is_closed
    }

    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }

    pub fn reply_to_message_id(&self) -> Option<i32> {
        self.reply_to_message_id
    }

    pub fn allow_sending_without_reply(&self) -> Option<bool> {
        self.allow_sending_without_reply
    }

    pub fn reply_markup(&self) -> Option<ReplyMarkup> {
        self.reply_markup.clone()
    }
}

impl SendDiceParams {
    pub fn new(chat_id: ChatId) -> Self {
        Self {
            chat_id,
            emoji: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_emoji(&mut self, emoji: Option<String>) {
        self.emoji = emoji;
    }

    pub fn set_disable_notification(&mut self, disable_notification: Option<bool>) {
        self.disable_notification = disable_notification;
    }

    pub fn set_reply_to_message_id(&mut self, reply_to_message_id: Option<i32>) {
        self.reply_to_message_id = reply_to_message_id;
    }

    pub fn set_allow_sending_without_reply(&mut self, allow_sending_without_reply: Option<bool>) {
        self.allow_sending_without_reply = allow_sending_without_reply;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<ReplyMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn emoji(&self) -> Option<String> {
        self.emoji.clone()
    }

    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }

    pub fn reply_to_message_id(&self) -> Option<i32> {
        self.reply_to_message_id
    }

    pub fn allow_sending_without_reply(&self) -> Option<bool> {
        self.allow_sending_without_reply
    }

    pub fn reply_markup(&self) -> Option<ReplyMarkup> {
        self.reply_markup.clone()
    }
}

impl SendChatActionParams {
    pub fn new(chat_id: ChatId, action: ChatAction) -> Self {
        Self { chat_id, action }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_action(&mut self, action: ChatAction) {
        self.action = action;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn action(&self) -> ChatAction {
        self.action.clone()
    }
}

impl GetUserProfilePhotosParams {
    pub fn new(user_id: u64) -> Self {
        Self {
            user_id,
            offset: None,
            limit: None,
        }
    }

    pub fn set_user_id(&mut self, user_id: u64) {
        self.user_id = user_id;
    }

    pub fn set_offset(&mut self, offset: Option<u32>) {
        self.offset = offset;
    }

    pub fn set_limit(&mut self, limit: Option<u32>) {
        self.limit = limit;
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    pub fn offset(&self) -> Option<u32> {
        self.offset
    }

    pub fn limit(&self) -> Option<u32> {
        self.limit
    }
}

impl GetFileParams {
    pub fn new(file_id: String) -> Self {
        Self { file_id }
    }

    pub fn set_file_id(&mut self, file_id: String) {
        self.file_id = file_id;
    }

    pub fn file_id(&self) -> String {
        self.file_id.clone()
    }
}

impl BanChatMemberParams {
    pub fn new(chat_id: ChatId, user_id: u64) -> Self {
        Self {
            chat_id,
            user_id,
            until_date: None,
            revoke_messages: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_user_id(&mut self, user_id: u64) {
        self.user_id = user_id;
    }

    pub fn set_until_date(&mut self, until_date: Option<u64>) {
        self.until_date = until_date;
    }

    pub fn set_revoke_messages(&mut self, revoke_messages: Option<bool>) {
        self.revoke_messages = revoke_messages;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    pub fn until_date(&self) -> Option<u64> {
        self.until_date
    }

    pub fn revoke_messages(&self) -> Option<bool> {
        self.revoke_messages
    }
}

impl UnbanChatMemberParams {
    pub fn new(chat_id: ChatId, user_id: u64) -> Self {
        Self {
            chat_id,
            user_id,
            only_if_banned: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_user_id(&mut self, user_id: u64) {
        self.user_id = user_id;
    }

    pub fn set_only_if_banned(&mut self, only_if_banned: Option<bool>) {
        self.only_if_banned = only_if_banned;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    pub fn only_if_banned(&self) -> Option<bool> {
        self.only_if_banned
    }
}

impl RestrictChatMemberParams {
    pub fn new(chat_id: ChatId, user_id: u64, permissions: ChatPermissions) -> Self {
        Self {
            chat_id,
            user_id,
            permissions,
            until_date: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_user_id(&mut self, user_id: u64) {
        self.user_id = user_id;
    }

    pub fn set_permissions(&mut self, permissions: ChatPermissions) {
        self.permissions = permissions;
    }

    pub fn set_until_date(&mut self, until_date: Option<u64>) {
        self.until_date = until_date;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    pub fn permissions(&self) -> ChatPermissions {
        self.permissions.clone()
    }

    pub fn until_date(&self) -> Option<u64> {
        self.until_date
    }
}

impl PromoteChatMemberParams {
    pub fn new(chat_id: ChatId, user_id: u64) -> Self {
        Self {
            chat_id,
            user_id,
            is_anonymous: None,
            can_manage_chat: None,
            can_post_messages: None,
            can_edit_messages: None,
            can_delete_messages: None,
            can_manage_voice_chats: None,
            can_restrict_members: None,
            can_promote_members: None,
            can_change_info: None,
            can_invite_users: None,
            can_pin_messages: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_user_id(&mut self, user_id: u64) {
        self.user_id = user_id;
    }

    pub fn set_is_anonymous(&mut self, is_anonymous: Option<bool>) {
        self.is_anonymous = is_anonymous;
    }

    pub fn set_can_manage_chat(&mut self, can_manage_chat: Option<bool>) {
        self.can_manage_chat = can_manage_chat;
    }

    pub fn set_can_post_messages(&mut self, can_post_messages: Option<bool>) {
        self.can_post_messages = can_post_messages;
    }

    pub fn set_can_edit_messages(&mut self, can_edit_messages: Option<bool>) {
        self.can_edit_messages = can_edit_messages;
    }

    pub fn set_can_delete_messages(&mut self, can_delete_messages: Option<bool>) {
        self.can_delete_messages = can_delete_messages;
    }

    pub fn set_can_manage_voice_chats(&mut self, can_manage_voice_chats: Option<bool>) {
        self.can_manage_voice_chats = can_manage_voice_chats;
    }

    pub fn set_can_restrict_members(&mut self, can_restrict_members: Option<bool>) {
        self.can_restrict_members = can_restrict_members;
    }

    pub fn set_can_promote_members(&mut self, can_promote_members: Option<bool>) {
        self.can_promote_members = can_promote_members;
    }

    pub fn set_can_change_info(&mut self, can_change_info: Option<bool>) {
        self.can_change_info = can_change_info;
    }

    pub fn set_can_invite_users(&mut self, can_invite_users: Option<bool>) {
        self.can_invite_users = can_invite_users;
    }

    pub fn set_can_pin_messages(&mut self, can_pin_messages: Option<bool>) {
        self.can_pin_messages = can_pin_messages;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    pub fn is_anonymous(&self) -> Option<bool> {
        self.is_anonymous
    }

    pub fn can_manage_chat(&self) -> Option<bool> {
        self.can_manage_chat
    }

    pub fn can_post_messages(&self) -> Option<bool> {
        self.can_post_messages
    }

    pub fn can_edit_messages(&self) -> Option<bool> {
        self.can_edit_messages
    }

    pub fn can_delete_messages(&self) -> Option<bool> {
        self.can_delete_messages
    }

    pub fn can_manage_voice_chats(&self) -> Option<bool> {
        self.can_manage_voice_chats
    }

    pub fn can_restrict_members(&self) -> Option<bool> {
        self.can_restrict_members
    }

    pub fn can_promote_members(&self) -> Option<bool> {
        self.can_promote_members
    }

    pub fn can_change_info(&self) -> Option<bool> {
        self.can_change_info
    }

    pub fn can_invite_users(&self) -> Option<bool> {
        self.can_invite_users
    }

    pub fn can_pin_messages(&self) -> Option<bool> {
        self.can_pin_messages
    }
}

impl SetChatAdministratorCustomTitleParams {
    pub fn new(chat_id: ChatId, user_id: u64, custom_title: String) -> Self {
        Self {
            chat_id,
            user_id,
            custom_title,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_user_id(&mut self, user_id: u64) {
        self.user_id = user_id;
    }

    pub fn set_custom_title(&mut self, custom_title: String) {
        self.custom_title = custom_title;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    pub fn custom_title(&self) -> String {
        self.custom_title.clone()
    }
}

impl SetChatPermissionsParams {
    pub fn new(chat_id: ChatId, permissions: ChatPermissions) -> Self {
        Self {
            chat_id,
            permissions,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_permissions(&mut self, permissions: ChatPermissions) {
        self.permissions = permissions;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn permissions(&self) -> ChatPermissions {
        self.permissions.clone()
    }
}

impl ExportChatInviteLinkParams {
    pub fn new(chat_id: ChatId) -> Self {
        Self { chat_id }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }
}

impl CreateChatInviteLinkParams {
    pub fn new(chat_id: ChatId) -> Self {
        Self {
            chat_id,
            expire_date: None,
            member_limit: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_expire_date(&mut self, expire_date: Option<u64>) {
        self.expire_date = expire_date;
    }

    pub fn set_member_limit(&mut self, member_limit: Option<u32>) {
        self.member_limit = member_limit;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn expire_date(&self) -> Option<u64> {
        self.expire_date
    }

    pub fn member_limit(&self) -> Option<u32> {
        self.member_limit
    }
}

impl EditChatInviteLinkParams {
    pub fn new(chat_id: ChatId, invite_link: String) -> Self {
        Self {
            chat_id,
            invite_link,
            expire_date: None,
            member_limit: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_invite_link(&mut self, invite_link: String) {
        self.invite_link = invite_link;
    }

    pub fn set_expire_date(&mut self, expire_date: Option<u64>) {
        self.expire_date = expire_date;
    }

    pub fn set_member_limit(&mut self, member_limit: Option<u32>) {
        self.member_limit = member_limit;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn invite_link(&self) -> String {
        self.invite_link.clone()
    }

    pub fn expire_date(&self) -> Option<u64> {
        self.expire_date
    }

    pub fn member_limit(&self) -> Option<u32> {
        self.member_limit
    }
}

impl RevokeChatInviteLinkParams {
    pub fn new(chat_id: ChatId, invite_link: String) -> Self {
        Self {
            chat_id,
            invite_link,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_invite_link(&mut self, invite_link: String) {
        self.invite_link = invite_link;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn invite_link(&self) -> String {
        self.invite_link.clone()
    }
}

impl SetChatPhotoParams {
    pub fn new(chat_id: ChatId, photo: InputFile) -> Self {
        Self { chat_id, photo }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_photo(&mut self, photo: InputFile) {
        self.photo = photo;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn photo(&self) -> InputFile {
        self.photo.clone()
    }
}

impl DeleteChatPhotoParams {
    pub fn new(chat_id: ChatId) -> Self {
        Self { chat_id }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }
}

impl SetChatTitleParams {
    pub fn new(chat_id: ChatId, title: String) -> Self {
        Self { chat_id, title }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }
}

impl SetChatDescriptionParams {
    pub fn new(chat_id: ChatId) -> Self {
        Self {
            chat_id,
            description: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }
}

impl PinChatMessageParams {
    pub fn new(chat_id: ChatId, message_id: i32) -> Self {
        Self {
            chat_id,
            message_id,
            disable_notification: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_message_id(&mut self, message_id: i32) {
        self.message_id = message_id;
    }

    pub fn set_disable_notification(&mut self, disable_notification: Option<bool>) {
        self.disable_notification = disable_notification;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn message_id(&self) -> i32 {
        self.message_id
    }

    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }
}

impl UnpinChatMessageParams {
    pub fn new(chat_id: ChatId) -> Self {
        Self {
            chat_id,
            message_id: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_message_id(&mut self, message_id: Option<i32>) {
        self.message_id = message_id;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn message_id(&self) -> Option<i32> {
        self.message_id
    }
}

impl UnpinAllChatMessagesParams {
    pub fn new(chat_id: ChatId) -> Self {
        Self { chat_id }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }
}

impl LeaveChatParams {
    pub fn new(chat_id: ChatId) -> Self {
        Self { chat_id }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }
}

impl GetChatParams {
    pub fn new(chat_id: ChatId) -> Self {
        Self { chat_id }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }
}

impl GetChatAdministratorsParams {
    pub fn new(chat_id: ChatId) -> Self {
        Self { chat_id }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }
}

impl GetChatMemberCountParams {
    pub fn new(chat_id: ChatId) -> Self {
        Self { chat_id }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }
}

impl GetChatMemberParams {
    pub fn new(chat_id: ChatId, user_id: u64) -> Self {
        Self { chat_id, user_id }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_user_id(&mut self, user_id: u64) {
        self.user_id = user_id;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}

impl SetChatStickerSetParams {
    pub fn new(chat_id: ChatId, sticker_set_name: String) -> Self {
        Self {
            chat_id,
            sticker_set_name,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_sticker_set_name(&mut self, sticker_set_name: String) {
        self.sticker_set_name = sticker_set_name;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn sticker_set_name(&self) -> String {
        self.sticker_set_name.clone()
    }
}

impl DeleteChatStickerSetParams {
    pub fn new(chat_id: ChatId) -> Self {
        Self { chat_id }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }
}

impl AnswerCallbackQueryParams {
    pub fn new(callback_query_id: String) -> Self {
        Self {
            callback_query_id,
            text: None,
            show_alert: None,
            url: None,
            cache_time: None,
        }
    }

    pub fn set_callback_query_id(&mut self, callback_query_id: String) {
        self.callback_query_id = callback_query_id;
    }

    pub fn set_text(&mut self, text: Option<String>) {
        self.text = text;
    }

    pub fn set_show_alert(&mut self, show_alert: Option<bool>) {
        self.show_alert = show_alert;
    }

    pub fn set_url(&mut self, url: Option<String>) {
        self.url = url;
    }

    pub fn set_cache_time(&mut self, cache_time: Option<u32>) {
        self.cache_time = cache_time;
    }

    pub fn callback_query_id(&self) -> String {
        self.callback_query_id.clone()
    }

    pub fn text(&self) -> Option<String> {
        self.text.clone()
    }

    pub fn show_alert(&self) -> Option<bool> {
        self.show_alert
    }

    pub fn url(&self) -> Option<String> {
        self.url.clone()
    }

    pub fn cache_time(&self) -> Option<u32> {
        self.cache_time
    }
}

impl SetMyCommandsParams {
    pub fn new(commands: Vec<BotCommand>) -> Self {
        Self {
            commands,
            scope: None,
            language_code: None,
        }
    }

    pub fn set_commands(&mut self, commands: Vec<BotCommand>) {
        self.commands = commands;
    }

    pub fn set_scope(&mut self, scope: Option<BotCommandScope>) {
        self.scope = scope;
    }

    pub fn set_language_code(&mut self, language_code: Option<String>) {
        self.language_code = language_code;
    }

    pub fn commands(&self) -> Vec<BotCommand> {
        self.commands.clone()
    }

    pub fn scope(&self) -> Option<BotCommandScope> {
        self.scope.clone()
    }

    pub fn language_code(&self) -> Option<String> {
        self.language_code.clone()
    }
}

impl GetMyCommandsParams {
    pub fn new() -> Self {
        Self {
            scope: None,
            language_code: None,
        }
    }

    pub fn set_scope(&mut self, scope: Option<BotCommandScope>) {
        self.scope = scope;
    }

    pub fn set_language_code(&mut self, language_code: Option<String>) {
        self.language_code = language_code;
    }

    pub fn scope(&self) -> Option<BotCommandScope> {
        self.scope.clone()
    }

    pub fn language_code(&self) -> Option<String> {
        self.language_code.clone()
    }
}

impl DeleteMyCommandsParams {
    pub fn new() -> Self {
        Self {
            scope: None,
            language_code: None,
        }
    }

    pub fn set_scope(&mut self, scope: Option<BotCommandScope>) {
        self.scope = scope;
    }

    pub fn set_language_code(&mut self, language_code: Option<String>) {
        self.language_code = language_code;
    }

    pub fn scope(&self) -> Option<BotCommandScope> {
        self.scope.clone()
    }

    pub fn language_code(&self) -> Option<String> {
        self.language_code.clone()
    }
}

impl EditMessageTextParams {
    pub fn new(text: String) -> Self {
        Self {
            text,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn set_chat_id(&mut self, chat_id: Option<ChatId>) {
        self.chat_id = chat_id;
    }

    pub fn set_message_id(&mut self, message_id: Option<i32>) {
        self.message_id = message_id;
    }

    pub fn set_inline_message_id(&mut self, inline_message_id: Option<String>) {
        self.inline_message_id = inline_message_id;
    }

    pub fn set_parse_mode(&mut self, parse_mode: Option<String>) {
        self.parse_mode = parse_mode;
    }

    pub fn set_entities(&mut self, entities: Option<Vec<MessageEntity>>) {
        self.entities = entities;
    }

    pub fn set_disable_web_page_preview(&mut self, disable_web_page_preview: Option<bool>) {
        self.disable_web_page_preview = disable_web_page_preview;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<InlineKeyboardMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn chat_id(&self) -> Option<ChatId> {
        self.chat_id.clone()
    }

    pub fn message_id(&self) -> Option<i32> {
        self.message_id
    }

    pub fn inline_message_id(&self) -> Option<String> {
        self.inline_message_id.clone()
    }

    pub fn parse_mode(&self) -> Option<String> {
        self.parse_mode.clone()
    }

    pub fn entities(&self) -> Option<Vec<MessageEntity>> {
        self.entities.clone()
    }

    pub fn disable_web_page_preview(&self) -> Option<bool> {
        self.disable_web_page_preview
    }

    pub fn reply_markup(&self) -> Option<InlineKeyboardMarkup> {
        self.reply_markup.clone()
    }
}

impl EditMessageCaptionParams {
    pub fn new() -> Self {
        Self {
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: Option<ChatId>) {
        self.chat_id = chat_id;
    }

    pub fn set_message_id(&mut self, message_id: Option<i32>) {
        self.message_id = message_id;
    }

    pub fn set_inline_message_id(&mut self, inline_message_id: Option<String>) {
        self.inline_message_id = inline_message_id;
    }

    pub fn set_caption(&mut self, caption: Option<String>) {
        self.caption = caption;
    }

    pub fn set_parse_mode(&mut self, parse_mode: Option<String>) {
        self.parse_mode = parse_mode;
    }

    pub fn set_caption_entities(&mut self, caption_entities: Option<Vec<MessageEntity>>) {
        self.caption_entities = caption_entities;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<InlineKeyboardMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> Option<ChatId> {
        self.chat_id.clone()
    }

    pub fn message_id(&self) -> Option<i32> {
        self.message_id
    }

    pub fn inline_message_id(&self) -> Option<String> {
        self.inline_message_id.clone()
    }

    pub fn caption(&self) -> Option<String> {
        self.caption.clone()
    }

    pub fn parse_mode(&self) -> Option<String> {
        self.parse_mode.clone()
    }

    pub fn caption_entities(&self) -> Option<Vec<MessageEntity>> {
        self.caption_entities.clone()
    }

    pub fn reply_markup(&self) -> Option<InlineKeyboardMarkup> {
        self.reply_markup.clone()
    }
}

impl EditMessageMediaParams {
    pub fn new(media: InputMedia) -> Self {
        Self {
            media,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            reply_markup: None,
        }
    }

    pub fn set_media(&mut self, media: InputMedia) {
        self.media = media;
    }

    pub fn set_chat_id(&mut self, chat_id: Option<ChatId>) {
        self.chat_id = chat_id;
    }

    pub fn set_message_id(&mut self, message_id: Option<i32>) {
        self.message_id = message_id;
    }

    pub fn set_inline_message_id(&mut self, inline_message_id: Option<String>) {
        self.inline_message_id = inline_message_id;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<InlineKeyboardMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn media(&self) -> InputMedia {
        self.media.clone()
    }

    pub fn chat_id(&self) -> Option<ChatId> {
        self.chat_id.clone()
    }

    pub fn message_id(&self) -> Option<i32> {
        self.message_id
    }

    pub fn inline_message_id(&self) -> Option<String> {
        self.inline_message_id.clone()
    }

    pub fn reply_markup(&self) -> Option<InlineKeyboardMarkup> {
        self.reply_markup.clone()
    }
}

impl EditMessageReplyMarkupParams {
    pub fn new() -> Self {
        Self {
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: Option<ChatId>) {
        self.chat_id = chat_id;
    }

    pub fn set_message_id(&mut self, message_id: Option<i32>) {
        self.message_id = message_id;
    }

    pub fn set_inline_message_id(&mut self, inline_message_id: Option<String>) {
        self.inline_message_id = inline_message_id;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<InlineKeyboardMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> Option<ChatId> {
        self.chat_id.clone()
    }

    pub fn message_id(&self) -> Option<i32> {
        self.message_id
    }

    pub fn inline_message_id(&self) -> Option<String> {
        self.inline_message_id.clone()
    }

    pub fn reply_markup(&self) -> Option<InlineKeyboardMarkup> {
        self.reply_markup.clone()
    }
}

impl StopPollParams {
    pub fn new(chat_id: ChatId, message_id: i32) -> Self {
        Self {
            chat_id,
            message_id,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_message_id(&mut self, message_id: i32) {
        self.message_id = message_id;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<InlineKeyboardMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn message_id(&self) -> i32 {
        self.message_id
    }

    pub fn reply_markup(&self) -> Option<InlineKeyboardMarkup> {
        self.reply_markup.clone()
    }
}

impl DeleteMessageParams {
    pub fn new(chat_id: ChatId, message_id: i32) -> Self {
        Self {
            chat_id,
            message_id,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_message_id(&mut self, message_id: i32) {
        self.message_id = message_id;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn message_id(&self) -> i32 {
        self.message_id
    }
}

impl SendStickerParams {
    pub fn new(chat_id: ChatId, sticker: File) -> Self {
        Self {
            chat_id,
            sticker,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn set_sticker(&mut self, sticker: File) {
        self.sticker = sticker;
    }

    pub fn set_disable_notification(&mut self, disable_notification: Option<bool>) {
        self.disable_notification = disable_notification;
    }

    pub fn set_reply_to_message_id(&mut self, reply_to_message_id: Option<i32>) {
        self.reply_to_message_id = reply_to_message_id;
    }

    pub fn set_allow_sending_without_reply(&mut self, allow_sending_without_reply: Option<bool>) {
        self.allow_sending_without_reply = allow_sending_without_reply;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<ReplyMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn sticker(&self) -> File {
        self.sticker.clone()
    }

    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }

    pub fn reply_to_message_id(&self) -> Option<i32> {
        self.reply_to_message_id
    }

    pub fn allow_sending_without_reply(&self) -> Option<bool> {
        self.allow_sending_without_reply
    }

    pub fn reply_markup(&self) -> Option<ReplyMarkup> {
        self.reply_markup.clone()
    }
}

impl GetStickerSetParams {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl UploadStickerFileParams {
    pub fn new(user_id: u64, png_sticker: InputFile) -> Self {
        Self {
            user_id,
            png_sticker,
        }
    }

    pub fn set_user_id(&mut self, user_id: u64) {
        self.user_id = user_id;
    }

    pub fn set_png_sticker(&mut self, png_sticker: InputFile) {
        self.png_sticker = png_sticker;
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    pub fn png_sticker(&self) -> InputFile {
        self.png_sticker.clone()
    }
}

impl CreateNewStickerSetParams {
    pub fn new(user_id: u64, name: String, title: String, emojis: String) -> Self {
        Self {
            user_id,
            name,
            title,
            emojis,
            png_sticker: None,
            tgs_sticker: None,
            contains_masks: None,
            mask_position: None,
        }
    }

    pub fn set_user_id(&mut self, user_id: u64) {
        self.user_id = user_id;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_emojis(&mut self, emojis: String) {
        self.emojis = emojis;
    }

    pub fn set_png_sticker(&mut self, png_sticker: Option<File>) {
        self.png_sticker = png_sticker;
    }

    pub fn set_tgs_sticker(&mut self, tgs_sticker: Option<InputFile>) {
        self.tgs_sticker = tgs_sticker;
    }

    pub fn set_contains_masks(&mut self, contains_masks: Option<bool>) {
        self.contains_masks = contains_masks;
    }

    pub fn set_mask_position(&mut self, mask_position: Option<MaskPosition>) {
        self.mask_position = mask_position;
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn emojis(&self) -> String {
        self.emojis.clone()
    }

    pub fn png_sticker(&self) -> Option<File> {
        self.png_sticker.clone()
    }

    pub fn tgs_sticker(&self) -> Option<InputFile> {
        self.tgs_sticker.clone()
    }

    pub fn contains_masks(&self) -> Option<bool> {
        self.contains_masks
    }

    pub fn mask_position(&self) -> Option<MaskPosition> {
        self.mask_position.clone()
    }
}

impl AddStickerToSetParams {
    pub fn new(user_id: u64, name: String, emojis: String) -> Self {
        Self {
            user_id,
            name,
            emojis,
            png_sticker: None,
            tgs_sticker: None,
            mask_position: None,
        }
    }

    pub fn set_user_id(&mut self, user_id: u64) {
        self.user_id = user_id;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_emojis(&mut self, emojis: String) {
        self.emojis = emojis;
    }

    pub fn set_png_sticker(&mut self, png_sticker: Option<File>) {
        self.png_sticker = png_sticker;
    }

    pub fn set_tgs_sticker(&mut self, tgs_sticker: Option<InputFile>) {
        self.tgs_sticker = tgs_sticker;
    }

    pub fn set_mask_position(&mut self, mask_position: Option<MaskPosition>) {
        self.mask_position = mask_position;
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn emojis(&self) -> String {
        self.emojis.clone()
    }

    pub fn png_sticker(&self) -> Option<File> {
        self.png_sticker.clone()
    }

    pub fn tgs_sticker(&self) -> Option<InputFile> {
        self.tgs_sticker.clone()
    }

    pub fn mask_position(&self) -> Option<MaskPosition> {
        self.mask_position.clone()
    }
}

impl SetStickerPositionInSetParams {
    pub fn new(sticker: String, position: u32) -> Self {
        Self { sticker, position }
    }

    pub fn set_sticker(&mut self, sticker: String) {
        self.sticker = sticker;
    }

    pub fn set_position(&mut self, position: u32) {
        self.position = position;
    }

    pub fn sticker(&self) -> String {
        self.sticker.clone()
    }

    pub fn position(&self) -> u32 {
        self.position
    }
}

impl DeleteStickerFromSetParams {
    pub fn new(sticker: String) -> Self {
        Self { sticker }
    }

    pub fn set_sticker(&mut self, sticker: String) {
        self.sticker = sticker;
    }

    pub fn sticker(&self) -> String {
        self.sticker.clone()
    }
}

impl SetStickerSetThumbParams {
    pub fn new(name: String, user_id: u64) -> Self {
        Self {
            name,
            user_id,
            thumb: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_user_id(&mut self, user_id: u64) {
        self.user_id = user_id;
    }

    pub fn set_thumb(&mut self, thumb: Option<File>) {
        self.thumb = thumb;
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    pub fn thumb(&self) -> Option<File> {
        self.thumb.clone()
    }
}

impl AnswerInlineQueryParams {
    pub fn new(inline_query_id: String, results: Vec<InlineQueryResult>) -> Self {
        Self {
            inline_query_id,
            results,
            cache_time: None,
            is_personal: None,
            next_offset: None,
            switch_pm_text: None,
            switch_pm_parameter: None,
        }
    }

    pub fn set_inline_query_id(&mut self, inline_query_id: String) {
        self.inline_query_id = inline_query_id;
    }

    pub fn set_results(&mut self, results: Vec<InlineQueryResult>) {
        self.results = results;
    }

    pub fn set_cache_time(&mut self, cache_time: Option<u32>) {
        self.cache_time = cache_time;
    }

    pub fn set_is_personal(&mut self, is_personal: Option<bool>) {
        self.is_personal = is_personal;
    }

    pub fn set_next_offset(&mut self, next_offset: Option<String>) {
        self.next_offset = next_offset;
    }

    pub fn set_switch_pm_text(&mut self, switch_pm_text: Option<String>) {
        self.switch_pm_text = switch_pm_text;
    }

    pub fn set_switch_pm_parameter(&mut self, switch_pm_parameter: Option<String>) {
        self.switch_pm_parameter = switch_pm_parameter;
    }

    pub fn inline_query_id(&self) -> String {
        self.inline_query_id.clone()
    }

    pub fn results(&self) -> Vec<InlineQueryResult> {
        self.results.clone()
    }

    pub fn cache_time(&self) -> Option<u32> {
        self.cache_time
    }

    pub fn is_personal(&self) -> Option<bool> {
        self.is_personal
    }

    pub fn next_offset(&self) -> Option<String> {
        self.next_offset.clone()
    }

    pub fn switch_pm_text(&self) -> Option<String> {
        self.switch_pm_text.clone()
    }

    pub fn switch_pm_parameter(&self) -> Option<String> {
        self.switch_pm_parameter.clone()
    }
}

impl SendInvoiceParams {
    pub fn new(
        chat_id: i64,
        title: String,
        description: String,
        payload: String,
        provider_token: String,
        currency: String,
        prices: Vec<LabeledPrice>,
    ) -> Self {
        Self {
            chat_id,
            title,
            description,
            payload,
            provider_token,
            currency,
            prices,
            max_tip_amount: None,
            suggested_tip_amounts: None,
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            start_parameter: None,
            is_flexible: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: i64) {
        self.chat_id = chat_id;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set_payload(&mut self, payload: String) {
        self.payload = payload;
    }

    pub fn set_provider_token(&mut self, provider_token: String) {
        self.provider_token = provider_token;
    }

    pub fn set_start_parameter(&mut self, start_parameter: Option<String>) {
        self.start_parameter = start_parameter;
    }

    pub fn set_currency(&mut self, currency: String) {
        self.currency = currency;
    }

    pub fn set_prices(&mut self, prices: Vec<LabeledPrice>) {
        self.prices = prices;
    }

    pub fn set_provider_data(&mut self, provider_data: Option<String>) {
        self.provider_data = provider_data;
    }

    pub fn set_max_tip_amount(&mut self, max_tip_amount: Option<u32>) {
        self.max_tip_amount = max_tip_amount;
    }

    pub fn set_suggested_tip_amounts(&mut self, suggested_tip_amounts: Option<Vec<u32>>) {
        self.suggested_tip_amounts = suggested_tip_amounts;
    }

    pub fn set_photo_url(&mut self, photo_url: Option<String>) {
        self.photo_url = photo_url;
    }

    pub fn set_photo_size(&mut self, photo_size: Option<u32>) {
        self.photo_size = photo_size;
    }

    pub fn set_photo_width(&mut self, photo_width: Option<u32>) {
        self.photo_width = photo_width;
    }

    pub fn set_photo_height(&mut self, photo_height: Option<u32>) {
        self.photo_height = photo_height;
    }

    pub fn set_need_name(&mut self, need_name: Option<bool>) {
        self.need_name = need_name;
    }

    pub fn set_need_phone_number(&mut self, need_phone_number: Option<bool>) {
        self.need_phone_number = need_phone_number;
    }

    pub fn set_need_email(&mut self, need_email: Option<bool>) {
        self.need_email = need_email;
    }

    pub fn set_need_shipping_address(&mut self, need_shipping_address: Option<bool>) {
        self.need_shipping_address = need_shipping_address;
    }

    pub fn set_send_phone_number_to_provider(
        &mut self,
        send_phone_number_to_provider: Option<bool>,
    ) {
        self.send_phone_number_to_provider = send_phone_number_to_provider;
    }

    pub fn set_send_email_to_provider(&mut self, send_email_to_provider: Option<bool>) {
        self.send_email_to_provider = send_email_to_provider;
    }

    pub fn set_is_flexible(&mut self, is_flexible: Option<bool>) {
        self.is_flexible = is_flexible;
    }

    pub fn set_disable_notification(&mut self, disable_notification: Option<bool>) {
        self.disable_notification = disable_notification;
    }

    pub fn set_reply_to_message_id(&mut self, reply_to_message_id: Option<i32>) {
        self.reply_to_message_id = reply_to_message_id;
    }

    pub fn set_allow_sending_without_reply(&mut self, allow_sending_without_reply: Option<bool>) {
        self.allow_sending_without_reply = allow_sending_without_reply;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<InlineKeyboardMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn payload(&self) -> String {
        self.payload.clone()
    }

    pub fn provider_token(&self) -> String {
        self.provider_token.clone()
    }

    pub fn start_parameter(&self) -> Option<String> {
        self.start_parameter.clone()
    }

    pub fn currency(&self) -> String {
        self.currency.clone()
    }

    pub fn prices(&self) -> Vec<LabeledPrice> {
        self.prices.clone()
    }

    pub fn max_tip_amount(&self) -> Option<u32> {
        self.max_tip_amount
    }

    pub fn suggested_tip_amounts(&self) -> Option<Vec<u32>> {
        self.suggested_tip_amounts.clone()
    }

    pub fn provider_data(&self) -> Option<String> {
        self.provider_data.clone()
    }

    pub fn photo_url(&self) -> Option<String> {
        self.photo_url.clone()
    }

    pub fn photo_size(&self) -> Option<u32> {
        self.photo_size
    }

    pub fn photo_width(&self) -> Option<u32> {
        self.photo_width
    }

    pub fn photo_height(&self) -> Option<u32> {
        self.photo_height
    }

    pub fn need_name(&self) -> Option<bool> {
        self.need_name
    }

    pub fn need_phone_number(&self) -> Option<bool> {
        self.need_phone_number
    }

    pub fn need_email(&self) -> Option<bool> {
        self.need_email
    }

    pub fn need_shipping_address(&self) -> Option<bool> {
        self.need_shipping_address
    }

    pub fn send_phone_number_to_provider(&self) -> Option<bool> {
        self.send_phone_number_to_provider
    }

    pub fn send_email_to_provider(&self) -> Option<bool> {
        self.send_email_to_provider
    }

    pub fn is_flexible(&self) -> Option<bool> {
        self.is_flexible
    }

    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }

    pub fn reply_to_message_id(&self) -> Option<i32> {
        self.reply_to_message_id
    }

    pub fn allow_sending_without_reply(&self) -> Option<bool> {
        self.allow_sending_without_reply
    }

    pub fn reply_markup(&self) -> Option<InlineKeyboardMarkup> {
        self.reply_markup.clone()
    }
}

impl AnswerShippingQueryParams {
    pub fn new(shipping_query_id: String, ok: bool) -> Self {
        Self {
            shipping_query_id,
            ok,
            shipping_options: None,
            error_message: None,
        }
    }

    pub fn set_shipping_query_id(&mut self, shipping_query_id: String) {
        self.shipping_query_id = shipping_query_id;
    }

    pub fn set_ok(&mut self, ok: bool) {
        self.ok = ok;
    }

    pub fn set_shipping_options(&mut self, shipping_options: Option<Vec<ShippingOption>>) {
        self.shipping_options = shipping_options;
    }

    pub fn set_error_message(&mut self, error_message: Option<String>) {
        self.error_message = error_message;
    }

    pub fn shipping_query_id(&self) -> String {
        self.shipping_query_id.clone()
    }

    pub fn ok(&self) -> bool {
        self.ok
    }

    pub fn shipping_options(&self) -> Option<Vec<ShippingOption>> {
        self.shipping_options.clone()
    }

    pub fn error_message(&self) -> Option<String> {
        self.error_message.clone()
    }
}

impl AnswerPreCheckoutQueryParams {
    pub fn new(pre_checkout_query_id: String, ok: bool) -> Self {
        Self {
            pre_checkout_query_id,
            ok,
            error_message: None,
        }
    }

    pub fn set_pre_checkout_query_id(&mut self, pre_checkout_query_id: String) {
        self.pre_checkout_query_id = pre_checkout_query_id;
    }

    pub fn set_ok(&mut self, ok: bool) {
        self.ok = ok;
    }

    pub fn set_error_message(&mut self, error_message: Option<String>) {
        self.error_message = error_message;
    }

    pub fn pre_checkout_query_id(&self) -> String {
        self.pre_checkout_query_id.clone()
    }

    pub fn ok(&self) -> bool {
        self.ok
    }

    pub fn error_message(&self) -> Option<String> {
        self.error_message.clone()
    }
}

impl SetPassportDataErrorsParams {
    pub fn new(user_id: u64, errors: Vec<PassportElementError>) -> Self {
        Self { user_id, errors }
    }

    pub fn set_user_id(&mut self, user_id: u64) {
        self.user_id = user_id;
    }

    pub fn set_errors(&mut self, errors: Vec<PassportElementError>) {
        self.errors = errors;
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    pub fn errors(&self) -> Vec<PassportElementError> {
        self.errors.clone()
    }
}

impl SendGameParams {
    pub fn new(chat_id: i64, game_short_name: String) -> Self {
        Self {
            chat_id,
            game_short_name,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    pub fn set_chat_id(&mut self, chat_id: i64) {
        self.chat_id = chat_id;
    }

    pub fn set_game_short_name(&mut self, game_short_name: String) {
        self.game_short_name = game_short_name;
    }

    pub fn set_disable_notification(&mut self, disable_notification: Option<bool>) {
        self.disable_notification = disable_notification;
    }

    pub fn set_reply_to_message_id(&mut self, reply_to_message_id: Option<i32>) {
        self.reply_to_message_id = reply_to_message_id;
    }

    pub fn set_allow_sending_without_reply(&mut self, allow_sending_without_reply: Option<bool>) {
        self.allow_sending_without_reply = allow_sending_without_reply;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<InlineKeyboardMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn game_short_name(&self) -> String {
        self.game_short_name.clone()
    }

    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }

    pub fn reply_to_message_id(&self) -> Option<i32> {
        self.reply_to_message_id
    }

    pub fn allow_sending_without_reply(&self) -> Option<bool> {
        self.allow_sending_without_reply
    }

    pub fn reply_markup(&self) -> Option<InlineKeyboardMarkup> {
        self.reply_markup.clone()
    }
}

impl SetGameScoreParams {
    pub fn new(user_id: u64, score: i32) -> Self {
        Self {
            user_id,
            score,
            force: None,
            disable_edit_message: None,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
        }
    }

    pub fn set_user_id(&mut self, user_id: u64) {
        self.user_id = user_id;
    }

    pub fn set_score(&mut self, score: i32) {
        self.score = score;
    }

    pub fn set_force(&mut self, force: Option<bool>) {
        self.force = force;
    }

    pub fn set_disable_edit_message(&mut self, disable_edit_message: Option<bool>) {
        self.disable_edit_message = disable_edit_message;
    }

    pub fn set_chat_id(&mut self, chat_id: Option<i64>) {
        self.chat_id = chat_id;
    }

    pub fn set_message_id(&mut self, message_id: Option<i32>) {
        self.message_id = message_id;
    }

    pub fn set_inline_message_id(&mut self, inline_message_id: Option<String>) {
        self.inline_message_id = inline_message_id;
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn force(&self) -> Option<bool> {
        self.force
    }

    pub fn disable_edit_message(&self) -> Option<bool> {
        self.disable_edit_message
    }

    pub fn chat_id(&self) -> Option<i64> {
        self.chat_id
    }

    pub fn message_id(&self) -> Option<i32> {
        self.message_id
    }

    pub fn inline_message_id(&self) -> Option<String> {
        self.inline_message_id.clone()
    }
}

impl GetGameHighScoresParams {
    pub fn new(user_id: u64) -> Self {
        Self {
            user_id,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
        }
    }

    pub fn set_user_id(&mut self, user_id: u64) {
        self.user_id = user_id;
    }

    pub fn set_chat_id(&mut self, chat_id: Option<i64>) {
        self.chat_id = chat_id;
    }

    pub fn set_message_id(&mut self, message_id: Option<i32>) {
        self.message_id = message_id;
    }

    pub fn set_inline_message_id(&mut self, inline_message_id: Option<String>) {
        self.inline_message_id = inline_message_id;
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    pub fn chat_id(&self) -> Option<i64> {
        self.chat_id
    }

    pub fn message_id(&self) -> Option<i32> {
        self.message_id
    }

    pub fn inline_message_id(&self) -> Option<String> {
        self.inline_message_id.clone()
    }
}

impl InputMediaPhoto {
    pub fn new(media: File) -> Self {
        Self {
            media,
            caption: None,
            parse_mode: None,
            caption_entities: None,
        }
    }

    pub fn set_media(&mut self, media: File) {
        self.media = media;
    }

    pub fn set_caption(&mut self, caption: Option<String>) {
        self.caption = caption;
    }

    pub fn set_parse_mode(&mut self, parse_mode: Option<String>) {
        self.parse_mode = parse_mode;
    }

    pub fn set_caption_entities(&mut self, caption_entities: Option<Vec<MessageEntity>>) {
        self.caption_entities = caption_entities;
    }

    pub fn media(&self) -> File {
        self.media.clone()
    }

    pub fn caption(&self) -> Option<String> {
        self.caption.clone()
    }

    pub fn parse_mode(&self) -> Option<String> {
        self.parse_mode.clone()
    }

    pub fn caption_entities(&self) -> Option<Vec<MessageEntity>> {
        self.caption_entities.clone()
    }
}

impl InputMediaVideo {
    pub fn new(media: File) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
        }
    }

    pub fn set_media(&mut self, media: File) {
        self.media = media;
    }

    pub fn set_thumb(&mut self, thumb: Option<File>) {
        self.thumb = thumb;
    }

    pub fn set_caption(&mut self, caption: Option<String>) {
        self.caption = caption;
    }

    pub fn set_parse_mode(&mut self, parse_mode: Option<String>) {
        self.parse_mode = parse_mode;
    }

    pub fn set_caption_entities(&mut self, caption_entities: Option<Vec<MessageEntity>>) {
        self.caption_entities = caption_entities;
    }

    pub fn set_width(&mut self, width: Option<u32>) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: Option<u32>) {
        self.height = height;
    }

    pub fn set_duration(&mut self, duration: Option<u32>) {
        self.duration = duration;
    }

    pub fn set_supports_streaming(&mut self, supports_streaming: Option<bool>) {
        self.supports_streaming = supports_streaming;
    }

    pub fn media(&self) -> File {
        self.media.clone()
    }

    pub fn thumb(&self) -> Option<File> {
        self.thumb.clone()
    }

    pub fn caption(&self) -> Option<String> {
        self.caption.clone()
    }

    pub fn parse_mode(&self) -> Option<String> {
        self.parse_mode.clone()
    }

    pub fn caption_entities(&self) -> Option<Vec<MessageEntity>> {
        self.caption_entities.clone()
    }

    pub fn width(&self) -> Option<u32> {
        self.width
    }

    pub fn height(&self) -> Option<u32> {
        self.height
    }

    pub fn duration(&self) -> Option<u32> {
        self.duration
    }

    pub fn supports_streaming(&self) -> Option<bool> {
        self.supports_streaming
    }
}

impl InputMediaAnimation {
    pub fn new(media: File) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            width: None,
            height: None,
            duration: None,
        }
    }

    pub fn set_media(&mut self, media: File) {
        self.media = media;
    }

    pub fn set_thumb(&mut self, thumb: Option<File>) {
        self.thumb = thumb;
    }

    pub fn set_caption(&mut self, caption: Option<String>) {
        self.caption = caption;
    }

    pub fn set_parse_mode(&mut self, parse_mode: Option<String>) {
        self.parse_mode = parse_mode;
    }

    pub fn set_caption_entities(&mut self, caption_entities: Option<Vec<MessageEntity>>) {
        self.caption_entities = caption_entities;
    }

    pub fn set_width(&mut self, width: Option<u32>) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: Option<u32>) {
        self.height = height;
    }

    pub fn set_duration(&mut self, duration: Option<u32>) {
        self.duration = duration;
    }

    pub fn media(&self) -> File {
        self.media.clone()
    }

    pub fn thumb(&self) -> Option<File> {
        self.thumb.clone()
    }

    pub fn caption(&self) -> Option<String> {
        self.caption.clone()
    }

    pub fn parse_mode(&self) -> Option<String> {
        self.parse_mode.clone()
    }

    pub fn caption_entities(&self) -> Option<Vec<MessageEntity>> {
        self.caption_entities.clone()
    }

    pub fn width(&self) -> Option<u32> {
        self.width
    }

    pub fn height(&self) -> Option<u32> {
        self.height
    }

    pub fn duration(&self) -> Option<u32> {
        self.duration
    }
}

impl InputMediaAudio {
    pub fn new(media: File) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            duration: None,
            performer: None,
            title: None,
        }
    }

    pub fn set_media(&mut self, media: File) {
        self.media = media;
    }

    pub fn set_thumb(&mut self, thumb: Option<File>) {
        self.thumb = thumb;
    }

    pub fn set_caption(&mut self, caption: Option<String>) {
        self.caption = caption;
    }

    pub fn set_parse_mode(&mut self, parse_mode: Option<String>) {
        self.parse_mode = parse_mode;
    }

    pub fn set_caption_entities(&mut self, caption_entities: Option<Vec<MessageEntity>>) {
        self.caption_entities = caption_entities;
    }

    pub fn set_duration(&mut self, duration: Option<u32>) {
        self.duration = duration;
    }

    pub fn set_performer(&mut self, performer: Option<String>) {
        self.performer = performer;
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    pub fn media(&self) -> File {
        self.media.clone()
    }

    pub fn thumb(&self) -> Option<File> {
        self.thumb.clone()
    }

    pub fn caption(&self) -> Option<String> {
        self.caption.clone()
    }

    pub fn parse_mode(&self) -> Option<String> {
        self.parse_mode.clone()
    }

    pub fn caption_entities(&self) -> Option<Vec<MessageEntity>> {
        self.caption_entities.clone()
    }

    pub fn duration(&self) -> Option<u32> {
        self.duration
    }

    pub fn performer(&self) -> Option<String> {
        self.performer.clone()
    }

    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }
}

impl InputMediaDocument {
    pub fn new(media: File) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_content_type_detection: None,
        }
    }

    pub fn set_media(&mut self, media: File) {
        self.media = media;
    }

    pub fn set_thumb(&mut self, thumb: Option<File>) {
        self.thumb = thumb;
    }

    pub fn set_caption(&mut self, caption: Option<String>) {
        self.caption = caption;
    }

    pub fn set_parse_mode(&mut self, parse_mode: Option<String>) {
        self.parse_mode = parse_mode;
    }

    pub fn set_caption_entities(&mut self, caption_entities: Option<Vec<MessageEntity>>) {
        self.caption_entities = caption_entities;
    }

    pub fn set_disable_content_type_detection(
        &mut self,
        disable_content_type_detection: Option<bool>,
    ) {
        self.disable_content_type_detection = disable_content_type_detection;
    }

    pub fn media(&self) -> File {
        self.media.clone()
    }

    pub fn thumb(&self) -> Option<File> {
        self.thumb.clone()
    }

    pub fn caption(&self) -> Option<String> {
        self.caption.clone()
    }

    pub fn parse_mode(&self) -> Option<String> {
        self.parse_mode.clone()
    }

    pub fn caption_entities(&self) -> Option<Vec<MessageEntity>> {
        self.caption_entities.clone()
    }

    pub fn disable_content_type_detection(&self) -> Option<bool> {
        self.disable_content_type_detection
    }
}

impl InputFile {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.path = path;
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl BotCommandScopeChat {
    pub fn new(chat_id: ChatId) -> Self {
        Self { chat_id }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }
}

impl BotCommandScopeChatAdministrators {
    pub fn new(chat_id: ChatId) -> Self {
        Self { chat_id }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }
}

impl BotCommandScopeChatMember {
    pub fn new(chat_id: ChatId, user_id: u64) -> Self {
        Self { chat_id, user_id }
    }

    pub fn set_chat_id(&mut self, chat_id: ChatId) {
        self.chat_id = chat_id;
    }

    pub fn chat_id(&self) -> ChatId {
        self.chat_id.clone()
    }

    pub fn set_user_id(&mut self, user_id: u64) {
        self.user_id = user_id;
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}
