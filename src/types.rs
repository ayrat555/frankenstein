//! [Available Types](https://core.telegram.org/bots/api#available-types) of the Bot API.

use serde::{Deserialize, Serialize};

use crate::games::{CallbackGame, Game};
use crate::gifts::{AcceptedGiftTypes, GiftInfo, UniqueGiftColors, UniqueGiftInfo};
use crate::macros::{apistruct, apply};
use crate::parse_mode::ParseMode;
use crate::passport::PassportData;
use crate::payments::{Invoice, RefundedPayment, StarAmount, SuccessfulPayment};
use crate::stickers::Sticker;

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

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ButtonStyle {
    Danger,
    Success,
    Primary,
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
pub struct ReplyParameters {
    pub message_id: i32,
    pub chat_id: Option<ChatId>,
    pub allow_sending_without_reply: Option<bool>,
    pub quote: Option<String>,
    pub quote_parse_mode: Option<ParseMode>,
    pub quote_entities: Option<Vec<MessageEntity>>,
    pub quote_position: Option<u32>,
    pub checklist_task_id: Option<i64>,
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
    Pattern(BackgroundTypePattern),
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
    pub can_manage_direct_messages: Option<bool>,
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
    BusinessConnection,
    BusinessMessage,
    EditedBusinessMessage,
    DeletedBusinessMessages,
    MessageReaction,
    MessageReactionCount,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
    ShippingQuery,
    PreCheckoutQuery,
    PurchasedPaidMedia,
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
    pub has_topics_enabled: Option<bool>,
    pub allows_users_to_create_topics: Option<bool>,
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
    pub is_direct_messages: Option<bool>,
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
    pub is_direct_messages: Option<bool>,
    pub photo: Option<ChatPhoto>,
    pub active_usernames: Option<Vec<String>>,
    pub birthdate: Option<Birthdate>,
    pub business_intro: Option<BusinessIntro>,
    pub business_location: Option<BusinessLocation>,
    pub business_opening_hours: Option<BusinessOpeningHours>,
    pub personal_chat: Option<Box<Chat>>,
    pub parent_chat: Option<Box<Chat>>,
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
    pub accepted_gift_types: AcceptedGiftTypes,
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
    pub rating: Option<UserRating>,
    pub first_profile_audio: Option<Audio>,
    pub unique_gift_colors: Option<UniqueGiftColors>,
    pub paid_message_star_count: Option<u32>,
}

#[apply(apistruct!)]
pub struct Message {
    pub message_id: i32,
    pub message_thread_id: Option<i32>,
    pub direct_messages_topic: Option<Box<DirectMessagesTopic>>,
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
    pub reply_to_message: Option<Box<Self>>,
    pub external_reply: Option<Box<ExternalReplyInfo>>,
    pub quote: Option<Box<TextQuote>>,
    pub reply_to_story: Option<Box<Story>>,
    pub reply_to_checklist_task_id: Option<i64>,
    pub via_bot: Option<Box<User>>,
    pub edit_date: Option<u64>,
    pub has_protected_content: Option<bool>,
    pub is_from_offline: Option<bool>,
    pub is_paid_post: Option<bool>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub link_preview_options: Option<LinkPreviewOptions>,
    pub suggested_post_info: Option<SuggestedPostInfo>,
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
    pub checklist: Option<Checklist>,
    pub contact: Option<Box<Contact>>,
    pub dice: Option<Box<Dice>>,
    pub game: Option<Box<Game>>,
    pub poll: Option<Box<Poll>>,
    pub venue: Option<Box<Venue>>,
    pub location: Option<Box<Location>>,
    pub new_chat_members: Option<Vec<User>>,
    pub left_chat_member: Option<Box<User>>,
    pub chat_owner_left: Option<Box<ChatOwnerLeft>>,
    pub chat_owner_changed: Option<Box<ChatOwnerChanged>>,
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
    pub gift: Option<GiftInfo>,
    pub unique_gift: Option<UniqueGiftInfo>,
    pub gift_upgrade_sent: Option<GiftInfo>,
    pub connected_website: Option<String>,
    pub write_access_allowed: Option<WriteAccessAllowed>,
    pub passport_data: Option<Box<PassportData>>,
    pub proximity_alert_triggered: Option<Box<ProximityAlertTriggered>>,
    pub boost_added: Option<Box<ChatBoostAdded>>,
    pub chat_background_set: Option<Box<ChatBackground>>,
    pub checklist_tasks_done: Option<Box<ChecklistTasksDone>>,
    pub checklist_tasks_added: Option<Box<ChecklistTasksAdded>>,
    pub direct_message_price_changed: Option<Box<DirectMessagePriceChanged>>,
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
    pub paid_message_price_changed: Option<PaidMessagePriceChanged>,
    pub suggested_post_approved: Option<Box<SuggestedPostApproved>>,
    pub suggested_post_approval_failed: Option<Box<SuggestedPostApprovalFailed>>,
    pub suggested_post_declined: Option<Box<SuggestedPostDeclined>>,
    pub suggested_post_paid: Option<Box<SuggestedPostPaid>>,
    pub suggested_post_refunded: Option<Box<SuggestedPostRefunded>>,
    pub video_chat_scheduled: Option<Box<VideoChatScheduled>>,
    pub video_chat_started: Option<Box<VideoChatStarted>>,
    pub video_chat_ended: Option<Box<VideoChatEnded>>,
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
    pub origin: MessageOrigin,
    pub chat: Option<Box<Chat>>,
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
    pub checklist: Option<Checklist>,
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

impl LinkPreviewOptions {
    pub const DISABLED: Self = Self {
        is_disabled: Some(true),
        url: None,
        prefer_small_media: None,
        prefer_large_media: None,
        show_above_text: None,
    };
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SuggestedPostPrice {
    pub currency: String,
    pub amount: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SuggestedPostState {
    Pending,
    Approved,
    Declined,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SuggestedPostInfo {
    pub state: SuggestedPostState,
    pub price: Option<SuggestedPostPrice>,
    pub send_date: Option<u64>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SuggestedPostParameters {
    pub price: Option<SuggestedPostPrice>,
    pub send_date: Option<u64>,
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
pub struct VideoQuality {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: u32,
    pub height: u32,
    pub codec: String,
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
    pub cover: Option<Vec<PhotoSize>>,
    pub start_timestamp: Option<u64>,
    pub qualities: Option<Vec<VideoQuality>>,
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
#[derive(Eq)]
pub struct ChecklistTask {
    pub id: i64,
    pub text: String,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub completed_by_user: Option<User>,
    pub completed_by_chat: Option<Box<Chat>>,
    pub completion_date: Option<u32>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct Checklist {
    pub title: String,
    pub title_entities: Option<Vec<MessageEntity>>,
    pub tasks: Vec<ChecklistTask>,
    pub others_can_add_tasks: Option<bool>,
    pub others_can_mark_tasks_as_done: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputChecklistTask {
    pub id: i64,
    pub text: String,
    pub parse_mode: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputChecklist {
    pub title: String,
    pub parse_mode: Option<String>,
    pub title_entities: Option<Vec<MessageEntity>>,
    pub tasks: Vec<InputChecklistTask>,
    pub others_can_add_tasks: Option<bool>,
    pub others_can_mark_tasks_as_done: Option<bool>,
}

#[apply(apistruct!)]
pub struct ChecklistTasksDone {
    pub checklist_message: Option<Message>,
    pub marked_as_done_task_ids: Option<Vec<i64>>,
    pub marked_as_not_done_task_ids: Option<Vec<i64>>,
}

#[apply(apistruct!)]
pub struct ChecklistTasksAdded {
    pub checklist_message: Option<Message>,
    pub tasks: Vec<ChecklistTask>,
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
#[derive(Eq)]
pub struct ChatOwnerLeft {
    pub new_owner: Option<User>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ChatOwnerChanged {
    pub new_owner: User,
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
    pub is_name_implicit: Option<bool>,
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
pub struct DirectMessagesTopic {
    pub topic_id: i64,
    pub user: Option<User>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct UserProfilePhotos {
    pub total_count: u32,
    pub photos: Vec<Vec<PhotoSize>>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct UserProfileAudios {
    pub total_count: u32,
    pub audios: Vec<Audio>,
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
    pub icon_custom_emoji_id: Option<String>,
    pub request_users: Option<KeyboardButtonRequestUsers>,
    pub request_chat: Option<KeyboardButtonRequestChat>,
    pub request_contact: Option<bool>,
    pub request_location: Option<bool>,
    pub request_poll: Option<KeyboardButtonPollType>,
    pub web_app: Option<WebAppInfo>,
    pub style: Option<ButtonStyle>,
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
    pub icon_custom_emoji_id: Option<String>,
    pub url: Option<String>,
    pub login_url: Option<LoginUrl>,
    pub callback_data: Option<String>,
    pub web_app: Option<WebAppInfo>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    pub copy_text: Option<CopyTextButton>,
    pub callback_game: Option<CallbackGame>,
    pub pay: Option<bool>,
    pub style: Option<ButtonStyle>,
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
#[derive(Eq)]
pub struct CopyTextButton {
    pub text: String,
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
    pub year: Option<u16>,
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
#[derive(Eq)]
pub struct UserRating {
    pub level: i32,
    pub rating: i32,
    pub current_level_rating: i32,
    pub next_level_rating: Option<i32>,
}

#[apply(apistruct!)]
pub struct StoryAreaPosition {
    pub x_percentage: f64,
    pub y_percentage: f64,
    pub width_percentage: f64,
    pub height_percentage: f64,
    pub rotation_angle: f64,
    pub corner_radius_percentage: f64,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct LocationAddress {
    pub country_code: String,
    pub state: Option<String>,
    pub city: Option<String>,
    pub street: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StoryAreaType {
    Location(StoryAreaTypeLocation),
    SuggestedReaction(StoryAreaTypeSuggestedReaction),
    Link(StoryAreaTypeLink),
    Weather(StoryAreaTypeWeather),
    UniqueGift(StoryAreaTypeUniqueGift),
}

#[apply(apistruct!)]
pub struct StoryAreaTypeLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub address: Option<LocationAddress>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct StoryAreaTypeSuggestedReaction {
    pub reaction_type: ReactionType,
    pub is_dark: Option<bool>,
    pub is_flipped: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct StoryAreaTypeLink {
    pub url: String,
}

#[apply(apistruct!)]
pub struct StoryAreaTypeWeather {
    pub temperature: f64,
    pub emoji: String,
    pub background_color: i64,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct StoryAreaTypeUniqueGift {
    pub name: String,
}

#[apply(apistruct!)]
pub struct StoryArea {
    pub position: StoryAreaPosition,
    #[serde(rename = "type")]
    pub type_field: StoryAreaType,
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
    pub is_name_implicit: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct Story {
    pub chat: Chat,
    pub id: u64,
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
    Video(Box<PaidMediaVideo>),
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

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PaidMessagePriceChanged {
    pub paid_message_star_count: u32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct DirectMessagePriceChanged {
    pub are_direct_messages_enabled: bool,
    pub direct_message_star_count: Option<u32>,
}

#[apply(apistruct!)]
pub struct SuggestedPostApproved {
    pub suggested_post_message: Option<Message>,
    pub price: Option<SuggestedPostPrice>,
    pub send_date: u64,
}

#[apply(apistruct!)]
pub struct SuggestedPostApprovalFailed {
    pub suggested_post_message: Option<Message>,
    pub price: SuggestedPostPrice,
}

#[apply(apistruct!)]
pub struct SuggestedPostDeclined {
    pub suggested_post_message: Option<Message>,
    pub comment: Option<String>,
}

#[apply(apistruct!)]
pub struct SuggestedPostPaid {
    pub suggested_post_message: Option<Message>,
    pub currency: String,
    pub amount: Option<u64>,
    pub star_amount: Option<StarAmount>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RefundReason {
    PostDeleted,
    PaymentRefunded,
}

#[apply(apistruct!)]
pub struct SuggestedPostRefunded {
    pub suggested_post_message: Option<Message>,
    pub reason: RefundReason,
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
    pub can_manage_direct_messages: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct WebAppInfo {
    pub url: String,
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
    pub rights: Option<BusinessBotRights>,
    pub is_enabled: bool,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct BusinessBotRights {
    pub can_reply: Option<bool>,
    pub can_read_messages: Option<bool>,
    pub can_delete_sent_messages: Option<bool>,
    pub can_delete_all_messages: Option<bool>,
    pub can_edit_name: Option<bool>,
    pub can_edit_bio: Option<bool>,
    pub can_edit_profile_photo: Option<bool>,
    pub can_edit_username: Option<bool>,
    pub can_change_gift_settings: Option<bool>,
    pub can_view_gifts_and_stars: Option<bool>,
    pub can_convert_gifts_to_stars: Option<bool>,
    pub can_transfer_and_upgrade_gifts: Option<bool>,
    pub can_transfer_stars: Option<bool>,
    pub can_manage_stories: Option<bool>,
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
    Message(Box<Message>),
    InaccessibleMessage(InaccessibleMessage),
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InaccessibleMessage {
    pub chat: Chat,
    pub message_id: i32,
    pub date: u64,
}

#[cfg(test)]
mod serde_tests {
    use super::*;

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
