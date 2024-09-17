//! Objects returned or used with the Telegram API.

#![allow(deprecated)]

use serde::{Deserialize, Serialize};

use crate::api_params::FileUpload;
use crate::macros::{apistruct, apply};
use crate::ParseMode;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StickerType {
    Regular,
    Mask,
    CustomEmoji,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StickerFormat {
    Static,
    Animated,
    Video,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum InputMessageContent {
    Text(InputTextMessageContent),
    Location(InputLocationMessageContent),
    Venue(InputVenueMessageContent),
    Contact(InputContactMessageContent),
    Invoice(InputInvoiceMessageContent),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ChatMember {
    Creator(ChatMemberOwner),
    Administrator(ChatMemberAdministrator),
    Member(ChatMemberMember),
    Restricted(ChatMemberRestricted),
    Left(ChatMemberLeft),
    Kicked(ChatMemberBanned),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
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
#[serde(rename_all = "snake_case")]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MenuButton {
    Commands,
    WebApp(MenuButtonWebApp),
    Default,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChatBackground {
    Fill(BackgroundTypeFill),
    Wallpaper(BackgroundTypeWallpaper),
    Patter(BackgroundTypePattern),
    ChatTheme(BackgroundTypeChatTheme),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BackgroundFill {
    Solid(BackgroundFillSolid),
    Gradient(BackgroundFillGradient),
    FreeformGradient(BackgroundFillFreeformGradient),
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct MenuButtonWebApp {
    pub text: String,
    pub web_app: WebAppInfo,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ChatMemberOwner {
    pub user: User,
    pub custom_title: Option<String>,
    pub is_anonymous: bool,
}

#[apply(apistruct!)]
#[derive(Eq)]
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
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_post_stories: Option<bool>,
    pub can_edit_stories: Option<bool>,
    pub can_delete_stories: Option<bool>,
    pub can_manage_topics: Option<bool>,
    pub custom_title: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ChatMemberMember {
    pub user: User,
    pub until_date: Option<u64>,
}

#[apply(apistruct!)]
#[derive(Eq)]
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

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ChatMemberLeft {
    pub user: User,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ChatMemberBanned {
    pub user: User,
    pub until_date: u64,
}

#[apply(apistruct!)]
#[derive(Copy, Eq)]
pub struct VideoChatStarted {}

#[apply(apistruct!)]
#[derive(Copy, Eq)]
pub struct VideoChatScheduled {
    pub start_date: u64,
}

#[apply(apistruct!)]
#[derive(Copy, Eq)]
pub struct CallbackGame {}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct BotDescription {
    pub description: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct BotName {
    pub name: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct BotShortDescription {
    pub short_description: String,
}

/// Represents an incoming update from telegram.
/// [Official documentation.](https://core.telegram.org/bots/api#update)
#[apply(apistruct!)]
pub struct Update {
    pub update_id: u32,

    /// Maps to exactly one of the many optional fields
    /// from [the official documentation](https://core.telegram.org/bots/api#update).
    #[serde(flatten)]
    pub content: UpdateContent,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

#[apply(apistruct!)]
#[derive(Eq)]
pub struct WebhookInfo {
    pub url: String,
    pub has_custom_certificate: bool,
    pub pending_update_count: u32,
    pub ip_address: Option<String>,
    pub last_error_date: Option<u64>,
    pub last_error_message: Option<String>,
    pub last_synchronization_error_date: Option<u64>,
    pub max_connections: Option<u16>,
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
}

/// Control which updates to receive.
/// Specify an empty list to receive all update types except `ChatMember`.
/// [Official documentation](https://core.telegram.org/bots/api#getupdates).
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

#[apply(apistruct!)]
#[derive(Eq)]
pub struct User {
    pub id: u64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub is_premium: Option<bool>,
    pub added_to_attachment_menu: Option<bool>,
    pub can_join_groups: Option<bool>,
    pub can_read_all_group_messages: Option<bool>,
    pub supports_inline_queries: Option<bool>,
    pub can_connect_to_business: Option<bool>,
    pub has_main_web_app: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct Chat {
    pub id: i64,

    #[serde(rename = "type")]
    pub type_field: ChatType,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_forum: Option<bool>,
}

#[apply(apistruct!)]
pub struct ChatFullInfo {
    pub id: i64,

    #[serde(rename = "type")]
    pub type_field: ChatType,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_forum: Option<bool>,
    pub photo: Option<ChatPhoto>,
    pub active_usernames: Option<Vec<String>>,
    pub birthdate: Option<Birthdate>,
    pub business_intro: Option<BusinessIntro>,
    pub business_location: Option<BusinessLocation>,
    pub business_opening_hours: Option<BusinessOpeningHours>,
    pub personal_chat: Option<Box<Chat>>,
    pub available_reactions: Option<Vec<ReactionType>>,
    pub accent_color_id: Option<u16>,
    pub max_reaction_count: Option<u16>,
    pub background_custom_emoji_id: Option<String>,
    pub profile_accent_color_id: Option<u16>,
    pub profile_background_custom_emoji_id: Option<String>,
    pub emoji_status_custom_emoji_id: Option<String>,
    pub emoji_status_expiration_date: Option<u32>,
    pub bio: Option<String>,
    pub has_private_forwards: Option<bool>,
    pub has_restricted_voice_and_video_messages: Option<bool>,
    pub join_to_send_messages: Option<bool>,
    pub join_by_request: Option<bool>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub pinned_message: Option<Box<Message>>,
    pub permissions: Option<ChatPermissions>,
    pub can_send_paid_media: Option<bool>,
    pub slow_mode_delay: Option<u16>,
    pub unrestrict_boost_count: Option<u32>,
    pub message_auto_delete_time: Option<u32>,
    pub has_aggressive_anti_spam_enabled: Option<bool>,
    pub has_hidden_members: Option<bool>,
    pub has_protected_content: Option<bool>,
    pub has_visible_history: Option<bool>,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<bool>,
    pub custom_emoji_sticker_set_name: Option<String>,
    pub linked_chat_id: Option<i64>,
    pub location: Option<ChatLocation>,
}

#[apply(apistruct!)]
pub struct Message {
    pub message_id: i32,
    pub message_thread_id: Option<i32>,
    pub from: Option<Box<User>>,
    pub sender_chat: Option<Box<Chat>>,
    pub sender_boost_count: Option<u32>,
    pub sender_business_bot: Option<Box<User>>,
    pub date: u64,
    pub business_connection_id: Option<String>,
    pub chat: Box<Chat>,
    pub forward_origin: Option<Box<MessageOrigin>>,
    pub is_topic_message: Option<bool>,
    pub is_automatic_forward: Option<bool>,
    pub reply_to_message: Option<Box<Message>>,
    pub external_reply: Option<Box<ExternalReplyInfo>>,
    pub quote: Option<Box<TextQuote>>,
    pub reply_to_story: Option<Box<Story>>,
    pub via_bot: Option<Box<User>>,
    pub edit_date: Option<u64>,
    pub has_protected_content: Option<bool>,
    pub is_from_offline: Option<bool>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub link_preview_options: Option<LinkPreviewOptions>,
    pub effect_id: Option<String>,
    pub animation: Option<Box<Animation>>,
    pub audio: Option<Box<Audio>>,
    pub document: Option<Box<Document>>,
    pub paid_media: Option<Box<PaidMediaInfo>>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Box<Sticker>>,
    pub story: Option<Box<Story>>,
    pub video: Option<Box<Video>>,
    pub video_note: Option<Box<VideoNote>>,
    pub voice: Option<Box<Voice>>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub has_media_spoiler: Option<bool>,
    pub contact: Option<Box<Contact>>,
    pub dice: Option<Box<Dice>>,
    pub game: Option<Box<Game>>,
    pub poll: Option<Box<Poll>>,
    pub venue: Option<Box<Venue>>,
    pub location: Option<Box<Location>>,
    pub new_chat_members: Option<Vec<User>>,
    pub left_chat_member: Option<Box<User>>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    pub delete_chat_photo: Option<bool>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub channel_chat_created: Option<bool>,
    pub message_auto_delete_timer_changed: Option<Box<MessageAutoDeleteTimerChanged>>,
    pub migrate_to_chat_id: Option<i64>,
    pub migrate_from_chat_id: Option<i64>,
    pub pinned_message: Option<Box<MaybeInaccessibleMessage>>,
    pub invoice: Option<Box<Invoice>>,
    pub successful_payment: Option<Box<SuccessfulPayment>>,
    pub refunded_payment: Option<Box<RefundedPayment>>,
    pub users_shared: Option<Box<UsersShared>>,
    pub chat_shared: Option<Box<ChatShared>>,
    pub connected_website: Option<String>,
    pub write_access_allowed: Option<WriteAccessAllowed>,
    pub passport_data: Option<Box<PassportData>>,
    pub proximity_alert_triggered: Option<Box<ProximityAlertTriggered>>,
    pub boost_added: Option<Box<ChatBoostAdded>>,
    pub chat_background_set: Option<Box<ChatBackground>>,
    pub forum_topic_created: Option<Box<ForumTopicCreated>>,
    pub forum_topic_edited: Option<Box<ForumTopicEdited>>,
    pub forum_topic_closed: Option<Box<ForumTopicClosed>>,
    pub forum_topic_reopened: Option<Box<ForumTopicReopened>>,
    pub general_forum_topic_hidden: Option<Box<GeneralForumTopicHidden>>,
    pub general_forum_topic_unhidden: Option<Box<GeneralForumTopicUnhidden>>,
    pub giveaway_created: Option<GiveawayCreated>,
    pub giveaway: Option<Giveaway>,
    pub giveaway_winners: Option<GiveawayWinners>,
    pub giveaway_completed: Option<GiveawayCompleted>,
    pub video_chat_started: Option<Box<VideoChatStarted>>,
    pub video_chat_ended: Option<Box<VideoChatEnded>>,
    pub video_chat_scheduled: Option<Box<VideoChatScheduled>>,
    pub video_chat_participants_invited: Option<Box<VideoChatParticipantsInvited>>,
    pub web_app_data: Option<Box<WebAppData>>,
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
}

#[apply(apistruct!)]
#[derive(Copy, Eq)]
pub struct MessageId {
    pub message_id: i32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    pub type_field: MessageEntityType,
    pub offset: u16,
    pub length: u16,
    pub url: Option<String>,
    pub user: Option<User>,
    pub language: Option<String>,
    pub custom_emoji_id: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct TextQuote {
    pub text: String,
    pub entities: Option<Vec<MessageEntity>>,
    pub position: u32,
    pub is_manual: Option<bool>,
}

#[apply(apistruct!)]
pub struct ExternalReplyInfo {
    pub origin: Option<MessageOrigin>,
    pub chat: Option<Chat>,
    pub message_id: Option<i32>,
    pub link_preview_options: Option<LinkPreviewOptions>,
    pub animation: Option<Animation>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub paid_media: Option<PaidMediaInfo>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub story: Option<Story>,
    pub video: Option<Video>,
    pub video_note: Option<VideoNote>,
    pub voice: Option<Voice>,
    pub has_media_spoiler: Option<bool>,
    pub contact: Option<Contact>,
    pub dice: Option<Dice>,
    pub game: Option<Game>,
    pub giveaway: Option<Giveaway>,
    pub giveaway_winners: Option<GiveawayWinners>,
    pub invoice: Option<Invoice>,
    pub location: Option<Location>,
    pub poll: Option<Poll>,
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

#[apply(apistruct!)]
#[derive(Eq)]
pub struct MessageOriginUser {
    pub date: u64,
    pub sender_user: User,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct MessageOriginHiddenUser {
    pub date: u64,
    pub sender_user_name: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct MessageOriginChat {
    pub date: u64,
    pub sender_chat: Chat,
    pub author_signature: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct MessageOriginChannel {
    pub date: u64,
    pub chat: Chat,
    pub message_id: i32,
    pub author_signature: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct LinkPreviewOptions {
    pub is_disabled: Option<bool>,
    pub url: Option<String>,
    pub prefer_small_media: Option<bool>,
    pub prefer_large_media: Option<bool>,
    pub show_above_text: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: u32,
    pub height: u32,
    pub file_size: Option<u64>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct Animation {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: u32,
    pub height: u32,
    pub duration: u32,
    pub thumbnail: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u64>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct Audio {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: u32,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u64>,
    pub thumbnail: Option<PhotoSize>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct Document {
    pub file_id: String,
    pub file_unique_id: String,
    pub thumbnail: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u64>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct Video {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: u32,
    pub height: u32,
    pub duration: u32,
    pub thumbnail: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u64>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: u32,
    pub duration: u32,
    pub thumbnail: Option<PhotoSize>,
    pub file_size: Option<u64>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct Voice {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: u32,
    pub mime_type: Option<String>,
    pub file_size: Option<u64>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<u64>,
    pub vcard: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct Dice {
    pub emoji: String,
    pub value: u8,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PollOption {
    pub text: String,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub voter_count: u32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputPollOption {
    pub text: Option<String>,
    pub text_parse_mode: Option<ParseMode>,
    pub text_entities: Option<Vec<MessageEntity>>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PollAnswer {
    pub poll_id: String,
    pub voter_chat: Option<Chat>,
    pub user: Option<Box<User>>,
    pub option_ids: Vec<u8>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct Poll {
    pub id: String,
    pub question: String,
    pub question_entities: Option<Vec<MessageEntity>>,
    pub options: Vec<PollOption>,
    pub total_voter_count: u32,
    pub is_closed: bool,
    pub is_anonymous: bool,

    #[serde(rename = "type")]
    pub type_field: PollType,
    pub allows_multiple_answers: bool,
    pub correct_option_id: Option<u8>,
    pub explanation: Option<String>,
    pub explanation_entities: Option<Vec<MessageEntity>>,
    pub open_period: Option<u32>,
    pub close_date: Option<u64>,
}

#[apply(apistruct!)]
#[derive(Copy)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<u32>,
    pub heading: Option<u16>,
    pub proximity_alert_radius: Option<u32>,
}

#[apply(apistruct!)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ProximityAlertTriggered {
    pub traveler: User,
    pub watcher: User,
    pub distance: u32,
}

#[apply(apistruct!)]
#[derive(Copy, Eq)]
pub struct MessageAutoDeleteTimerChanged {
    pub message_auto_delete_time: u32,
}

#[apply(apistruct!)]
#[derive(Copy, Eq)]
pub struct ChatBoostAdded {
    pub boost_count: u32,
}

#[apply(apistruct!)]
#[derive(Copy, Eq)]
pub struct BackgroundFillSolid {
    pub color: u32,
}

#[apply(apistruct!)]
#[derive(Copy, Eq)]
pub struct BackgroundFillGradient {
    pub top_color: u32,
    pub bottom_color: u32,
    pub rotation_angle: u16,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct BackgroundFillFreeformGradient {
    pub colors: Vec<u32>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct BackgroundTypeFill {
    pub fill: BackgroundFill,
    pub dark_theme_dimming: u8,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct BackgroundTypeWallpaper {
    pub document: Document,
    pub dark_theme_dimming: u8,
    pub is_blurred: Option<bool>,
    pub is_moving: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct BackgroundTypePattern {
    pub document: Document,
    pub fill: BackgroundFill,
    pub intensity: u8,
    pub is_inverted: Option<bool>,
    pub is_moving: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct BackgroundTypeChatTheme {
    pub theme_name: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ForumTopicCreated {
    pub name: String,
    pub icon_color: u32,
    pub icon_custom_emoji_id: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ForumTopicClosed {}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ForumTopicEdited {
    pub name: Option<String>,
    pub icon_custom_emoji_id: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ForumTopicReopened {}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GeneralForumTopicHidden {}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GeneralForumTopicUnhidden {}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SharedUser {
    pub user_id: u64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub photo: Option<Vec<PhotoSize>>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct UsersShared {
    pub request_id: i32,
    pub users: Vec<SharedUser>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ChatShared {
    pub request_id: i32,
    pub chat_id: i64,
    pub title: Option<String>,
    pub username: Option<String>,
    pub photo: Option<Vec<PhotoSize>>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct WriteAccessAllowed {
    pub from_request: Option<bool>,
    pub web_app_name: Option<String>,
    pub from_attachment_menu: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Copy, Eq)]
pub struct VideoChatEnded {
    pub duration: u32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct VideoChatParticipantsInvited {
    pub users: Option<Vec<User>>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct UserProfilePhotos {
    pub total_count: u32,
    pub photos: Vec<Vec<PhotoSize>>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct File {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: Option<u64>,
    pub file_path: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    pub is_persistent: Option<bool>,
    pub resize_keyboard: Option<bool>,
    pub one_time_keyboard: Option<bool>,
    pub input_field_placeholder: Option<String>,
    pub selective: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct KeyboardButton {
    pub text: String,
    pub request_users: Option<KeyboardButtonRequestUsers>,
    pub request_chat: Option<KeyboardButtonRequestChat>,
    pub request_contact: Option<bool>,
    pub request_location: Option<bool>,
    pub request_poll: Option<KeyboardButtonPollType>,
    pub web_app: Option<WebAppInfo>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct KeyboardButtonRequestUsers {
    pub request_id: i32,
    pub user_is_bot: Option<bool>,
    pub user_is_premium: Option<bool>,
    pub max_quantity: Option<u32>,
    pub request_name: Option<bool>,
    pub request_username: Option<bool>,
    pub request_photo: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct KeyboardButtonRequestChat {
    pub request_id: i32,
    pub chat_is_channel: bool,
    pub chat_is_forum: Option<bool>,
    pub chat_has_username: Option<bool>,
    pub chat_is_created: Option<bool>,
    pub user_administrator_rights: Option<ChatAdministratorRights>,
    pub bot_administrator_rights: Option<ChatAdministratorRights>,
    pub bot_is_member: Option<bool>,
    pub request_title: Option<bool>,
    pub request_username: Option<bool>,
    pub request_photo: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Copy, Eq)]
pub struct KeyboardButtonPollType {
    #[serde(rename = "type")]
    pub type_field: Option<PollType>,
}

#[apply(apistruct!)]
#[derive(Copy, Eq)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: bool,
    pub selective: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub url: Option<String>,
    pub login_url: Option<LoginUrl>,
    pub callback_data: Option<String>,
    pub web_app: Option<WebAppInfo>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    pub callback_game: Option<CallbackGame>,
    pub pay: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct LoginUrl {
    pub url: String,
    pub forward_text: Option<String>,
    pub bot_username: Option<String>,
    pub request_write_access: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SwitchInlineQueryChosenChat {
    pub query: Option<String>,
    pub allow_user_chats: Option<bool>,
    pub allow_bot_chats: Option<bool>,
    pub allow_group_chats: Option<bool>,
    pub allow_channel_chats: Option<bool>,
}

#[apply(apistruct!)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<MaybeInaccessibleMessage>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ForceReply {
    pub force_reply: bool,
    pub input_field_placeholder: Option<String>,
    pub selective: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub small_file_unique_id: String,
    pub big_file_id: String,
    pub big_file_unique_id: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ChatInviteLink {
    pub invite_link: String,
    pub creator: User,
    pub creates_join_request: bool,
    pub is_primary: bool,
    pub is_revoked: bool,
    pub name: Option<String>,
    pub expire_date: Option<u64>,
    pub member_limit: Option<u32>,
    pub pending_join_request_count: Option<u32>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ChatMemberUpdated {
    pub chat: Chat,
    pub from: User,
    pub date: u64,
    pub old_chat_member: ChatMember,
    pub new_chat_member: ChatMember,
    pub invite_link: Option<ChatInviteLink>,
    pub via_join_request: Option<bool>,
    pub via_chat_folder_invite_link: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ChatJoinRequest {
    pub chat: Chat,
    pub from: User,
    pub user_chat_id: u64,
    pub date: u64,
    pub bio: Option<String>,
    pub invite_link: Option<ChatInviteLink>,
}

#[apply(apistruct!)]
#[derive(Copy, Eq)]
pub struct ChatPermissions {
    pub can_send_messages: Option<bool>,
    pub can_send_audios: Option<bool>,
    pub can_send_documents: Option<bool>,
    pub can_send_photos: Option<bool>,
    pub can_send_videos: Option<bool>,
    pub can_send_video_notes: Option<bool>,
    pub can_send_voice_notes: Option<bool>,
    pub can_send_polls: Option<bool>,
    pub can_send_other_messages: Option<bool>,
    pub can_add_web_page_previews: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_manage_topics: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct Birthdate {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

#[apply(apistruct!)]
pub struct BusinessIntro {
    pub title: Option<String>,
    pub message: Option<String>,
    pub sticker: Option<Sticker>,
}

#[apply(apistruct!)]
pub struct BusinessLocation {
    pub address: String,
    pub location: Option<Location>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct BusinessOpeningHoursInterval {
    pub opening_minute: u16,
    pub closing_minute: u16,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct BusinessOpeningHours {
    pub time_zone_name: String,
    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}

#[apply(apistruct!)]
pub struct ChatLocation {
    pub location: Location,
    pub address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ReactionType {
    Emoji(ReactionTypeEmoji),
    CustomEmoji(ReactionTypeCustomEmoji),
    Paid(ReactionTypePaid),
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ReactionTypeEmoji {
    pub emoji: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ReactionTypeCustomEmoji {
    pub custom_emoji_id: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ReactionTypePaid {}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ReactionCount {
    #[serde(rename = "type")]
    pub type_field: ReactionType,
    pub total_count: i32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct MessageReactionUpdated {
    pub chat: Chat,
    pub message_id: i32,
    pub user: Option<User>,
    pub actor_chat: Option<Chat>,
    pub date: u64,
    pub old_reaction: Vec<ReactionType>,
    pub new_reaction: Vec<ReactionType>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct MessageReactionCountUpdated {
    pub chat: Chat,
    pub message_id: i32,
    pub date: u64,
    pub reactions: Vec<ReactionCount>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ForumTopic {
    pub message_thread_id: i32,
    pub name: String,
    pub icon_color: u32,
    pub icon_custom_emoji_id: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}

#[apply(apistruct!)]
#[derive(Copy, Eq)]
pub struct ResponseParameters {
    pub migrate_to_chat_id: Option<i64>,
    pub retry_after: Option<u16>,
}

#[apply(apistruct!)]
pub struct Sticker {
    pub file_id: String,
    pub file_unique_id: String,

    #[serde(rename = "type")]
    pub sticker_type: StickerType,
    pub width: u32,
    pub height: u32,
    pub is_animated: bool,
    pub is_video: bool,
    pub thumbnail: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
    pub premium_animation: Option<File>,
    pub mask_position: Option<MaskPosition>,
    pub custom_emoji_id: Option<String>,
    pub needs_repainting: Option<bool>,
    pub file_size: Option<u64>,
}

#[apply(apistruct!)]
pub struct InputSticker {
    pub sticker: FileUpload,
    pub format: StickerFormat,
    pub emoji_list: Vec<String>,
    pub mask_position: Option<MaskPosition>,
    pub keywords: Option<Vec<String>>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct Story {
    pub chat: Chat,
    pub id: u64,
}

#[apply(apistruct!)]
pub struct StickerSet {
    pub name: String,
    pub title: String,

    #[serde(rename = "sticker_type")]
    pub sticker_type: StickerType,

    #[doc(hidden)]
    #[deprecated(since = "0.19.2", note = "Please use `sticker_type` instead")]
    pub contains_masks: bool,
    pub stickers: Vec<Sticker>,
    pub thumbnail: Option<PhotoSize>,
}

#[apply(apistruct!)]
pub struct MaskPosition {
    pub point: String,
    pub x_shift: f64,
    pub y_shift: f64,
    pub scale: f64,
}

#[apply(apistruct!)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub location: Option<Location>,
    pub chat_type: Option<String>,
    pub query: String,
    pub offset: String,
}

#[apply(apistruct!)]
pub struct InlineQueryResultArticle {
    pub id: String,
    pub title: String,
    pub input_message_content: InputMessageContent,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub url: Option<String>,
    pub hide_url: Option<bool>,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<u32>,
    pub thumbnail_height: Option<u32>,
}

#[apply(apistruct!)]
pub struct InlineQueryResultPhoto {
    pub id: String,
    pub photo_url: String,
    pub thumbnail_url: String,
    pub photo_width: Option<u32>,
    pub photo_height: Option<u32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[apply(apistruct!)]
pub struct InlineQueryResultGif {
    pub id: String,
    pub gif_url: String,
    pub gif_width: Option<u32>,
    pub gif_height: Option<u32>,
    pub gif_duration: Option<u32>,
    pub thumbnail_url: String,
    pub thumbnail_mime_type: Option<String>,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[apply(apistruct!)]
pub struct InlineQueryResultMpeg4Gif {
    pub id: String,
    pub mpeg4_url: String,
    pub mpeg4_width: Option<u32>,
    pub mpeg4_height: Option<u32>,
    pub mpeg4_duration: Option<u32>,
    pub thumbnail_url: String,
    pub thumbnail_mime_type: Option<String>,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[apply(apistruct!)]
pub struct InlineQueryResultVideo {
    pub id: String,
    pub video_url: String,
    pub mime_type: String,
    pub thumbnail_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub video_width: Option<u32>,
    pub video_height: Option<u32>,
    pub video_duration: Option<u32>,
    pub description: Option<String>,
    pub show_caption_above_media: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[apply(apistruct!)]
pub struct InlineQueryResultAudio {
    pub id: String,
    pub audio_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub performer: Option<String>,
    pub audio_duration: Option<u32>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[apply(apistruct!)]
pub struct InlineQueryResultVoice {
    pub id: String,
    pub voice_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub voice_duration: Option<u32>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[apply(apistruct!)]
pub struct InlineQueryResultDocument {
    pub id: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub document_url: String,
    pub mime_type: String,
    pub description: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<u32>,
    pub thumbnail_height: Option<u32>,
}

#[apply(apistruct!)]
pub struct InlineQueryResultLocation {
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<u32>,
    pub heading: Option<u16>,
    pub proximity_alert_radius: Option<u32>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<u32>,
    pub thumbnail_height: Option<u32>,
}

#[apply(apistruct!)]
pub struct InlineQueryResultVenue {
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<u32>,
    pub thumbnail_height: Option<u32>,
}

#[apply(apistruct!)]
pub struct InlineQueryResultContact {
    pub id: String,
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<u32>,
    pub thumbnail_height: Option<u32>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InlineQueryResultGame {
    pub id: String,
    pub game_short_name: String,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[apply(apistruct!)]
pub struct InlineQueryResultCachedPhoto {
    pub id: String,
    pub photo_file_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[apply(apistruct!)]
pub struct InlineQueryResultCachedGif {
    pub id: String,
    pub gif_file_id: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[apply(apistruct!)]
pub struct InlineQueryResultCachedMpeg4Gif {
    pub id: String,
    pub mpeg4_file_id: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[apply(apistruct!)]
pub struct InlineQueryResultCachedSticker {
    pub id: String,
    pub sticker_file_id: String,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[apply(apistruct!)]
pub struct InlineQueryResultCachedDocument {
    pub id: String,
    pub title: String,
    pub document_file_id: String,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[apply(apistruct!)]
pub struct InlineQueryResultCachedVideo {
    pub id: String,
    pub video_file_id: String,
    pub title: String,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[apply(apistruct!)]
pub struct InlineQueryResultCachedVoice {
    pub id: String,
    pub voice_file_id: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[apply(apistruct!)]
pub struct InlineQueryResultCachedAudio {
    pub id: String,
    pub audio_file_id: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputTextMessageContent {
    pub message_text: String,
    pub parse_mode: Option<ParseMode>,
    pub entities: Option<Vec<MessageEntity>>,
    pub link_preview_options: Option<LinkPreviewOptions>,
}

#[apply(apistruct!)]
#[derive(Copy)]
pub struct InputLocationMessageContent {
    pub latitude: f64,
    pub longitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<u32>,
    pub heading: Option<u16>,
    pub proximity_alert_radius: Option<u32>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputInvoiceMessageContent {
    pub title: String,
    pub description: String,
    pub payload: String,
    pub provider_token: Option<String>,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
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
pub struct InputVenueMessageContent {
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputContactMessageContent {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
}

#[apply(apistruct!)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<String>,
    pub query: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: u32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: u32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PaidMediaInfo {
    pub star_count: u32,
    pub paid_media: Vec<PaidMedia>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PaidMedia {
    Preview(PaidMediaPreview),
    Photo(PaidMediaPhoto),
    Video(PaidMediaVideo),
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PaidMediaPreview {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<u32>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PaidMediaPhoto {
    pub photo: Vec<PhotoSize>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PaidMediaVideo {
    pub video: Video,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputPaidMedia {
    Photo(InputPaidMediaPhoto),
    Video(InputPaidMediaVideo),
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputPaidMediaPhoto {
    pub media: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputPaidMediaVideo {
    pub media: String,
    pub thumbnail: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<u32>,
    pub supports_streaming: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct OrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ShippingOption {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: u32,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct RefundedPayment {
    pub currency: String,
    pub total_amount: u32,
    pub invoice_payload: String,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: u32,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PaidMediaPurchased {
    pub from: User,
    pub paid_media_payload: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PassportFile {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: u64,
    pub file_date: u64,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct EncryptedPassportElement {
    #[serde(rename = "type")]
    pub type_field: EncryptedPassportElementType,
    pub data: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub files: Option<Vec<PassportFile>>,
    pub front_side: Option<PassportFile>,
    pub reverse_side: Option<PassportFile>,
    pub selfie: Option<PassportFile>,
    pub translation: Option<Vec<PassportFile>>,
    pub hash: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct EncryptedCredentials {
    pub data: String,
    pub hash: String,
    pub secret: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PassportElementErrorDataField {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorDataFieldType,
    pub field_name: String,
    pub data_hash: String,
    pub message: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PassportElementErrorFrontSide {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorFrontSideType,
    pub file_hash: String,
    pub message: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PassportElementErrorReverseSide {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorReverseSideType,
    pub file_hash: String,
    pub message: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PassportElementErrorSelfie {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorSelfieType,
    pub file_hash: String,
    pub message: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PassportElementErrorFile {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorFileType,
    pub file_hash: String,
    pub message: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PassportElementErrorFiles {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorFileType,
    pub file_hashes: Vec<String>,
    pub message: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PassportElementErrorTranslationFile {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorTranslationFileType,
    pub file_hash: String,
    pub message: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PassportElementErrorTranslationFiles {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorTranslationFileType,
    pub file_hashes: Vec<String>,
    pub message: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PassportElementErrorUnspecified {
    #[serde(rename = "type")]
    pub type_field: EncryptedPassportElementType,
    pub element_hash: String,
    pub message: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GameHighScore {
    pub position: u32,
    pub user: User,
    pub score: i32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GiveawayCreated {
    pub prize_star_count: Option<u32>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct Giveaway {
    pub chats: Vec<Chat>,
    pub winners_selection_date: u64,
    pub winner_count: u32,
    pub only_new_members: Option<bool>,
    pub has_public_winners: Option<bool>,
    pub prize_description: Option<String>,
    pub country_codes: Option<Vec<String>>,
    pub prize_star_count: Option<u32>,
    pub premium_subscription_month_count: Option<u32>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GiveawayWinners {
    pub chat: Chat,
    pub giveaway_message_id: i32,
    pub winners_selection_date: u64,
    pub winner_count: u32,
    pub winners: Vec<User>,
    pub additional_chat_count: Option<u32>,
    pub prize_star_count: Option<u32>,
    pub premium_subscription_month_count: Option<u32>,
    pub unclaimed_prize_count: Option<u32>,
    pub only_new_members: Option<bool>,
    pub was_refunded: Option<bool>,
    pub prize_description: Option<String>,
}

#[apply(apistruct!)]
pub struct GiveawayCompleted {
    pub winner_count: u32,
    pub unclaimed_prize_count: Option<u32>,
    pub giveaway_message: Option<Box<Message>>,
    pub is_star_giveaway: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Copy, Eq)]
pub struct ChatAdministratorRights {
    pub is_anonymous: bool,
    pub can_manage_chat: bool,
    pub can_delete_messages: bool,
    pub can_manage_video_chats: bool,
    pub can_restrict_members: bool,
    pub can_promote_members: bool,
    pub can_change_info: bool,
    pub can_invite_users: bool,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_post_stories: Option<bool>,
    pub can_edit_stories: Option<bool>,
    pub can_delete_stories: Option<bool>,
    pub can_manage_topics: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct WebAppInfo {
    pub url: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SentWebAppMessage {
    pub inline_message_id: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct WebAppData {
    pub data: String,
    pub button_text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum ChatBoostSource {
    Premium(ChatBoostSourcePremium),
    GiftCode(ChatBoostSourceGiftCode),
    Giveaway(ChatBoostSourceGiveaway),
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ChatBoostSourcePremium {
    pub user: User,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ChatBoostSourceGiftCode {
    pub user: User,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ChatBoostSourceGiveaway {
    pub giveaway_message_id: i32,
    pub user: Option<User>,
    pub prize_star_count: Option<u32>,
    pub is_unclaimed: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ChatBoost {
    pub boost_id: String,
    pub add_date: u64,
    pub expiration_date: u64,
    pub source: ChatBoostSource,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ChatBoostUpdated {
    pub chat: Chat,
    pub boost: ChatBoost,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ChatBoostRemoved {
    pub chat: Chat,
    pub boost_id: String,
    pub remove_date: u64,
    pub source: ChatBoostSource,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct UserChatBoosts {
    pub boosts: Vec<ChatBoost>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct BusinessConnection {
    pub id: String,
    pub user: User,
    pub user_chat_id: u64,
    pub date: u64,
    pub can_reply: bool,
    pub is_enabled: bool,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct BusinessMessagesDeleted {
    pub business_connection_id: String,
    pub chat: Chat,
    pub message_ids: Vec<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MaybeInaccessibleMessage {
    Message(Message),
    InaccessibleMessage(InaccessibleMessage),
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InaccessibleMessage {
    pub chat: Chat,
    pub message_id: i32,
    pub date: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RevenueWithdrawalState {
    Pending(RevenueWithdrawalStatePending),
    Succeeded(RevenueWithdrawalStateSucceeded),
    Failed(RevenueWithdrawalStateFailed),
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct RevenueWithdrawalStatePending {}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct RevenueWithdrawalStateSucceeded {
    pub date: u64,
    pub url: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct RevenueWithdrawalStateFailed {}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransactionPartner {
    User(TransactionPartnerUser),
    Fragment(TransactionPartnerFragment),
    TelegramAds(TransactionPartnerTelegramAds),
    Other(TransactionPartnerOther),
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct TransactionPartnerUser {
    pub user: User,
    pub invoice_payload: Option<String>,
    pub paid_media: Option<Vec<PaidMedia>>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct TransactionPartnerFragment {
    pub withdrawal_state: Option<RevenueWithdrawalState>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct TransactionPartnerTelegramAds {}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct TransactionPartnerOther {}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct StarTransaction {
    pub id: String,
    pub amount: u32,
    pub date: u64,
    pub source: Option<TransactionPartner>,
    pub receiver: Option<TransactionPartner>,
}

#[apply(apistruct!)]
#[derive(Eq)]
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
