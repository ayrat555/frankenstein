#![allow(deprecated)]
use super::api_params::File as InputFile;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder as Builder;

use crate::ParseMode;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StickerType {
    Regular,
    Mask,
    CustomEmoji,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum StickerFormat {
    Static,
    Animated,
    Video,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum InputMessageContent {
    Text(InputTextMessageContent),
    Location(InputLocationMessageContent),
    Venue(InputVenueMessageContent),
    Contact(InputContactMessageContent),
    Invoice(InputInvoiceMessageContent),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum ChatMember {
    Creator(ChatMemberOwner),
    Administrator(ChatMemberAdministrator),
    Member(ChatMemberMember),
    Restricted(ChatMemberRestricted),
    Left(ChatMemberLeft),
    Banned(ChatMemberBanned),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ChatType {
    Private,
    Group,
    Supergroup,
    Channel,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MessageEntityType {
    Mention,
    Hashtag,
    Cashtag,
    BotCommand,
    Url,
    Email,
    PhoneNumber,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Spoiler,
    Code,
    Pre,
    TextLink,
    TextMention,
    CustomEmoji,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PollType {
    Regular,
    Quiz,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EncryptedPassportElementType {
    PersonalDetails,
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
    Address,
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
    PhoneNumber,
    Email,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorDataFieldType {
    PersonalDetails,
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
    Address,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorFrontSideType {
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorReverseSideType {
    DriverLicense,
    IdentityCard,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorSelfieType {
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorFileType {
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorTranslationFileType {
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MenuButton {
    Commands,
    WebApp(MenuButtonWebApp),
    Default,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct MenuButtonWebApp {
    #[builder(setter(into))]
    pub text: String,

    pub web_app: WebAppInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatMemberOwner {
    pub user: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub custom_title: Option<String>,

    pub is_anonymous: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatMemberAdministrator {
    pub user: User,

    pub can_be_edited: bool,

    pub is_anonymous: bool,

    pub can_manage_chat: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_post_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_edit_messages: Option<bool>,

    pub can_delete_messages: bool,

    pub can_manage_video_chats: bool,

    pub can_restrict_members: bool,

    pub can_promote_members: bool,

    pub can_change_info: bool,

    pub can_invite_users: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_pin_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_manage_topics: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub custom_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatMemberMember {
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatMemberRestricted {
    #[builder(setter(into))]
    pub status: String,

    pub user: User,

    pub is_member: bool,

    pub can_send_messages: bool,

    pub can_send_audios: bool,

    pub can_send_documents: bool,

    pub can_send_photos: bool,

    pub can_send_videos: bool,

    pub can_send_video_notes: bool,

    pub can_send_voice_notes: bool,

    pub can_send_polls: bool,

    pub can_send_other_messages: bool,

    pub can_add_web_page_previews: bool,

    pub can_change_info: bool,

    pub can_invite_users: bool,

    pub can_pin_messages: bool,

    pub can_manage_topics: bool,

    pub until_date: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatMemberLeft {
    pub user: User,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatMemberBanned {
    pub user: User,

    pub until_date: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct VideoChatStarted {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct VideoChatScheduled {
    pub start_date: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct CallbackGame {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BotDescription {
    #[builder(setter(into))]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BotName {
    #[builder(setter(into))]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BotShortDescription {
    #[builder(setter(into))]
    pub short_description: String,
}

/// Represents an incoming update from telegram.
/// [Official documentation.](https://core.telegram.org/bots/api#update)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Update {
    pub update_id: u32,

    /// Maps to exactly one of the many optional fields
    /// from [the official documentation](https://core.telegram.org/bots/api#update).
    #[serde(flatten)]
    pub content: UpdateContent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateContent {
    Message(Message),
    EditedMessage(Message),
    ChannelPost(Message),
    EditedChannelPost(Message),
    InlineQuery(InlineQuery),
    ChosenInlineResult(ChosenInlineResult),
    CallbackQuery(CallbackQuery),
    ShippingQuery(ShippingQuery),
    PreCheckoutQuery(PreCheckoutQuery),
    Poll(Poll),
    PollAnswer(PollAnswer),
    MyChatMember(ChatMemberUpdated),
    ChatMember(ChatMemberUpdated),
    ChatJoinRequest(ChatJoinRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct WebhookInfo {
    #[builder(setter(into))]
    pub url: String,

    pub has_custom_certificate: bool,

    pub pending_update_count: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub ip_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub last_error_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub last_error_message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub last_synchronization_error_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub max_connections: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
}

/// Control which updates to receive.
/// Specify an empty list to receive all update types except `ChatMember`.
/// [Official documentation](https://core.telegram.org/bots/api#getupdates).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AllowedUpdate {
    Message,
    EditedMessage,
    ChannelPost,
    EditedChannelPost,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
    ShippingQuery,
    PreCheckoutQuery,
    Poll,
    PollAnswer,
    MyChatMember,
    ChatMember,
    ChatJoinRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct User {
    pub id: u64,

    pub is_bot: bool,

    #[builder(setter(into))]
    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub language_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub is_premium: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub added_to_attachment_menu: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_join_groups: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_read_all_group_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub supports_inline_queries: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Chat {
    pub id: i64,

    #[serde(rename = "type")]
    pub type_field: ChatType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub is_forum: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub photo: Option<ChatPhoto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub active_usernames: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub emoji_status_custom_emoji_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub bio: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub has_private_forwards: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub has_restricted_voice_and_video_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub join_to_send_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub join_by_request: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub invite_link: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub pinned_message: Option<Box<Message>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub permissions: Option<ChatPermissions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub slow_mode_delay: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub message_auto_delete_time: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub has_aggressive_anti_spam_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub has_hidden_members: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub has_protected_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub sticker_set_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_set_sticker_set: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub linked_chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub location: Option<ChatLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Message {
    pub message_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub message_thread_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub from: Option<Box<User>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub sender_chat: Option<Box<Chat>>,

    pub date: u64,

    #[builder(setter(into))]
    pub chat: Box<Chat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub forward_from: Option<Box<User>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub forward_from_chat: Option<Box<Chat>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub forward_from_message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub forward_signature: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub forward_sender_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub forward_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub is_topic_message: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub is_automatic_forward: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_to_message: Option<Box<Message>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub via_bot: Option<Box<User>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub edit_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub has_protected_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub media_group_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub author_signature: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub animation: Option<Box<Animation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub audio: Option<Box<Audio>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub document: Option<Box<Document>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub photo: Option<Vec<PhotoSize>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub sticker: Option<Box<Sticker>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub video: Option<Box<Video>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub video_note: Option<Box<VideoNote>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub voice: Option<Box<Voice>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub has_media_spoiler: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub contact: Option<Box<Contact>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub dice: Option<Box<Dice>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub game: Option<Box<Game>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub poll: Option<Box<Poll>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub venue: Option<Box<Venue>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub location: Option<Box<Location>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub new_chat_members: Option<Vec<User>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub left_chat_member: Option<Box<User>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub new_chat_title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub new_chat_photo: Option<Vec<PhotoSize>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub delete_chat_photo: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub group_chat_created: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub supergroup_chat_created: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub channel_chat_created: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub message_auto_delete_timer_changed: Option<Box<MessageAutoDeleteTimerChanged>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub migrate_to_chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub migrate_from_chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub pinned_message: Option<Box<Message>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub invoice: Option<Box<Invoice>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub successful_payment: Option<Box<SuccessfulPayment>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub user_shared: Option<Box<UserShared>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub chat_shared: Option<Box<ChatShared>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub connected_website: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub write_access_allowed: Option<WriteAccessAllowed>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub passport_data: Option<Box<PassportData>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub proximity_alert_triggered: Option<Box<ProximityAlertTriggered>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub forum_topic_created: Option<Box<ForumTopicCreated>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub forum_topic_edited: Option<Box<ForumTopicEdited>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub forum_topic_closed: Option<Box<ForumTopicClosed>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub forum_topic_reopened: Option<Box<ForumTopicReopened>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub general_forum_topic_hidden: Option<Box<GeneralForumTopicHidden>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub general_forum_topic_unhidden: Option<Box<GeneralForumTopicUnhidden>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub video_chat_started: Option<Box<VideoChatStarted>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub video_chat_ended: Option<Box<VideoChatEnded>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub video_chat_scheduled: Option<Box<VideoChatScheduled>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub video_chat_participants_invited: Option<Box<VideoChatParticipantsInvited>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub web_app_data: Option<Box<WebAppData>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct MessageId {
    pub message_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    pub type_field: MessageEntityType,

    pub offset: u16,

    pub length: u16,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub user: Option<User>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PhotoSize {
    #[builder(setter(into))]
    pub file_id: String,

    #[builder(setter(into))]
    pub file_unique_id: String,

    pub width: u32,

    pub height: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Animation {
    #[builder(setter(into))]
    pub file_id: String,

    #[builder(setter(into))]
    pub file_unique_id: String,

    pub width: u32,

    pub height: u32,

    pub duration: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail: Option<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub file_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Audio {
    #[builder(setter(into))]
    pub file_id: String,

    #[builder(setter(into))]
    pub file_unique_id: String,

    pub duration: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub performer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub file_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub file_size: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail: Option<PhotoSize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Document {
    #[builder(setter(into))]
    pub file_id: String,

    #[builder(setter(into))]
    pub file_unique_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail: Option<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub file_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Video {
    #[builder(setter(into))]
    pub file_id: String,

    #[builder(setter(into))]
    pub file_unique_id: String,

    pub width: u32,

    pub height: u32,

    pub duration: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail: Option<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub file_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct VideoNote {
    #[builder(setter(into))]
    pub file_id: String,

    #[builder(setter(into))]
    pub file_unique_id: String,

    pub length: u32,

    pub duration: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail: Option<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Voice {
    #[builder(setter(into))]
    pub file_id: String,

    #[builder(setter(into))]
    pub file_unique_id: String,

    pub duration: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Contact {
    #[builder(setter(into))]
    pub phone_number: String,

    #[builder(setter(into))]
    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub user_id: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub vcard: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Dice {
    #[builder(setter(into))]
    pub emoji: String,

    pub value: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PollOption {
    #[builder(setter(into))]
    pub text: String,

    pub voter_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PollAnswer {
    #[builder(setter(into))]
    pub poll_id: String,

    pub user: User,

    pub option_ids: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Poll {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub question: String,

    pub options: Vec<PollOption>,

    pub total_voter_count: u32,

    pub is_closed: bool,

    pub is_anonymous: bool,
    #[serde(rename = "type")]
    pub type_field: PollType,

    pub allows_multiple_answers: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub correct_option_id: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub explanation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub explanation_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub open_period: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub close_date: Option<u64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Builder)]
pub struct Location {
    pub longitude: f64,

    pub latitude: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub horizontal_accuracy: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub live_period: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub heading: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub proximity_alert_radius: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Venue {
    pub location: Location,

    #[builder(setter(into))]
    pub title: String,

    #[builder(setter(into))]
    pub address: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub foursquare_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub foursquare_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub google_place_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub google_place_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ProximityAlertTriggered {
    pub traveler: User,

    pub watcher: User,

    pub distance: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct MessageAutoDeleteTimerChanged {
    pub message_auto_delete_time: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ForumTopicCreated {
    #[builder(setter(into))]
    pub name: String,

    pub icon_color: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ForumTopicClosed {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ForumTopicEdited {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ForumTopicReopened {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GeneralForumTopicHidden {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GeneralForumTopicUnhidden {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct UserShared {
    pub request_id: i32,

    pub user_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatShared {
    pub request_id: i32,

    pub user_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct WriteAccessAllowed {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub web_app_name: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct VideoChatEnded {
    pub duration: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct VideoChatParticipantsInvited {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub users: Option<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct UserProfilePhotos {
    pub total_count: u32,

    pub photos: Vec<Vec<PhotoSize>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct File {
    #[builder(setter(into))]
    pub file_id: String,

    #[builder(setter(into))]
    pub file_unique_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub file_size: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub is_persistent: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub resize_keyboard: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub one_time_keyboard: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input_field_placeholder: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct KeyboardButton {
    #[builder(setter(into))]
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub request_user: Option<KeyboardButtonRequestUser>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub request_chat: Option<KeyboardButtonRequestChat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub request_contact: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub request_location: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub request_poll: Option<KeyboardButtonPollType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub web_app: Option<WebAppInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct KeyboardButtonRequestUser {
    pub request_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub user_is_bot: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub user_is_premium: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct KeyboardButtonRequestChat {
    pub request_id: i32,

    pub chat_is_channel: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub chat_is_forum: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub chat_has_username: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub chat_is_created: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub user_administrator_rights: Option<ChatAdministratorRights>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub bot_administrator_rights: Option<ChatAdministratorRights>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub bot_is_member: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct KeyboardButtonPollType {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub type_field: Option<PollType>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InlineKeyboardButton {
    #[builder(setter(into))]
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub login_url: Option<LoginUrl>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub callback_data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub web_app: Option<WebAppInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub switch_inline_query: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub switch_inline_query_current_chat: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub callback_game: Option<CallbackGame>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub pay: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct LoginUrl {
    #[builder(setter(into))]
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub forward_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub bot_username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub request_write_access: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SwitchInlineQueryChosenChat {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub query: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub allow_user_chats: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub allow_bot_chats: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub allow_group_chats: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub allow_channel_chats: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct CallbackQuery {
    #[builder(setter(into))]
    pub id: String,

    pub from: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub message: Option<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub inline_message_id: Option<String>,

    #[builder(setter(into))]
    pub chat_instance: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub game_short_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ForceReply {
    pub force_reply: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input_field_placeholder: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatPhoto {
    #[builder(setter(into))]
    pub small_file_id: String,

    #[builder(setter(into))]
    pub small_file_unique_id: String,

    #[builder(setter(into))]
    pub big_file_id: String,

    #[builder(setter(into))]
    pub big_file_unique_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatInviteLink {
    #[builder(setter(into))]
    pub invite_link: String,

    pub creator: User,

    pub creates_join_request: bool,

    pub is_primary: bool,

    pub is_revoked: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub expire_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub member_limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub pending_join_request_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ChatMemberUpdated {
    pub chat: Chat,

    pub from: User,

    pub date: u64,

    pub old_chat_member: ChatMember,

    pub new_chat_member: ChatMember,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub invite_link: Option<ChatInviteLink>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub via_chat_folder_invite_link: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ChatJoinRequest {
    pub chat: Chat,

    pub from: User,

    pub user_chat_id: u64,

    pub date: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub bio: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub invite_link: Option<ChatInviteLink>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatPermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_send_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_send_audios: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_send_documents: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_send_photos: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_send_videos: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_send_video_notes: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_send_voice_notes: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_send_polls: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_send_other_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_add_web_page_previews: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_change_info: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_invite_users: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_pin_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_manage_topics: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ChatLocation {
    pub location: Location,

    #[builder(setter(into))]
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Builder)]
pub struct ForumTopic {
    pub message_thread_id: i32,

    #[builder(setter(into))]
    pub name: String,

    pub icon_color: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BotCommand {
    #[builder(setter(into))]
    pub command: String,

    #[builder(setter(into))]
    pub description: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ResponseParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub migrate_to_chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub retry_after: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Sticker {
    #[builder(setter(into))]
    pub file_id: String,

    #[builder(setter(into))]
    pub file_unique_id: String,

    #[serde(rename = "type")]
    pub sticker_type: StickerType,

    pub width: u32,

    pub height: u32,

    pub is_animated: bool,

    pub is_video: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail: Option<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub emoji: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub set_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub premium_animation: Option<File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub mask_position: Option<MaskPosition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub custom_emoji_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub needs_repainting: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InputSticker {
    pub sticker: InputFile,
    pub emoji_list: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub mask_position: Option<MaskPosition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub keywords: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct StickerSet {
    #[builder(setter(into))]
    pub name: String,

    #[builder(setter(into))]
    pub title: String,

    #[serde(rename = "sticker_type")]
    pub sticker_type: StickerType,

    pub is_animated: bool,

    pub is_video: bool,

    #[doc(hidden)]
    #[deprecated(since = "0.19.2", note = "Please use `sticker_type` instead")]
    pub contains_masks: bool,

    pub stickers: Vec<Sticker>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail: Option<PhotoSize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct MaskPosition {
    #[builder(setter(into))]
    pub point: String,

    pub x_shift: f64,

    pub y_shift: f64,

    pub scale: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQuery {
    #[builder(setter(into))]
    pub id: String,

    pub from: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub location: Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub chat_type: Option<String>,

    #[builder(setter(into))]
    pub query: String,

    #[builder(setter(into))]
    pub offset: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultArticle {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub title: String,

    pub input_message_content: InputMessageContent,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub hide_url: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultPhoto {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub photo_url: String,

    #[builder(setter(into))]
    pub thumbnail_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub photo_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub photo_height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultGif {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub gif_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub gif_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub gif_height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub gif_duration: Option<u32>,

    #[builder(setter(into))]
    pub thumbnail_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail_mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultMpeg4Gif {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub mpeg4_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub mpeg4_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub mpeg4_height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub mpeg4_duration: Option<u32>,

    #[builder(setter(into))]
    pub thumbnail_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail_mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultVideo {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub video_url: String,

    #[builder(setter(into))]
    pub mime_type: String,

    #[builder(setter(into))]
    pub thumbnail_url: String,

    #[builder(setter(into))]
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub video_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub video_height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub video_duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultAudio {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub audio_url: String,

    #[builder(setter(into))]
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub performer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub audio_duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultVoice {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub voice_url: String,

    #[builder(setter(into))]
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub voice_duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultDocument {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[builder(setter(into))]
    pub document_url: String,

    #[builder(setter(into))]
    pub mime_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input_message_content: Option<InputMessageContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultLocation {
    #[builder(setter(into))]
    pub id: String,

    pub latitude: f64,

    pub longitude: f64,

    #[builder(setter(into))]
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub horizontal_accuracy: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub live_period: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub heading: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub proximity_alert_radius: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input_message_content: Option<InputMessageContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultVenue {
    #[builder(setter(into))]
    pub id: String,

    pub latitude: f64,

    pub longitude: f64,

    #[builder(setter(into))]
    pub title: String,

    #[builder(setter(into))]
    pub address: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub foursquare_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub foursquare_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub google_place_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub google_place_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input_message_content: Option<InputMessageContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultContact {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub phone_number: String,

    #[builder(setter(into))]
    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub vcard: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input_message_content: Option<InputMessageContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub thumbnail_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InlineQueryResultGame {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub game_short_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedPhoto {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub photo_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedGif {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub gif_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedMpeg4Gif {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub mpeg4_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedSticker {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub sticker_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedDocument {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub title: String,

    #[builder(setter(into))]
    pub document_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedVideo {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub video_file_id: String,

    #[builder(setter(into))]
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedVoice {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub voice_file_id: String,

    #[builder(setter(into))]
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedAudio {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub audio_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InputTextMessageContent {
    #[builder(setter(into))]
    pub message_text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub disable_web_page_preview: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Builder)]
pub struct InputLocationMessageContent {
    pub latitude: f64,

    pub longitude: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub horizontal_accuracy: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub live_period: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub heading: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub proximity_alert_radius: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InputInvoiceMessageContent {
    #[builder(setter(into))]
    pub title: String,

    #[builder(setter(into))]
    pub description: String,

    #[builder(setter(into))]
    pub payload: String,

    #[builder(setter(into))]
    pub provider_token: String,

    #[builder(setter(into))]
    pub currency: String,

    pub prices: Vec<LabeledPrice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub max_tip_amount: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub suggested_tip_amounts: Option<Vec<u32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub provider_data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub photo_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub photo_size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub photo_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub photo_height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub need_name: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub need_phone_number: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub need_email: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub need_shipping_address: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub send_phone_number_to_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub send_email_to_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub is_flexible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InputVenueMessageContent {
    pub latitude: f64,

    pub longitude: f64,

    #[builder(setter(into))]
    pub title: String,

    #[builder(setter(into))]
    pub address: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub foursquare_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub foursquare_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub google_place_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub google_place_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InputContactMessageContent {
    #[builder(setter(into))]
    pub phone_number: String,

    #[builder(setter(into))]
    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub vcard: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ChosenInlineResult {
    #[builder(setter(into))]
    pub result_id: String,

    pub from: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub location: Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub inline_message_id: Option<String>,

    #[builder(setter(into))]
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct LabeledPrice {
    #[builder(setter(into))]
    pub label: String,

    pub amount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Invoice {
    #[builder(setter(into))]
    pub title: String,

    #[builder(setter(into))]
    pub description: String,

    #[builder(setter(into))]
    pub start_parameter: String,

    #[builder(setter(into))]
    pub currency: String,

    pub total_amount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ShippingAddress {
    #[builder(setter(into))]
    pub country_code: String,

    #[builder(setter(into))]
    pub state: String,

    #[builder(setter(into))]
    pub city: String,

    #[builder(setter(into))]
    pub street_line1: String,

    #[builder(setter(into))]
    pub street_line2: String,

    #[builder(setter(into))]
    pub post_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct OrderInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub phone_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub shipping_address: Option<ShippingAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ShippingOption {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub title: String,

    pub prices: Vec<LabeledPrice>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SuccessfulPayment {
    #[builder(setter(into))]
    pub currency: String,

    pub total_amount: u32,

    #[builder(setter(into))]
    pub invoice_payload: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub shipping_option_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub order_info: Option<OrderInfo>,

    #[builder(setter(into))]
    pub telegram_payment_charge_id: String,

    #[builder(setter(into))]
    pub provider_payment_charge_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ShippingQuery {
    #[builder(setter(into))]
    pub id: String,

    pub from: User,

    #[builder(setter(into))]
    pub invoice_payload: String,

    pub shipping_address: ShippingAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PreCheckoutQuery {
    #[builder(setter(into))]
    pub id: String,

    pub from: User,

    #[builder(setter(into))]
    pub currency: String,

    pub total_amount: u32,

    #[builder(setter(into))]
    pub invoice_payload: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub shipping_option_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub order_info: Option<OrderInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,

    pub credentials: EncryptedCredentials,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportFile {
    #[builder(setter(into))]
    pub file_id: String,

    #[builder(setter(into))]
    pub file_unique_id: String,

    pub file_size: u64,

    pub file_date: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct EncryptedPassportElement {
    #[serde(rename = "type")]
    pub type_field: EncryptedPassportElementType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub phone_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub files: Option<Vec<PassportFile>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub front_side: Option<PassportFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub reverse_side: Option<PassportFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub selfie: Option<PassportFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub translation: Option<Vec<PassportFile>>,

    #[builder(setter(into))]
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct EncryptedCredentials {
    #[builder(setter(into))]
    pub data: String,

    #[builder(setter(into))]
    pub hash: String,

    #[builder(setter(into))]
    pub secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportElementErrorDataField {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorDataFieldType,

    #[builder(setter(into))]
    pub field_name: String,

    #[builder(setter(into))]
    pub data_hash: String,

    #[builder(setter(into))]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportElementErrorFrontSide {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorFrontSideType,

    #[builder(setter(into))]
    pub file_hash: String,

    #[builder(setter(into))]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportElementErrorReverseSide {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorReverseSideType,

    #[builder(setter(into))]
    pub file_hash: String,

    #[builder(setter(into))]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportElementErrorSelfie {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorSelfieType,

    #[builder(setter(into))]
    pub file_hash: String,

    #[builder(setter(into))]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportElementErrorFile {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorFileType,

    #[builder(setter(into))]
    pub file_hash: String,

    #[builder(setter(into))]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportElementErrorFiles {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorFileType,

    pub file_hashes: Vec<String>,

    #[builder(setter(into))]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportElementErrorTranslationFile {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorTranslationFileType,

    #[builder(setter(into))]
    pub file_hash: String,

    #[builder(setter(into))]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportElementErrorTranslationFiles {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorTranslationFileType,

    pub file_hashes: Vec<String>,

    #[builder(setter(into))]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportElementErrorUnspecified {
    #[serde(rename = "type")]
    pub type_field: EncryptedPassportElementType,

    #[builder(setter(into))]
    pub element_hash: String,

    #[builder(setter(into))]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Game {
    #[builder(setter(into))]
    pub title: String,

    #[builder(setter(into))]
    pub description: String,

    pub photo: Vec<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub text_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub animation: Option<Animation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GameHighScore {
    pub position: u32,

    pub user: User,

    pub score: i32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatAdministratorRights {
    pub is_anonymous: bool,

    pub can_manage_chat: bool,

    pub can_delete_messages: bool,

    pub can_manage_video_chats: bool,

    pub can_restrict_members: bool,

    pub can_promote_members: bool,

    pub can_change_info: bool,

    pub can_invite_users: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_post_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_edit_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_pin_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub can_manage_topics: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct WebAppInfo {
    #[builder(setter(into))]
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SentWebAppMessage {
    #[builder(setter(into))]
    pub inline_message_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct WebAppData {
    #[builder(setter(into))]
    pub data: String,

    #[builder(setter(into))]
    pub button_text: String,
}

#[cfg(test)]
mod serde_tests {
    use super::*;

    #[test]
    pub fn update_content_is_flattened() {
        let update_content = r#"{
            "update_id": 2341,
            "message": {
                "message_id": 2746,
                "from": {
                    "id": 1276618370,
                    "is_bot": true,
                    "first_name": "test_el_bot",
                    "username": "el_mon_test_bot"
                },
                "date": 1618207352,
                "chat": {
                    "id": 275808073,
                    "type": "private",
                    "username": "Ayrat555",
                    "first_name": "Ayrat",
                    "last_name": "Badykov"
                },
                "text": "Hello!"
            }
        }"#;

        let update: Update = serde_json::from_str(update_content).unwrap();

        let message = Message::builder()
            .message_id(2746)
            .from(
                User::builder()
                    .id(1276618370)
                    .is_bot(true)
                    .first_name("test_el_bot")
                    .username("el_mon_test_bot")
                    .build(),
            )
            .date(1618207352)
            .chat(
                Chat::builder()
                    .id(275808073)
                    .type_field(ChatType::Private)
                    .username("Ayrat555")
                    .first_name("Ayrat")
                    .last_name("Badykov")
                    .build(),
            )
            .text("Hello!")
            .build();

        let expected = Update {
            update_id: 2341,
            content: UpdateContent::Message(message),
        };

        assert_eq!(update, expected);
    }
}
