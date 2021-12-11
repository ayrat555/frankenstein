use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum InputMessageContent {
    Text(InputTextMessageContent),
    Location(InputLocationMessageContent),
    Venue(InputVenueMessageContent),
    Contact(InputContactMessageContent),
    Invoice(InputInvoiceMessageContent),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status")]
pub enum ChatMember {
    #[serde(rename = "creator")]
    Owner(ChatMemberOwner),
    #[serde(rename = "administrator")]
    Administrator(ChatMemberAdministrator),
    #[serde(rename = "member")]
    Member(ChatMemberMember),
    #[serde(rename = "restricted")]
    Restricted(ChatMemberRestricted),
    #[serde(rename = "left")]
    Left(ChatMemberLeft),
    #[serde(rename = "banned")]
    Banned(ChatMemberBanned),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ChatType {
    Private,
    Group,
    Supergroup,
    Channel,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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
    Code,
    Pre,
    TextLink,
    TextMention,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PollType {
    Regular,
    Quiz,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorDataFieldType {
    PersonalDetails,
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
    Address,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorFrontSideType {
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorReverseSideType {
    DriverLicense,
    IdentityCard,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorSelfieType {
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorFileType {
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ChatMemberOwner {
    pub user: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,

    pub is_anonymous: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ChatMemberAdministrator {
    pub user: User,

    pub can_be_edited: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,

    pub is_anonymous: bool,

    pub can_manage_chat: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,

    pub can_delete_messages: bool,

    pub can_manage_voice_chats: bool,

    pub can_restrict_members: bool,

    pub can_promote_members: bool,

    pub can_change_info: bool,

    pub can_invite_users: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ChatMemberMember {
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ChatMemberRestricted {
    pub user: User,

    pub is_member: bool,

    pub can_change_info: bool,

    pub can_invite_users: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,

    pub can_send_messages: bool,

    pub can_send_media_messages: bool,

    pub can_send_polls: bool,

    pub can_send_other_messages: bool,

    pub can_add_web_page_previews: bool,

    pub until_date: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ChatMemberLeft {
    pub user: User,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ChatMemberBanned {
    pub user: User,

    pub until_date: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct VoiceChatStarted {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct VoiceChatScheduled {
    pub start_date: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct CallbackGame {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Update {
    pub update_id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_message: Option<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_post: Option<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_channel_post: Option<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_query: Option<InlineQuery>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chosen_inline_result: Option<ChosenInlineResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_query: Option<CallbackQuery>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_query: Option<ShippingQuery>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_checkout_query: Option<PreCheckoutQuery>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_answer: Option<PollAnswer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_chat_member: Option<ChatMemberUpdated>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_member: Option<ChatMemberUpdated>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_join_request: Option<ChatJoinRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct WebhookInfo {
    pub url: String,

    pub has_custom_certificate: bool,

    pub pending_update_count: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct User {
    pub id: u64,

    pub is_bot: bool,

    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_join_groups: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_all_group_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_inline_queries: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Chat {
    pub id: i64,

    #[serde(rename = "type")]
    pub type_field: ChatType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_private_forwards: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ChatLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Message {
    pub message_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<User>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_chat: Option<Chat>,

    pub date: u64,

    pub chat: Chat,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from: Option<User>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from_chat: Option<Chat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from_message_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_signature: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_sender_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_forward: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message: Option<Box<Message>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<User>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Game>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_members: Option<Vec<User>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_chat_member: Option<User>,

    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_from_chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_payment: Option<SuccessfulPayment>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_website: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_data: Option<PassportData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_chat_started: Option<VoiceChatStarted>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_chat_ended: Option<VoiceChatEnded>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_chat_scheduled: Option<VoiceChatScheduled>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_chat_participants_invited: Option<VoiceChatParticipantsInvited>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct MessageId {
    pub message_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    pub type_field: MessageEntityType,

    pub offset: u16,

    pub length: u16,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct PhotoSize {
    pub file_id: String,

    pub file_unique_id: String,

    pub width: u32,

    pub height: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Animation {
    pub file_id: String,

    pub file_unique_id: String,

    pub width: u32,

    pub height: u32,

    pub duration: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Audio {
    pub file_id: String,

    pub file_unique_id: String,

    pub duration: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Document {
    pub file_id: String,

    pub file_unique_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Video {
    pub file_id: String,

    pub file_unique_id: String,

    pub width: u32,

    pub height: u32,

    pub duration: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct VideoNote {
    pub file_id: String,

    pub file_unique_id: String,

    pub length: u32,

    pub duration: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Voice {
    pub file_id: String,

    pub file_unique_id: String,

    pub duration: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Contact {
    pub phone_number: String,

    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Dice {
    pub emoji: String,

    pub value: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct PollOption {
    pub text: String,

    pub voter_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct PollAnswer {
    pub poll_id: String,

    pub user: User,

    pub option_ids: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Poll {
    pub id: String,

    pub question: String,

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
    pub explanation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ProximityAlertTriggered {
    pub traveler: User,

    pub watcher: User,

    pub distance: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct MessageAutoDeleteTimerChanged {
    pub message_auto_delete_time: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct VoiceChatEnded {
    pub duration: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct VoiceChatParticipantsInvited {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct UserProfilePhotos {
    pub total_count: u32,

    pub photos: Vec<Vec<PhotoSize>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct File {
    pub file_id: String,

    pub file_unique_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_keyboard: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_keyboard: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct KeyboardButton {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<KeyboardButtonPollType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct KeyboardButtonPollType {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<PollType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineKeyboardButton {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<LoginUrl>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<CallbackGame>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct LoginUrl {
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_write_access: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct CallbackQuery {
    pub id: String,

    pub from: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    pub chat_instance: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_short_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ForceReply {
    pub force_reply: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ChatPhoto {
    pub small_file_id: String,

    pub small_file_unique_id: String,

    pub big_file_id: String,

    pub big_file_unique_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ChatInviteLink {
    pub invite_link: String,

    pub creator: User,

    pub creates_join_request: bool,

    pub is_primary: bool,

    pub is_revoked: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub invite_link: Option<ChatInviteLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ChatJoinRequest {
    pub chat: Chat,

    pub from: User,

    pub date: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ChatPermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_media_messages: Option<bool>,

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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ChatLocation {
    pub location: Location,

    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct BotCommand {
    pub command: String,

    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ResponseParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Sticker {
    pub file_id: String,

    pub file_unique_id: String,

    pub width: u32,

    pub height: u32,

    pub is_animated: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct StickerSet {
    pub name: String,

    pub title: String,

    pub is_animated: bool,

    pub contains_masks: bool,

    pub stickers: Vec<Sticker>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct MaskPosition {
    pub point: String,

    pub x_shift: f64,

    pub y_shift: f64,

    pub scale: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQuery {
    pub id: String,

    pub from: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,

    pub query: String,

    pub offset: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultArticle {
    pub id: String,

    pub title: String,

    pub input_message_content: InputMessageContent,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_url: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultPhoto {
    pub id: String,

    pub photo_url: String,

    pub thumb_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultGif {
    pub id: String,

    pub gif_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_duration: Option<u32>,

    pub thumb_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultMpeg4Gif {
    pub id: String,

    pub mpeg4_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_duration: Option<u32>,

    pub thumb_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultVideo {
    pub id: String,

    pub video_url: String,

    pub mime_type: String,

    pub thumb_url: String,

    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultAudio {
    pub id: String,

    pub audio_url: String,

    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub id: String,

    pub voice_url: String,

    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

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
    pub id: String,

    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    pub document_url: String,

    pub mime_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultLocation {
    pub id: String,

    pub latitude: f64,

    pub longitude: f64,

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
    pub thumb_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultVenue {
    pub id: String,

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
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultContact {
    pub id: String,

    pub phone_number: String,

    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultGame {
    pub id: String,

    pub game_short_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedPhoto {
    pub id: String,

    pub photo_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedGif {
    pub id: String,

    pub gif_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedMpeg4Gif {
    pub id: String,

    pub mpeg4_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedSticker {
    pub id: String,

    pub sticker_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedDocument {
    pub id: String,

    pub title: String,

    pub document_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedVideo {
    pub id: String,

    pub video_file_id: String,

    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedVoice {
    pub id: String,

    pub voice_file_id: String,

    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InlineQueryResultCachedAudio {
    pub id: String,

    pub audio_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InputTextMessageContent {
    pub message_text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InputInvoiceMessageContent {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InputVenueMessageContent {
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct InputContactMessageContent {
    pub phone_number: String,

    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ChosenInlineResult {
    pub result_id: String,

    pub from: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct LabeledPrice {
    pub label: String,

    pub amount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Invoice {
    pub title: String,

    pub description: String,

    pub start_parameter: String,

    pub currency: String,

    pub total_amount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ShippingAddress {
    pub country_code: String,

    pub state: String,

    pub city: String,

    pub street_line1: String,

    pub street_line2: String,

    pub post_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct OrderInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ShippingOption {
    pub id: String,

    pub title: String,

    pub prices: Vec<LabeledPrice>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct SuccessfulPayment {
    pub currency: String,

    pub total_amount: u32,

    pub invoice_payload: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,

    pub telegram_payment_charge_id: String,

    pub provider_payment_charge_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct ShippingQuery {
    pub id: String,

    pub from: User,

    pub invoice_payload: String,

    pub shipping_address: ShippingAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct PreCheckoutQuery {
    pub id: String,

    pub from: User,

    pub currency: String,

    pub total_amount: u32,

    pub invoice_payload: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,

    pub credentials: EncryptedCredentials,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct PassportFile {
    pub file_id: String,

    pub file_unique_id: String,

    pub file_size: u32,

    pub file_date: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct EncryptedPassportElement {
    #[serde(rename = "type")]
    pub type_field: EncryptedPassportElementType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
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

    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct EncryptedCredentials {
    pub data: String,

    pub hash: String,

    pub secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct PassportElementErrorDataField {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorDataFieldType,

    pub field_name: String,

    pub data_hash: String,

    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct PassportElementErrorFrontSide {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorFrontSideType,

    pub file_hash: String,

    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct PassportElementErrorReverseSide {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorReverseSideType,

    pub file_hash: String,

    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct PassportElementErrorSelfie {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorSelfieType,

    pub file_hash: String,

    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct PassportElementErrorFile {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorFileType,

    pub file_hash: String,

    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct PassportElementErrorFiles {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorFileType,

    pub file_hashes: Vec<String>,

    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct PassportElementErrorTranslationFile {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorTranslationFileType,

    pub file_hash: String,

    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct PassportElementErrorTranslationFiles {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorTranslationFileType,

    pub file_hashes: Vec<String>,

    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct PassportElementErrorUnspecified {
    #[serde(rename = "type")]
    pub type_field: EncryptedPassportElementType,

    pub element_hash: String,

    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct Game {
    pub title: String,

    pub description: String,

    pub photo: Vec<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct GameHighScore {
    pub position: u32,

    pub user: User,

    pub score: i32,
}
