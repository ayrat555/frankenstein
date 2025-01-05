//! Parameters to Telegram API methods.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::macros::{apistruct, apply};
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
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InlineQueryResult {
    Audio(MaybeCached<InlineQueryResultCachedAudio, InlineQueryResultAudio>),
    Document(MaybeCached<InlineQueryResultCachedDocument, InlineQueryResultDocument>),
    Gif(MaybeCached<InlineQueryResultCachedGif, InlineQueryResultGif>),
    Mpeg4Gif(MaybeCached<InlineQueryResultCachedMpeg4Gif, InlineQueryResultMpeg4Gif>),
    Photo(MaybeCached<InlineQueryResultCachedPhoto, InlineQueryResultPhoto>),
    Sticker(InlineQueryResultCachedSticker),
    Video(MaybeCached<InlineQueryResultCachedVideo, InlineQueryResultVideo>),
    Voice(MaybeCached<InlineQueryResultCachedVoice, InlineQueryResultVoice>),
    Article(InlineQueryResultArticle),
    Contact(InlineQueryResultContact),
    Game(InlineQueryResultGame),
    Location(InlineQueryResultLocation),
    Venue(InlineQueryResultVenue),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum MaybeCached<T1, T2> {
    Cached(T1),
    NotCached(T2),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputMedia {
    Animation(InputMediaAnimation),
    Document(InputMediaDocument),
    Audio(InputMediaAudio),
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum PassportElementError {
    #[serde(rename = "data")]
    DataField(PassportElementErrorDataField),
    FrontSide(PassportElementErrorFrontSide),
    ReverseSide(PassportElementErrorReverseSide),
    Selfie(PassportElementErrorSelfie),
    File(PassportElementErrorFile),
    Files(PassportElementErrorFiles),
    TranslationFile(PassportElementErrorTranslationFile),
    TranslationFiles(PassportElementErrorTranslationFiles),
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
#[serde(tag = "type", rename_all = "snake_case")]
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

#[apply(apistruct!)]
#[derive(Eq)]
pub struct BotCommandScopeChat {
    pub chat_id: ChatId,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct BotCommandScopeChatAdministrators {
    pub chat_id: ChatId,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct BotCommandScopeChatMember {
    pub chat_id: ChatId,
    pub user_id: u64,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputFile {
    pub path: PathBuf,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GetUpdatesParams {
    pub offset: Option<i64>,
    pub limit: Option<u32>,
    pub timeout: Option<u32>,
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetWebhookParams {
    pub url: String,
    pub certificate: Option<InputFile>,
    pub ip_address: Option<String>,
    pub max_connections: Option<u32>,
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
    pub drop_pending_updates: Option<bool>,
    pub secret_token: Option<String>,
}

#[apply(apistruct!)]
#[derive(Copy, Eq)]
pub struct DeleteWebhookParams {
    pub drop_pending_updates: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SendMessageParams {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub text: String,
    pub parse_mode: Option<ParseMode>,
    pub entities: Option<Vec<MessageEntity>>,
    pub link_preview_options: Option<LinkPreviewOptions>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ForwardMessageParams {
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub from_chat_id: ChatId,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_id: i32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ForwardMessagesParams {
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub from_chat_id: ChatId,
    pub message_ids: Vec<i32>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct CopyMessageParams {
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub from_chat_id: ChatId,
    pub message_id: i32,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct CopyMessagesParams {
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub from_chat_id: ChatId,
    pub message_ids: Vec<i32>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub remove_caption: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SendPhotoParams {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub photo: FileUpload,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub has_spoiler: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SendAudioParams {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub audio: FileUpload,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub duration: Option<u32>,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub thumbnail: Option<FileUpload>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SendDocumentParams {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub document: FileUpload,
    pub thumbnail: Option<FileUpload>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub disable_content_type_detection: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SendVideoParams {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub video: FileUpload,
    pub duration: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub thumbnail: Option<FileUpload>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub has_spoiler: Option<bool>,
    pub supports_streaming: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SendAnimationParams {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub animation: FileUpload,
    pub duration: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub thumbnail: Option<FileUpload>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub has_spoiler: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SendVoiceParams {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub voice: FileUpload,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub duration: Option<u32>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SendVideoNoteParams {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub video_note: FileUpload,
    pub duration: Option<u32>,
    pub length: Option<u32>,
    pub thumbnail: Option<FileUpload>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SendPaidMediaParams {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub star_count: u32,
    pub media: Vec<InputPaidMedia>,
    pub payload: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SendMediaGroupParams {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub media: Vec<Media>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
}

#[apply(apistruct!)]
pub struct SendLocationParams {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub latitude: f64,
    pub longitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<u32>,
    pub heading: Option<u16>,
    pub proximity_alert_radius: Option<u32>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(apistruct!)]
pub struct EditMessageLiveLocationParams {
    pub business_connection_id: Option<String>,
    pub chat_id: Option<ChatId>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub live_period: Option<u32>,
    pub horizontal_accuracy: Option<f64>,
    pub heading: Option<u16>,
    pub proximity_alert_radius: Option<u32>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct StopMessageLiveLocationParams {
    pub business_connection_id: Option<String>,
    pub chat_id: Option<ChatId>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[apply(apistruct!)]
pub struct SendVenueParams {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SendContactParams {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SendPollParams {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub question: String,
    pub question_parse_mode: Option<ParseMode>,
    pub question_entities: Option<Vec<MessageEntity>>,
    pub options: Vec<InputPollOption>,
    pub is_anonymous: Option<bool>,
    #[serde(rename = "type")]
    pub type_field: Option<PollType>,
    pub allows_multiple_answers: Option<bool>,
    pub correct_option_id: Option<u8>,
    pub explanation: Option<String>,
    pub explanation_parse_mode: Option<ParseMode>,
    pub explanation_entities: Option<Vec<MessageEntity>>,
    pub open_period: Option<u32>,
    pub close_date: Option<u64>,
    pub is_closed: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SendDiceParams {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub emoji: Option<String>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SendChatActionParams {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub action: ChatAction,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetMessageReactionParams {
    pub chat_id: ChatId,
    pub message_id: i32,
    pub reaction: Vec<ReactionType>,
    pub is_big: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Copy, Eq)]
pub struct GetUserProfilePhotosParams {
    pub user_id: u64,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetUserEmojiStatusParams {
    pub user_id: u64,
    pub emoji_status_custom_emoji_id: Option<String>,
    pub emoji_status_expiration_date: Option<u64>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GetFileParams {
    pub file_id: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct BanChatMemberParams {
    pub chat_id: ChatId,
    pub user_id: u64,
    pub until_date: Option<u64>,
    pub revoke_messages: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct UnbanChatMemberParams {
    pub chat_id: ChatId,
    pub user_id: u64,
    pub only_if_banned: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct RestrictChatMemberParams {
    pub chat_id: ChatId,
    pub user_id: u64,
    pub permissions: ChatPermissions,
    pub use_independent_chat_permissions: Option<bool>,
    pub until_date: Option<u64>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PromoteChatMemberParams {
    pub chat_id: ChatId,
    pub user_id: u64,
    pub is_anonymous: Option<bool>,
    pub can_manage_chat: Option<bool>,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_delete_messages: Option<bool>,
    pub can_post_stories: Option<bool>,
    pub can_edit_stories: Option<bool>,
    pub can_delete_stories: Option<bool>,
    pub can_manage_video_chats: Option<bool>,
    pub can_restrict_members: Option<bool>,
    pub can_promote_members: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_manage_topics: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetChatAdministratorCustomTitleParams {
    pub chat_id: ChatId,
    pub user_id: u64,
    pub custom_title: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct BanChatSenderChatParams {
    pub chat_id: ChatId,
    pub sender_chat_id: i64,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct UnbanChatSenderChatParams {
    pub chat_id: ChatId,
    pub sender_chat_id: i64,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetChatPermissionsParams {
    pub chat_id: ChatId,
    pub permissions: ChatPermissions,
    pub use_independent_chat_permissions: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ExportChatInviteLinkParams {
    pub chat_id: ChatId,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct CreateChatInviteLinkParams {
    pub chat_id: ChatId,
    pub name: Option<String>,
    pub expire_date: Option<u64>,
    pub member_limit: Option<u32>,
    pub creates_join_request: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct EditChatInviteLinkParams {
    pub chat_id: ChatId,
    pub invite_link: String,
    pub name: Option<String>,
    pub expire_date: Option<u64>,
    pub member_limit: Option<u32>,
    pub creates_join_request: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct CreateChatSubscriptionInviteLinkParams {
    pub chat_id: ChatId,
    pub name: Option<String>,
    pub subscription_period: u32,
    pub subscription_price: u16,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct EditChatSubscriptionInviteLinkParams {
    pub chat_id: ChatId,
    pub invite_link: String,
    pub name: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct RevokeChatInviteLinkParams {
    pub chat_id: ChatId,
    pub invite_link: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ApproveChatJoinRequestParams {
    pub chat_id: ChatId,
    pub user_id: u64,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct DeclineChatJoinRequestParams {
    pub chat_id: ChatId,
    pub user_id: u64,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetChatPhotoParams {
    pub chat_id: ChatId,
    pub photo: InputFile,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct DeleteChatPhotoParams {
    pub chat_id: ChatId,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetChatTitleParams {
    pub chat_id: ChatId,
    pub title: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetChatDescriptionParams {
    pub chat_id: ChatId,
    pub description: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PinChatMessageParams {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_id: i32,
    pub disable_notification: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct UnpinChatMessageParams {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_id: Option<i32>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct UnpinAllChatMessagesParams {
    pub chat_id: ChatId,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct LeaveChatParams {
    pub chat_id: ChatId,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GetChatParams {
    pub chat_id: ChatId,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GetChatAdministratorsParams {
    pub chat_id: ChatId,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GetChatMemberCountParams {
    pub chat_id: ChatId,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GetChatMemberParams {
    pub chat_id: ChatId,
    pub user_id: u64,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetChatStickerSetParams {
    pub chat_id: ChatId,
    pub sticker_set_name: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct DeleteChatStickerSetParams {
    pub chat_id: ChatId,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct CreateForumTopicParams {
    pub chat_id: ChatId,
    pub name: String,
    pub icon_color: Option<u32>,
    pub icon_custom_emoji_id: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct EditForumTopicParams {
    pub chat_id: ChatId,
    pub message_thread_id: i32,
    pub name: Option<String>,
    pub icon_custom_emoji_id: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct CloseForumTopicParams {
    pub chat_id: ChatId,
    pub message_thread_id: i32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ReopenForumTopicParams {
    pub chat_id: ChatId,
    pub message_thread_id: i32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct DeleteForumTopicParams {
    pub chat_id: ChatId,
    pub message_thread_id: i32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct UnpinAllForumTopicMessagesParams {
    pub chat_id: ChatId,
    pub message_thread_id: i32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct EditGeneralForumTopicParams {
    pub chat_id: ChatId,
    pub name: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct CloseGeneralForumTopicParams {
    pub chat_id: ChatId,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ReopenGeneralForumTopicParams {
    pub chat_id: ChatId,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct HideGeneralForumTopicParams {
    pub chat_id: ChatId,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct UnhideGeneralForumTopicParams {
    pub chat_id: ChatId,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct AnswerCallbackQueryParams {
    pub callback_query_id: String,
    pub text: Option<String>,
    pub show_alert: Option<bool>,
    pub url: Option<String>,
    pub cache_time: Option<u32>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GetUserChatBoostsParams {
    pub chat_id: ChatId,
    pub user_id: u64,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GetBusinessConnectionParams {
    pub business_connection_id: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetMyCommandsParams {
    pub commands: Vec<BotCommand>,
    pub scope: Option<BotCommandScope>,
    pub language_code: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetMyNameParams {
    pub name: Option<String>,
    pub language_code: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GetMyNameParams {
    pub language_code: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetMyDescriptionParams {
    pub description: Option<String>,
    pub language_code: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GetMyDescriptionParams {
    pub language_code: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetMyShortDescriptionParams {
    pub short_description: Option<String>,
    pub language_code: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GetMyShortDescriptionParams {
    pub language_code: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GetMyCommandsParams {
    pub scope: Option<BotCommandScope>,
    pub language_code: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct DeleteMyCommandsParams {
    pub scope: Option<BotCommandScope>,
    pub language_code: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct EditMessageTextParams {
    pub business_connection_id: Option<String>,
    pub chat_id: Option<ChatId>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
    pub text: String,
    pub parse_mode: Option<ParseMode>,
    pub entities: Option<Vec<MessageEntity>>,
    pub link_preview_options: Option<LinkPreviewOptions>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct EditMessageCaptionParams {
    pub business_connection_id: Option<String>,
    pub chat_id: Option<ChatId>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct EditMessageMediaParams {
    pub business_connection_id: Option<String>,
    pub chat_id: Option<ChatId>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
    pub media: InputMedia,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct EditMessageReplyMarkupParams {
    pub business_connection_id: Option<String>,
    pub chat_id: Option<ChatId>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct StopPollParams {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_id: i32,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct DeleteMessageParams {
    pub chat_id: ChatId,
    pub message_id: i32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct DeleteMessagesParams {
    pub chat_id: ChatId,
    pub message_ids: Vec<i32>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SendStickerParams {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub sticker: FileUpload,
    pub emoji: Option<String>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GetStickerSetParams {
    pub name: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct UploadStickerFileParams {
    pub user_id: u64,
    pub sticker: InputFile,
    pub sticker_format: StickerFormat,
}

#[apply(apistruct!)]
pub struct CreateNewStickerSetParams {
    pub user_id: u64,
    pub name: String,
    pub title: String,
    pub stickers: Vec<InputSticker>,
    pub sticker_type: Option<StickerType>,
    pub needs_repainting: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GetCustomEmojiStickersParams {
    pub custom_emoji_ids: Vec<String>,
}

#[apply(apistruct!)]
pub struct AddStickerToSetParams {
    pub user_id: u64,
    pub name: String,
    pub sticker: InputSticker,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetStickerPositionInSetParams {
    pub sticker: String,
    pub position: u32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct DeleteStickerFromSetParams {
    pub sticker: String,
}

#[apply(apistruct!)]
pub struct ReplaceStickerInSetParams {
    pub user_id: u64,
    pub name: String,
    pub old_sticker: String,
    pub sticker: InputSticker,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetStickerEmojiListParams {
    pub sticker: String,
    pub emoji_list: Vec<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetStickerKeywordsParams {
    pub sticker: String,
    pub keywords: Option<Vec<String>>,
}

#[apply(apistruct!)]
pub struct SetStickerMaskPositionParams {
    pub sticker: String,
    pub mask_position: Option<MaskPosition>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetStickerSetTitleParams {
    pub name: String,
    pub title: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetStickerSetThumbnailParams {
    pub name: String,
    pub user_id: u64,
    pub thumbnail: Option<FileUpload>,
    pub format: StickerFormat,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetCustomEmojiStickerSetThumbnailParams {
    pub name: String,
    pub custom_emoji_id: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct DeleteStickerSetParams {
    pub name: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SendGiftParams {
    pub user_id: u64,
    pub gift_id: String,
    pub pay_for_upgrade: Option<bool>,
    pub text: Option<String>,
    pub text_parse_mode: Option<ParseMode>,
    pub text_entities: Option<Vec<MessageEntity>>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct VerifyUserParams {
    pub user_id: u64,
    pub custom_description: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct VerifyChatParams {
    pub chat_id: ChatId,
    pub custom_description: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct RemoveUserVerificationParams {
    pub user_id: u64,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct RemoveChatVerificationParams {
    pub chat_id: ChatId,
}

#[apply(apistruct!)]
pub struct AnswerInlineQueryParams {
    pub inline_query_id: String,
    pub results: Vec<InlineQueryResult>,
    pub cache_time: Option<u32>,
    pub is_personal: Option<bool>,
    pub next_offset: Option<String>,
    pub button: Option<InlineQueryResultsButton>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InlineQueryResultsButton {
    pub text: String,
    pub web_app: Option<WebAppInfo>,
    pub start_parameter: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SendInvoiceParams {
    pub chat_id: ChatId,
    pub message_thread_id: Option<i32>,
    pub title: String,
    pub description: String,
    pub payload: String,
    pub provider_token: Option<String>,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
    pub max_tip_amount: Option<u32>,
    pub suggested_tip_amounts: Option<Vec<u32>>,
    pub start_parameter: Option<String>,
    pub provider_data: Option<String>,
    pub photo_url: Option<String>,
    pub photo_size: Option<u32>,
    pub photo_width: Option<u32>,
    pub photo_height: Option<u32>,
    pub need_name: Option<bool>,
    pub need_phone_number: Option<bool>,
    pub need_email: Option<bool>,
    pub need_shipping_address: Option<bool>,
    pub send_phone_number_to_provider: Option<bool>,
    pub send_email_to_provider: Option<bool>,
    pub is_flexible: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct CreateInvoiceLinkParams {
    pub business_connection_id: Option<String>,
    pub title: String,
    pub description: String,
    pub payload: String,
    pub provider_token: Option<String>,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
    pub subscription_period: Option<u32>,
    pub max_tip_amount: Option<u32>,
    pub suggested_tip_amounts: Option<Vec<u32>>,
    pub provider_data: Option<String>,
    pub photo_url: Option<String>,
    pub photo_size: Option<u32>,
    pub photo_width: Option<u32>,
    pub photo_height: Option<u32>,
    pub need_name: Option<bool>,
    pub need_phone_number: Option<bool>,
    pub need_email: Option<bool>,
    pub need_shipping_address: Option<bool>,
    pub send_phone_number_to_provider: Option<bool>,
    pub send_email_to_provider: Option<bool>,
    pub is_flexible: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct AnswerShippingQueryParams {
    pub shipping_query_id: String,
    pub ok: bool,
    pub shipping_options: Option<Vec<ShippingOption>>,
    pub error_message: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct AnswerPreCheckoutQueryParams {
    pub pre_checkout_query_id: String,
    pub ok: bool,
    pub error_message: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GetStarTransactionsParams {
    offset: u32,

    limit: u32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct RefundStarPaymentParams {
    pub user_id: u64,
    pub telegram_payment_charge_id: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct EditUserStarSubscriptionParams {
    pub user_id: u64,
    pub telegram_payment_charge_id: String,
    pub is_canceled: bool,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetPassportDataErrorsParams {
    pub user_id: u64,
    pub errors: Vec<PassportElementError>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SendGameParams {
    pub business_connection_id: Option<String>,
    pub chat_id: i64,
    pub message_thread_id: Option<i32>,
    pub game_short_name: String,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetGameScoreParams {
    pub user_id: u64,
    pub score: i32,
    pub force: Option<bool>,
    pub disable_edit_message: Option<bool>,
    pub chat_id: Option<i64>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GetGameHighScoresParams {
    pub user_id: u64,
    pub chat_id: Option<i64>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputMediaPhoto {
    pub media: FileUpload,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub has_spoiler: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputMediaVideo {
    pub media: FileUpload,
    pub thumbnail: Option<FileUpload>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<u32>,
    pub supports_streaming: Option<bool>,
    pub has_spoiler: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputMediaAnimation {
    pub media: FileUpload,
    pub thumbnail: Option<FileUpload>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<u32>,
    pub has_spoiler: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputMediaAudio {
    pub media: FileUpload,
    pub thumbnail: Option<FileUpload>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub duration: Option<u32>,
    pub performer: Option<String>,
    pub title: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputMediaDocument {
    pub media: FileUpload,
    pub thumbnail: Option<FileUpload>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub disable_content_type_detection: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetMyDefaultAdministratorRightsParams {
    pub rights: ChatAdministratorRights,
    pub for_channels: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GetMyDefaultAdministratorRightsParams {
    pub for_channels: Option<bool>,
}

#[apply(apistruct!)]
pub struct AnswerWebAppQueryParams {
    pub web_app_query_id: String,
    pub result: InlineQueryResult,
}

#[apply(apistruct!)]
pub struct SavePreparedInlineMessageParams {
    pub user_id: u64,
    pub result: InlineQueryResult,
    pub allow_user_chats: Option<bool>,
    pub allow_bot_chats: Option<bool>,
    pub allow_group_chats: Option<bool>,
    pub allow_channel_chats: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SetChatMenuButtonParams {
    pub chat_id: Option<i64>,
    pub menu_button: Option<MenuButton>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GetChatMenuButtonParams {
    pub chat_id: Option<i64>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct UnpinAllGeneralForumTopicMessagesParams {
    pub chat_id: ChatId,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ReplyParameters {
    pub message_id: i32,
    pub chat_id: Option<ChatId>,
    pub allow_sending_without_reply: Option<bool>,
    pub quote: Option<String>,
    pub quote_parse_mode: Option<ParseMode>,
    pub quote_entities: Option<Vec<MessageEntity>>,
    pub quote_position: Option<u32>,
}
