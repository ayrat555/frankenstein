#![allow(deprecated)]
use super::api_params::FileUpload;
use bon::Builder;
use serde::{Deserialize, Serialize};

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
    Kicked(ChatMemberBanned),
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
    Blockquote,
    ExpandableBlockquote,
    #[serde(other)]
    Unknown,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChatBackground {
    Fill(BackgroundTypeFill),
    Wallpaper(BackgroundTypeWallpaper),
    Patter(BackgroundTypePattern),
    ChatTheme(BackgroundTypeChatTheme),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BackgroundFill {
    Solid(BackgroundFillSolid),
    Gradient(BackgroundFillGradient),
    FreeformGradient(BackgroundFillFreeformGradient),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct MenuButtonWebApp {
    #[builder(into)]
    pub text: String,

    pub web_app: WebAppInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatMemberOwner {
    pub user: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub custom_title: Option<String>,

    pub is_anonymous: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatMemberAdministrator {
    pub user: User,

    pub can_be_edited: bool,

    pub is_anonymous: bool,

    pub can_manage_chat: bool,

    pub can_delete_messages: bool,

    pub can_manage_video_chats: bool,

    pub can_restrict_members: bool,

    pub can_promote_members: bool,

    pub can_change_info: bool,

    pub can_invite_users: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_stories: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_stories: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_stories: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub custom_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatMemberMember {
    pub user: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatMemberRestricted {
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
    #[builder(into)]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BotName {
    #[builder(into)]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BotShortDescription {
    #[builder(into)]
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
    BusinessConnection(BusinessConnection),
    BusinessMessage(Message),
    EditedBusinessMessage(Message),
    DeletedBusinessMessages(BusinessMessagesDeleted),
    MessageReaction(MessageReactionUpdated),
    MessageReactionCount(MessageReactionCountUpdated),
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
    ChatBoost(ChatBoostUpdated),
    RemovedChatBoost(ChatBoostRemoved),
    PurchasedPaidMedia(PaidMediaPurchased),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct WebhookInfo {
    #[builder(into)]
    pub url: String,

    pub has_custom_certificate: bool,

    pub pending_update_count: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub ip_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub last_error_message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_synchronization_error_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
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
    MessageReaction,
    MessageReactionCount,
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
    ChatBoost,
    RemovedChatBoost,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct User {
    pub id: u64,

    pub is_bot: bool,

    #[builder(into)]
    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub language_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_to_attachment_menu: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_join_groups: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_all_group_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_inline_queries: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_connect_to_business: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_main_web_app: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Chat {
    pub id: i64,

    #[serde(rename = "type")]
    pub type_field: ChatType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ChatFullInfo {
    pub id: i64,

    #[serde(rename = "type")]
    pub type_field: ChatType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_usernames: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<Birthdate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_intro: Option<BusinessIntro>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_location: Option<BusinessLocation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_opening_hours: Option<BusinessOpeningHours>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub personal_chat: Option<Box<Chat>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_reactions: Option<Vec<ReactionType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color_id: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_reaction_count: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub background_custom_emoji_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_accent_color_id: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub profile_background_custom_emoji_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub emoji_status_custom_emoji_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_expiration_date: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub bio: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_private_forwards: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_restricted_voice_and_video_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_to_send_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_by_request: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub invite_link: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub pinned_message: Option<Box<Message>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_paid_media: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unrestrict_boost_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_aggressive_anti_spam_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_hidden_members: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_visible_history: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub sticker_set_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub custom_emoji_sticker_set_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ChatLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Message {
    pub message_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub from: Option<Box<User>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub sender_chat: Option<Box<Chat>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_boost_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub sender_business_bot: Option<Box<User>>,

    pub date: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub business_connection_id: Option<String>,

    #[builder(into)]
    pub chat: Box<Chat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub forward_origin: Option<Box<MessageOrigin>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_topic_message: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_forward: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub reply_to_message: Option<Box<Message>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub external_reply: Option<Box<ExternalReplyInfo>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub quote: Option<Box<TextQuote>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub reply_to_story: Option<Box<Story>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub via_bot: Option<Box<User>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_from_offline: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub media_group_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub author_signature: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub effect_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub animation: Option<Box<Animation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub audio: Option<Box<Audio>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub document: Option<Box<Document>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub paid_media: Option<Box<PaidMediaInfo>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub sticker: Option<Box<Sticker>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub story: Option<Box<Story>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub video: Option<Box<Video>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub video_note: Option<Box<VideoNote>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub voice: Option<Box<Voice>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub contact: Option<Box<Contact>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub dice: Option<Box<Dice>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub game: Option<Box<Game>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub poll: Option<Box<Poll>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub venue: Option<Box<Venue>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub location: Option<Box<Location>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_members: Option<Vec<User>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub left_chat_member: Option<Box<User>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub new_chat_title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_photo: Option<Vec<PhotoSize>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_chat_photo: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_chat_created: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supergroup_chat_created: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_chat_created: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_auto_delete_timer_changed: Option<Box<MessageAutoDeleteTimerChanged>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_from_chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub pinned_message: Option<Box<MaybeInaccessibleMessage>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub invoice: Option<Box<Invoice>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub successful_payment: Option<Box<SuccessfulPayment>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub refunded_payment: Option<Box<RefundedPayment>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub users_shared: Option<Box<UsersShared>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub chat_shared: Option<Box<ChatShared>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub connected_website: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_access_allowed: Option<WriteAccessAllowed>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub passport_data: Option<Box<PassportData>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub proximity_alert_triggered: Option<Box<ProximityAlertTriggered>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub boost_added: Option<Box<ChatBoostAdded>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub chat_background_set: Option<Box<ChatBackground>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub forum_topic_created: Option<Box<ForumTopicCreated>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub forum_topic_edited: Option<Box<ForumTopicEdited>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub forum_topic_closed: Option<Box<ForumTopicClosed>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub forum_topic_reopened: Option<Box<ForumTopicReopened>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub general_forum_topic_hidden: Option<Box<GeneralForumTopicHidden>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub general_forum_topic_unhidden: Option<Box<GeneralForumTopicUnhidden>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_created: Option<GiveawayCreated>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway: Option<Giveaway>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_winners: Option<GiveawayWinners>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_completed: Option<GiveawayCompleted>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub video_chat_started: Option<Box<VideoChatStarted>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub video_chat_ended: Option<Box<VideoChatEnded>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub video_chat_scheduled: Option<Box<VideoChatScheduled>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub video_chat_participants_invited: Option<Box<VideoChatParticipantsInvited>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub web_app_data: Option<Box<WebAppData>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
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
    #[builder(into)]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub custom_emoji_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct TextQuote {
    #[builder(into)]
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    pub position: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_manual: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Builder)]
pub struct ExternalReplyInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<MessageOrigin>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<Chat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<PaidMediaInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<Story>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Game>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway: Option<Giveaway>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_winners: Option<GiveawayWinners>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageOrigin {
    User(MessageOriginUser),
    HiddenUser(MessageOriginHiddenUser),
    Chat(MessageOriginChat),
    Channel(MessageOriginChannel),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct MessageOriginUser {
    pub date: u64,
    pub sender_user: User,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct MessageOriginHiddenUser {
    pub date: u64,
    #[builder(into)]
    pub sender_user_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct MessageOriginChat {
    pub date: u64,

    pub sender_chat: Chat,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub author_signature: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct MessageOriginChannel {
    pub date: u64,

    pub chat: Chat,

    pub message_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub author_signature: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct LinkPreviewOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_small_media: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_large_media: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_above_text: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PhotoSize {
    #[builder(into)]
    pub file_id: String,

    #[builder(into)]
    pub file_unique_id: String,

    pub width: u32,

    pub height: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Animation {
    #[builder(into)]
    pub file_id: String,

    #[builder(into)]
    pub file_unique_id: String,

    pub width: u32,

    pub height: u32,

    pub duration: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub file_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Audio {
    #[builder(into)]
    pub file_id: String,

    #[builder(into)]
    pub file_unique_id: String,

    pub duration: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub performer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub file_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Document {
    #[builder(into)]
    pub file_id: String,

    #[builder(into)]
    pub file_unique_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub file_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Video {
    #[builder(into)]
    pub file_id: String,

    #[builder(into)]
    pub file_unique_id: String,

    pub width: u32,

    pub height: u32,

    pub duration: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub file_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct VideoNote {
    #[builder(into)]
    pub file_id: String,

    #[builder(into)]
    pub file_unique_id: String,

    pub length: u32,

    pub duration: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Voice {
    #[builder(into)]
    pub file_id: String,

    #[builder(into)]
    pub file_unique_id: String,

    pub duration: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Contact {
    #[builder(into)]
    pub phone_number: String,

    #[builder(into)]
    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub vcard: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Dice {
    #[builder(into)]
    pub emoji: String,

    pub value: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PollOption {
    #[builder(into)]
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,

    pub voter_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InputPollOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PollAnswer {
    #[builder(into)]
    pub poll_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voter_chat: Option<Chat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub user: Option<Box<User>>,

    pub option_ids: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Poll {
    #[builder(into)]
    pub id: String,

    #[builder(into)]
    pub question: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_entities: Option<Vec<MessageEntity>>,

    pub options: Vec<PollOption>,

    pub total_voter_count: u32,

    pub is_closed: bool,

    pub is_anonymous: bool,
    #[serde(rename = "type")]
    pub type_field: PollType,

    pub allows_multiple_answers: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub explanation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<u64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Builder)]
pub struct Location {
    pub longitude: f64,

    pub latitude: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Venue {
    pub location: Location,

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatBoostAdded {
    pub boost_count: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BackgroundFillSolid {
    pub color: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BackgroundFillGradient {
    pub top_color: u32,

    pub bottom_color: u32,

    pub rotation_angle: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BackgroundFillFreeformGradient {
    pub colors: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BackgroundTypeFill {
    pub fill: BackgroundFill,

    pub dark_theme_dimming: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BackgroundTypeWallpaper {
    pub document: Document,

    pub dark_theme_dimming: u8,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_blurred: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_moving: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BackgroundTypePattern {
    pub document: Document,

    pub fill: BackgroundFill,

    pub intensity: u8,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_inverted: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_moving: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BackgroundTypeChatTheme {
    #[builder(into)]
    pub theme_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ForumTopicCreated {
    #[builder(into)]
    pub name: String,

    pub icon_color: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ForumTopicClosed {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ForumTopicEdited {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ForumTopicReopened {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GeneralForumTopicHidden {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GeneralForumTopicUnhidden {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SharedUser {
    pub user_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct UsersShared {
    pub request_id: i32,

    pub users: Vec<SharedUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatShared {
    pub request_id: i32,

    pub chat_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct WriteAccessAllowed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_request: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub web_app_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_attachment_menu: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct VideoChatEnded {
    pub duration: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct VideoChatParticipantsInvited {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct UserProfilePhotos {
    pub total_count: u32,

    pub photos: Vec<Vec<PhotoSize>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct File {
    #[builder(into)]
    pub file_id: String,

    #[builder(into)]
    pub file_unique_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_persistent: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_keyboard: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_keyboard: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub input_field_placeholder: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct KeyboardButton {
    #[builder(into)]
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_users: Option<KeyboardButtonRequestUsers>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_chat: Option<KeyboardButtonRequestChat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<KeyboardButtonPollType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct KeyboardButtonRequestUsers {
    pub request_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_bot: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_premium: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_quantity: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_name: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct KeyboardButtonRequestChat {
    pub request_id: i32,

    pub chat_is_channel: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_forum: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_has_username: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_created: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_administrator_rights: Option<ChatAdministratorRights>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_administrator_rights: Option<ChatAdministratorRights>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_is_member: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_title: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct KeyboardButtonPollType {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<PollType>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InlineKeyboardButton {
    #[builder(into)]
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<LoginUrl>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub callback_data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub switch_inline_query: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub switch_inline_query_current_chat: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<CallbackGame>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct LoginUrl {
    #[builder(into)]
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub forward_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub bot_username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_write_access: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SwitchInlineQueryChosenChat {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub query: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_chats: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_bot_chats: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_group_chats: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_channel_chats: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct CallbackQuery {
    #[builder(into)]
    pub id: String,

    pub from: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<MaybeInaccessibleMessage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub inline_message_id: Option<String>,

    #[builder(into)]
    pub chat_instance: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub game_short_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ForceReply {
    pub force_reply: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub input_field_placeholder: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatPhoto {
    #[builder(into)]
    pub small_file_id: String,

    #[builder(into)]
    pub small_file_unique_id: String,

    #[builder(into)]
    pub big_file_id: String,

    #[builder(into)]
    pub big_file_unique_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatInviteLink {
    #[builder(into)]
    pub invite_link: String,

    pub creator: User,

    pub creates_join_request: bool,

    pub is_primary: bool,

    pub is_revoked: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_join_request_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatMemberUpdated {
    pub chat: Chat,

    pub from: User,

    pub date: u64,

    pub old_chat_member: ChatMember,

    pub new_chat_member: ChatMember,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_join_request: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_chat_folder_invite_link: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatJoinRequest {
    pub chat: Chat,

    pub from: User,

    pub user_chat_id: u64,

    pub date: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub bio: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatPermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_audios: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_documents: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_photos: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_videos: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_video_notes: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_voice_notes: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,

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
pub struct Birthdate {
    pub day: u8,

    pub month: u8,

    pub year: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct BusinessIntro {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct BusinessLocation {
    #[builder(into)]
    pub address: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BusinessOpeningHoursInterval {
    pub opening_minute: u16,

    pub closing_minute: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BusinessOpeningHours {
    pub time_zone_name: String,

    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ChatLocation {
    pub location: Location,

    #[builder(into)]
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ReactionType {
    Emoji(ReactionTypeEmoji),
    CustomEmoji(ReactionTypeCustomEmoji),
    Paid(ReactionTypePaid),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ReactionTypeEmoji {
    #[builder(into)]
    pub emoji: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ReactionTypeCustomEmoji {
    #[builder(into)]
    pub custom_emoji_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ReactionTypePaid {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ReactionCount {
    #[serde(rename = "type")]
    pub type_field: ReactionType,

    pub total_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct MessageReactionUpdated {
    pub chat: Chat,

    pub message_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_chat: Option<Chat>,

    pub date: u64,

    pub old_reaction: Vec<ReactionType>,

    pub new_reaction: Vec<ReactionType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct MessageReactionCountUpdated {
    pub chat: Chat,

    pub message_id: i32,

    pub date: u64,

    pub reactions: Vec<ReactionCount>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Builder)]
pub struct ForumTopic {
    pub message_thread_id: i32,

    #[builder(into)]
    pub name: String,

    pub icon_color: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BotCommand {
    #[builder(into)]
    pub command: String,

    #[builder(into)]
    pub description: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ResponseParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Sticker {
    #[builder(into)]
    pub file_id: String,

    #[builder(into)]
    pub file_unique_id: String,

    #[serde(rename = "type")]
    pub sticker_type: StickerType,

    pub width: u32,

    pub height: u32,

    pub is_animated: bool,

    pub is_video: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub emoji: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub set_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_animation: Option<File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub custom_emoji_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InputSticker {
    pub sticker: FileUpload,
    pub format: StickerFormat,
    pub emoji_list: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Story {
    pub chat: Chat,
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct StickerSet {
    #[builder(into)]
    pub name: String,

    #[builder(into)]
    pub title: String,

    #[serde(rename = "sticker_type")]
    pub sticker_type: StickerType,

    #[doc(hidden)]
    #[deprecated(since = "0.19.2", note = "Please use `sticker_type` instead")]
    pub contains_masks: bool,

    pub stickers: Vec<Sticker>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct MaskPosition {
    #[builder(into)]
    pub point: String,

    pub x_shift: f64,

    pub y_shift: f64,

    pub scale: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQuery {
    #[builder(into)]
    pub id: String,

    pub from: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub chat_type: Option<String>,

    #[builder(into)]
    pub query: String,

    #[builder(into)]
    pub offset: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultArticle {
    #[builder(into)]
    pub id: String,

    #[builder(into)]
    pub title: String,

    pub input_message_content: InputMessageContent,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_url: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub thumbnail_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultPhoto {
    #[builder(into)]
    pub id: String,

    #[builder(into)]
    pub photo_url: String,

    #[builder(into)]
    pub thumbnail_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub description: Option<String>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultGif {
    #[builder(into)]
    pub id: String,

    #[builder(into)]
    pub gif_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_duration: Option<u32>,

    #[builder(into)]
    pub thumbnail_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub thumbnail_mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub title: Option<String>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultMpeg4Gif {
    #[builder(into)]
    pub id: String,

    #[builder(into)]
    pub mpeg4_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_duration: Option<u32>,

    #[builder(into)]
    pub thumbnail_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub thumbnail_mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub title: Option<String>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultVideo {
    #[builder(into)]
    pub id: String,

    #[builder(into)]
    pub video_url: String,

    #[builder(into)]
    pub mime_type: String,

    #[builder(into)]
    pub thumbnail_url: String,

    #[builder(into)]
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultAudio {
    #[builder(into)]
    pub id: String,

    #[builder(into)]
    pub audio_url: String,

    #[builder(into)]
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub performer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultVoice {
    #[builder(into)]
    pub id: String,

    #[builder(into)]
    pub voice_url: String,

    #[builder(into)]
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultDocument {
    #[builder(into)]
    pub id: String,

    #[builder(into)]
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[builder(into)]
    pub document_url: String,

    #[builder(into)]
    pub mime_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub thumbnail_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultLocation {
    #[builder(into)]
    pub id: String,

    pub latitude: f64,

    pub longitude: f64,

    #[builder(into)]
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub thumbnail_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultVenue {
    #[builder(into)]
    pub id: String,

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
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub thumbnail_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultContact {
    #[builder(into)]
    pub id: String,

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
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub thumbnail_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InlineQueryResultGame {
    #[builder(into)]
    pub id: String,

    #[builder(into)]
    pub game_short_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedPhoto {
    #[builder(into)]
    pub id: String,

    #[builder(into)]
    pub photo_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub description: Option<String>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedGif {
    #[builder(into)]
    pub id: String,

    #[builder(into)]
    pub gif_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub title: Option<String>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedMpeg4Gif {
    #[builder(into)]
    pub id: String,

    #[builder(into)]
    pub mpeg4_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub title: Option<String>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedSticker {
    #[builder(into)]
    pub id: String,

    #[builder(into)]
    pub sticker_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedDocument {
    #[builder(into)]
    pub id: String,

    #[builder(into)]
    pub title: String,

    #[builder(into)]
    pub document_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedVideo {
    #[builder(into)]
    pub id: String,

    #[builder(into)]
    pub video_file_id: String,

    #[builder(into)]
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub description: Option<String>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedVoice {
    #[builder(into)]
    pub id: String,

    #[builder(into)]
    pub voice_file_id: String,

    #[builder(into)]
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedAudio {
    #[builder(into)]
    pub id: String,

    #[builder(into)]
    pub audio_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InputTextMessageContent {
    #[builder(into)]
    pub message_text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Builder)]
pub struct InputLocationMessageContent {
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InputInvoiceMessageContent {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InputVenueMessageContent {
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InputContactMessageContent {
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ChosenInlineResult {
    #[builder(into)]
    pub result_id: String,

    pub from: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub inline_message_id: Option<String>,

    #[builder(into)]
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct LabeledPrice {
    #[builder(into)]
    pub label: String,

    pub amount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Invoice {
    #[builder(into)]
    pub title: String,

    #[builder(into)]
    pub description: String,

    #[builder(into)]
    pub start_parameter: String,

    #[builder(into)]
    pub currency: String,

    pub total_amount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PaidMediaInfo {
    pub star_count: u32,

    pub paid_media: Vec<PaidMedia>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PaidMedia {
    Preview(PaidMediaPreview),
    Photo(PaidMediaPhoto),
    Video(PaidMediaVideo),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PaidMediaPreview {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaidMediaPhoto {
    pub photo: Vec<PhotoSize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaidMediaVideo {
    pub video: Video,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputPaidMedia {
    Photo(InputPaidMediaPhoto),
    Video(InputPaidMediaVideo),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InputPaidMediaPhoto {
    #[builder(into)]
    pub media: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InputPaidMediaVideo {
    #[builder(into)]
    pub media: String,

    #[builder(into)]
    pub thumbnail: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ShippingAddress {
    #[builder(into)]
    pub country_code: String,

    #[builder(into)]
    pub state: String,

    #[builder(into)]
    pub city: String,

    #[builder(into)]
    pub street_line1: String,

    #[builder(into)]
    pub street_line2: String,

    #[builder(into)]
    pub post_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct OrderInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub phone_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ShippingOption {
    #[builder(into)]
    pub id: String,

    #[builder(into)]
    pub title: String,

    pub prices: Vec<LabeledPrice>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SuccessfulPayment {
    #[builder(into)]
    pub currency: String,

    pub total_amount: u32,

    #[builder(into)]
    pub invoice_payload: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub shipping_option_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,

    #[builder(into)]
    pub telegram_payment_charge_id: String,

    #[builder(into)]
    pub provider_payment_charge_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct RefundedPayment {
    #[builder(into)]
    pub currency: String,

    pub total_amount: u32,

    #[builder(into)]
    pub invoice_payload: String,

    #[builder(into)]
    pub telegram_payment_charge_id: String,

    #[builder(into)]
    pub provider_payment_charge_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ShippingQuery {
    #[builder(into)]
    pub id: String,

    pub from: User,

    #[builder(into)]
    pub invoice_payload: String,

    pub shipping_address: ShippingAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PreCheckoutQuery {
    #[builder(into)]
    pub id: String,

    pub from: User,

    #[builder(into)]
    pub currency: String,

    pub total_amount: u32,

    #[builder(into)]
    pub invoice_payload: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub shipping_option_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PaidMediaPurchased {
    pub from: User,

    #[builder(into)]
    pub paid_media_payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,

    pub credentials: EncryptedCredentials,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportFile {
    #[builder(into)]
    pub file_id: String,

    #[builder(into)]
    pub file_unique_id: String,

    pub file_size: u64,

    pub file_date: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct EncryptedPassportElement {
    #[serde(rename = "type")]
    pub type_field: EncryptedPassportElementType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub phone_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<PassportFile>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_side: Option<PassportFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_side: Option<PassportFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<PassportFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,

    #[builder(into)]
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct EncryptedCredentials {
    #[builder(into)]
    pub data: String,

    #[builder(into)]
    pub hash: String,

    #[builder(into)]
    pub secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportElementErrorDataField {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorDataFieldType,

    #[builder(into)]
    pub field_name: String,

    #[builder(into)]
    pub data_hash: String,

    #[builder(into)]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportElementErrorFrontSide {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorFrontSideType,

    #[builder(into)]
    pub file_hash: String,

    #[builder(into)]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportElementErrorReverseSide {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorReverseSideType,

    #[builder(into)]
    pub file_hash: String,

    #[builder(into)]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportElementErrorSelfie {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorSelfieType,

    #[builder(into)]
    pub file_hash: String,

    #[builder(into)]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportElementErrorFile {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorFileType,

    #[builder(into)]
    pub file_hash: String,

    #[builder(into)]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportElementErrorFiles {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorFileType,

    pub file_hashes: Vec<String>,

    #[builder(into)]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportElementErrorTranslationFile {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorTranslationFileType,

    #[builder(into)]
    pub file_hash: String,

    #[builder(into)]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportElementErrorTranslationFiles {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorTranslationFileType,

    pub file_hashes: Vec<String>,

    #[builder(into)]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct PassportElementErrorUnspecified {
    #[serde(rename = "type")]
    pub type_field: EncryptedPassportElementType,

    #[builder(into)]
    pub element_hash: String,

    #[builder(into)]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Game {
    #[builder(into)]
    pub title: String,

    #[builder(into)]
    pub description: String,

    pub photo: Vec<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GameHighScore {
    pub position: u32,

    pub user: User,

    pub score: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GiveawayCreated {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct Giveaway {
    pub chats: Vec<Chat>,

    pub winners_selection_date: u64,

    pub winner_count: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_public_winners: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub prize_description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct GiveawayWinners {
    pub chat: Chat,

    pub giveaway_message_id: i32,

    pub winners_selection_date: u64,

    pub winner_count: u32,

    pub winners: Vec<User>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_chat_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_refunded: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub prize_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct GiveawayCompleted {
    pub winner_count: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub giveaway_message: Option<Box<Message>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_star_giveaway: Option<bool>,
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
    pub can_post_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_stories: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_stories: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_stories: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct WebAppInfo {
    #[builder(into)]
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct SentWebAppMessage {
    #[builder(into)]
    pub inline_message_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct WebAppData {
    #[builder(into)]
    pub data: String,

    #[builder(into)]
    pub button_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum ChatBoostSource {
    Premium(ChatBoostSourcePremium),
    GiftCode(ChatBoostSourceGiftCode),
    Giveaway(ChatBoostSourceGiveaway),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatBoostSourcePremium {
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatBoostSourceGiftCode {
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatBoostSourceGiveaway {
    pub giveaway_message_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unclaimed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatBoost {
    #[builder(into)]
    pub boost_id: String,

    pub add_date: u64,

    pub expiration_date: u64,

    pub source: ChatBoostSource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatBoostUpdated {
    pub chat: Chat,

    pub boost: ChatBoost,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct ChatBoostRemoved {
    pub chat: Chat,

    #[builder(into)]
    pub boost_id: String,

    pub remove_date: u64,

    pub source: ChatBoostSource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct UserChatBoosts {
    pub boosts: Vec<ChatBoost>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BusinessConnection {
    #[builder(into)]
    pub id: String,

    pub user: User,

    pub user_chat_id: u64,

    pub date: u64,

    pub can_reply: bool,

    pub is_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct BusinessMessagesDeleted {
    #[builder(into)]
    pub business_connection_id: String,

    pub chat: Chat,

    pub message_ids: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MaybeInaccessibleMessage {
    Message(Message),
    InaccessibleMessage(InaccessibleMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct InaccessibleMessage {
    pub chat: Chat,

    pub message_id: i32,

    pub date: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RevenueWithdrawalState {
    Pending(RevenueWithdrawalStatePending),
    Succeeded(RevenueWithdrawalStateSucceeded),
    Failed(RevenueWithdrawalStateFailed),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct RevenueWithdrawalStatePending {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct RevenueWithdrawalStateSucceeded {
    pub date: u64,

    #[builder(into)]
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct RevenueWithdrawalStateFailed {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransactionPartner {
    User(TransactionPartnerUser),
    Fragment(TransactionPartnerFragment),
    TelegramAds(TransactionPartnerTelegramAds),
    Other(TransactionPartnerOther),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct TransactionPartnerUser {
    pub user: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub invoice_payload: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<Vec<PaidMedia>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct TransactionPartnerFragment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdrawal_state: Option<RevenueWithdrawalState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct TransactionPartnerTelegramAds {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct TransactionPartnerOther {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct StarTransaction {
    #[builder(into)]
    pub id: String,

    pub amount: u32,

    pub date: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<TransactionPartner>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<TransactionPartner>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
pub struct StarTransactions {
    pub transactions: Vec<StarTransaction>,
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

    #[test]
    pub fn kicked_user_status_is_parsed() {
        let member_content = r#"{
            "status": "kicked",
            "until_date": 0,
            "user": {
                "id": 0,
                "is_bot": false,
                "first_name": "First"
            }
        }"#;

        let member: ChatMember = serde_json::from_str(member_content).unwrap();
        assert!(matches!(member, ChatMember::Kicked(_)));
    }

    #[test]
    pub fn unknown_entity_kind_is_parsed() {
        let entity_content = r#"{
            "type": "__unknown__",
            "offset": 10,
            "length": 20
        }"#;

        let entity: MessageEntity = serde_json::from_str(entity_content).unwrap();
        assert!(matches!(
            entity,
            MessageEntity {
                type_field: MessageEntityType::Unknown,
                ..
            }
        ));
    }
}
