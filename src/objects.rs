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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberOwner {
    pub user: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,

    pub is_anonymous: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberMember {
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberLeft {
    pub user: User,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberBanned {
    pub user: User,

    pub until_date: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VoiceChatStarted {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VoiceChatScheduled {
    pub start_date: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallbackGame {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageId {
    pub message_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PhotoSize {
    pub file_id: String,

    pub file_unique_id: String,

    pub width: u32,

    pub height: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Voice {
    pub file_id: String,

    pub file_unique_id: String,

    pub duration: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dice {
    pub emoji: String,

    pub value: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PollOption {
    pub text: String,

    pub voter_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PollAnswer {
    pub poll_id: String,

    pub user: User,

    pub option_ids: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProximityAlertTriggered {
    pub traveler: User,

    pub watcher: User,

    pub distance: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageAutoDeleteTimerChanged {
    pub message_auto_delete_time: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VoiceChatEnded {
    pub duration: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VoiceChatParticipantsInvited {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserProfilePhotos {
    pub total_count: u32,

    pub photos: Vec<Vec<PhotoSize>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct File {
    pub file_id: String,

    pub file_unique_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyboardButton {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<KeyboardButtonPollType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyboardButtonPollType {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<PollType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoginUrl {
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_write_access: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForceReply {
    pub force_reply: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatPhoto {
    pub small_file_id: String,

    pub small_file_unique_id: String,

    pub big_file_id: String,

    pub big_file_unique_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberUpdated {
    pub chat: Chat,

    pub from: User,

    pub date: u64,

    pub old_chat_member: ChatMember,

    pub new_chat_member: ChatMember,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatJoinRequest {
    pub chat: Chat,

    pub from: User,

    pub date: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatLocation {
    pub location: Location,

    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotCommand {
    pub command: String,

    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StickerSet {
    pub name: String,

    pub title: String,

    pub is_animated: bool,

    pub contains_masks: bool,

    pub stickers: Vec<Sticker>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MaskPosition {
    pub point: String,

    pub x_shift: f64,

    pub y_shift: f64,

    pub scale: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultGame {
    pub id: String,

    pub game_short_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultCachedSticker {
    pub id: String,

    pub sticker_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputTextMessageContent {
    pub message_text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputContactMessageContent {
    pub phone_number: String,

    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChosenInlineResult {
    pub result_id: String,

    pub from: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LabeledPrice {
    pub label: String,

    pub amount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Invoice {
    pub title: String,

    pub description: String,

    pub start_parameter: String,

    pub currency: String,

    pub total_amount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShippingAddress {
    pub country_code: String,

    pub state: String,

    pub city: String,

    pub street_line1: String,

    pub street_line2: String,

    pub post_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShippingOption {
    pub id: String,

    pub title: String,

    pub prices: Vec<LabeledPrice>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShippingQuery {
    pub id: String,

    pub from: User,

    pub invoice_payload: String,

    pub shipping_address: ShippingAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,

    pub credentials: EncryptedCredentials,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportFile {
    pub file_id: String,

    pub file_unique_id: String,

    pub file_size: u32,

    pub file_date: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EncryptedCredentials {
    pub data: String,

    pub hash: String,

    pub secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorDataField {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorDataFieldType,

    pub field_name: String,

    pub data_hash: String,

    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorFrontSide {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorFrontSideType,

    pub file_hash: String,

    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorReverseSide {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorReverseSideType,

    pub file_hash: String,

    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorSelfie {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorSelfieType,

    pub file_hash: String,

    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorFile {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorFileType,

    pub file_hash: String,

    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorFiles {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorFileType,

    pub file_hashes: Vec<String>,

    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorTranslationFile {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorTranslationFileType,

    pub file_hash: String,

    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorTranslationFiles {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorTranslationFileType,

    pub file_hashes: Vec<String>,

    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorUnspecified {
    #[serde(rename = "type")]
    pub type_field: EncryptedPassportElementType,

    pub element_hash: String,

    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameHighScore {
    pub position: u32,

    pub user: User,

    pub score: i32,
}

impl Update {
    pub fn new(update_id: u32) -> Self {
        Self {
            update_id,
            message: None,
            edited_message: None,
            channel_post: None,
            edited_channel_post: None,
            inline_query: None,
            chosen_inline_result: None,
            callback_query: None,
            shipping_query: None,
            pre_checkout_query: None,
            poll: None,
            poll_answer: None,
            my_chat_member: None,
            chat_member: None,
            chat_join_request: None,
        }
    }

    pub fn set_update_id(&mut self, update_id: u32) {
        self.update_id = update_id;
    }

    pub fn set_message(&mut self, message: Option<Message>) {
        self.message = message;
    }

    pub fn set_edited_message(&mut self, edited_message: Option<Message>) {
        self.edited_message = edited_message;
    }

    pub fn set_channel_post(&mut self, channel_post: Option<Message>) {
        self.channel_post = channel_post;
    }

    pub fn set_edited_channel_post(&mut self, edited_channel_post: Option<Message>) {
        self.edited_channel_post = edited_channel_post;
    }

    pub fn set_inline_query(&mut self, inline_query: Option<InlineQuery>) {
        self.inline_query = inline_query;
    }

    pub fn set_chosen_inline_result(&mut self, chosen_inline_result: Option<ChosenInlineResult>) {
        self.chosen_inline_result = chosen_inline_result;
    }

    pub fn set_callback_query(&mut self, callback_query: Option<CallbackQuery>) {
        self.callback_query = callback_query;
    }

    pub fn set_shipping_query(&mut self, shipping_query: Option<ShippingQuery>) {
        self.shipping_query = shipping_query;
    }

    pub fn set_pre_checkout_query(&mut self, pre_checkout_query: Option<PreCheckoutQuery>) {
        self.pre_checkout_query = pre_checkout_query;
    }

    pub fn set_poll(&mut self, poll: Option<Poll>) {
        self.poll = poll;
    }

    pub fn set_poll_answer(&mut self, poll_answer: Option<PollAnswer>) {
        self.poll_answer = poll_answer;
    }

    pub fn set_my_chat_member(&mut self, my_chat_member: Option<ChatMemberUpdated>) {
        self.my_chat_member = my_chat_member;
    }

    pub fn set_chat_member(&mut self, chat_member: Option<ChatMemberUpdated>) {
        self.chat_member = chat_member;
    }

    pub fn set_chat_join_request(&mut self, chat_join_request: Option<ChatJoinRequest>) {
        self.chat_join_request = chat_join_request;
    }

    pub fn update_id(&self) -> u32 {
        self.update_id
    }

    pub fn message(&self) -> Option<Message> {
        self.message.clone()
    }

    pub fn edited_message(&self) -> Option<Message> {
        self.edited_message.clone()
    }

    pub fn channel_post(&self) -> Option<Message> {
        self.channel_post.clone()
    }

    pub fn edited_channel_post(&self) -> Option<Message> {
        self.edited_channel_post.clone()
    }

    pub fn inline_query(&self) -> Option<InlineQuery> {
        self.inline_query.clone()
    }

    pub fn chosen_inline_result(&self) -> Option<ChosenInlineResult> {
        self.chosen_inline_result.clone()
    }

    pub fn callback_query(&self) -> Option<CallbackQuery> {
        self.callback_query.clone()
    }

    pub fn shipping_query(&self) -> Option<ShippingQuery> {
        self.shipping_query.clone()
    }

    pub fn pre_checkout_query(&self) -> Option<PreCheckoutQuery> {
        self.pre_checkout_query.clone()
    }

    pub fn poll(&self) -> Option<Poll> {
        self.poll.clone()
    }

    pub fn poll_answer(&self) -> Option<PollAnswer> {
        self.poll_answer.clone()
    }

    pub fn my_chat_member(&self) -> Option<ChatMemberUpdated> {
        self.my_chat_member.clone()
    }

    pub fn chat_member(&self) -> Option<ChatMemberUpdated> {
        self.chat_member.clone()
    }

    pub fn chat_join_request(&self) -> Option<ChatJoinRequest> {
        self.chat_join_request.clone()
    }
}

impl WebhookInfo {
    pub fn new(url: String, has_custom_certificate: bool, pending_update_count: u32) -> Self {
        Self {
            url,
            has_custom_certificate,
            pending_update_count,
            ip_address: None,
            last_error_date: None,
            last_error_message: None,
            max_connections: None,
            allowed_updates: None,
        }
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn set_has_custom_certificate(&mut self, has_custom_certificate: bool) {
        self.has_custom_certificate = has_custom_certificate;
    }

    pub fn set_pending_update_count(&mut self, pending_update_count: u32) {
        self.pending_update_count = pending_update_count;
    }

    pub fn set_ip_address(&mut self, ip_address: Option<String>) {
        self.ip_address = ip_address;
    }

    pub fn set_last_error_date(&mut self, last_error_date: Option<u64>) {
        self.last_error_date = last_error_date;
    }

    pub fn set_last_error_message(&mut self, last_error_message: Option<String>) {
        self.last_error_message = last_error_message;
    }

    pub fn set_max_connections(&mut self, max_connections: Option<u16>) {
        self.max_connections = max_connections;
    }

    pub fn set_allowed_updates(&mut self, allowed_updates: Option<Vec<String>>) {
        self.allowed_updates = allowed_updates;
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn has_custom_certificate(&self) -> bool {
        self.has_custom_certificate
    }

    pub fn pending_update_count(&self) -> u32 {
        self.pending_update_count
    }

    pub fn ip_address(&self) -> Option<String> {
        self.ip_address.clone()
    }

    pub fn last_error_date(&self) -> Option<u64> {
        self.last_error_date
    }

    pub fn last_error_message(&self) -> Option<String> {
        self.last_error_message.clone()
    }

    pub fn max_connections(&self) -> Option<u16> {
        self.max_connections
    }

    pub fn allowed_updates(&self) -> Option<Vec<String>> {
        self.allowed_updates.clone()
    }
}

impl User {
    pub fn new(id: u64, is_bot: bool, first_name: String) -> Self {
        Self {
            id,
            is_bot,
            first_name,
            last_name: None,
            username: None,
            language_code: None,
            can_join_groups: None,
            can_read_all_group_messages: None,
            supports_inline_queries: None,
        }
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    pub fn set_is_bot(&mut self, is_bot: bool) {
        self.is_bot = is_bot;
    }

    pub fn set_first_name(&mut self, first_name: String) {
        self.first_name = first_name;
    }

    pub fn set_last_name(&mut self, last_name: Option<String>) {
        self.last_name = last_name;
    }

    pub fn set_username(&mut self, username: Option<String>) {
        self.username = username;
    }

    pub fn set_language_code(&mut self, language_code: Option<String>) {
        self.language_code = language_code;
    }

    pub fn set_can_join_groups(&mut self, can_join_groups: Option<bool>) {
        self.can_join_groups = can_join_groups;
    }

    pub fn set_can_read_all_group_messages(&mut self, can_read_all_group_messages: Option<bool>) {
        self.can_read_all_group_messages = can_read_all_group_messages;
    }

    pub fn set_supports_inline_queries(&mut self, supports_inline_queries: Option<bool>) {
        self.supports_inline_queries = supports_inline_queries;
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn is_bot(&self) -> bool {
        self.is_bot
    }

    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn last_name(&self) -> Option<String> {
        self.last_name.clone()
    }

    pub fn username(&self) -> Option<String> {
        self.username.clone()
    }

    pub fn language_code(&self) -> Option<String> {
        self.language_code.clone()
    }

    pub fn can_join_groups(&self) -> Option<bool> {
        self.can_join_groups
    }

    pub fn can_read_all_group_messages(&self) -> Option<bool> {
        self.can_read_all_group_messages
    }

    pub fn supports_inline_queries(&self) -> Option<bool> {
        self.supports_inline_queries
    }
}

impl Chat {
    pub fn new(id: i64, type_field: ChatType) -> Self {
        Self {
            id,
            type_field,
            title: None,
            username: None,
            first_name: None,
            last_name: None,
            photo: None,
            bio: None,
            has_private_forwards: None,
            description: None,
            invite_link: None,
            pinned_message: None,
            permissions: None,
            slow_mode_delay: None,
            message_auto_delete_time: None,
            has_protected_content: None,
            sticker_set_name: None,
            can_set_sticker_set: None,
            linked_chat_id: None,
            location: None,
        }
    }

    pub fn set_id(&mut self, id: i64) {
        self.id = id;
    }

    pub fn set_type_field(&mut self, type_field: ChatType) {
        self.type_field = type_field;
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    pub fn set_username(&mut self, username: Option<String>) {
        self.username = username;
    }

    pub fn set_first_name(&mut self, first_name: Option<String>) {
        self.first_name = first_name;
    }

    pub fn set_last_name(&mut self, last_name: Option<String>) {
        self.last_name = last_name;
    }

    pub fn set_photo(&mut self, photo: Option<ChatPhoto>) {
        self.photo = photo;
    }

    pub fn set_bio(&mut self, bio: Option<String>) {
        self.bio = bio;
    }

    pub fn set_has_private_forwards(&mut self, has_private_forwards: Option<bool>) {
        self.has_private_forwards = has_private_forwards;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn set_invite_link(&mut self, invite_link: Option<String>) {
        self.invite_link = invite_link;
    }

    pub fn set_pinned_message(&mut self, pinned_message: Option<Box<Message>>) {
        self.pinned_message = pinned_message;
    }

    pub fn set_permissions(&mut self, permissions: Option<ChatPermissions>) {
        self.permissions = permissions;
    }

    pub fn set_slow_mode_delay(&mut self, slow_mode_delay: Option<u16>) {
        self.slow_mode_delay = slow_mode_delay;
    }

    pub fn set_message_auto_delete_time(&mut self, message_auto_delete_time: Option<u32>) {
        self.message_auto_delete_time = message_auto_delete_time;
    }

    pub fn set_has_protected_content(&mut self, has_protected_content: Option<bool>) {
        self.has_protected_content = has_protected_content;
    }

    pub fn set_sticker_set_name(&mut self, sticker_set_name: Option<String>) {
        self.sticker_set_name = sticker_set_name;
    }

    pub fn set_can_set_sticker_set(&mut self, can_set_sticker_set: Option<bool>) {
        self.can_set_sticker_set = can_set_sticker_set;
    }

    pub fn set_linked_chat_id(&mut self, linked_chat_id: Option<i64>) {
        self.linked_chat_id = linked_chat_id;
    }

    pub fn set_location(&mut self, location: Option<ChatLocation>) {
        self.location = location;
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn type_field(&self) -> ChatType {
        self.type_field.clone()
    }

    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }

    pub fn username(&self) -> Option<String> {
        self.username.clone()
    }

    pub fn first_name(&self) -> Option<String> {
        self.first_name.clone()
    }

    pub fn last_name(&self) -> Option<String> {
        self.last_name.clone()
    }

    pub fn photo(&self) -> Option<ChatPhoto> {
        self.photo.clone()
    }

    pub fn bio(&self) -> Option<String> {
        self.bio.clone()
    }

    pub fn has_private_forwards(&self) -> Option<bool> {
        self.has_private_forwards
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn invite_link(&self) -> Option<String> {
        self.invite_link.clone()
    }

    pub fn pinned_message(&self) -> Option<Box<Message>> {
        self.pinned_message.clone()
    }

    pub fn permissions(&self) -> Option<ChatPermissions> {
        self.permissions.clone()
    }

    pub fn slow_mode_delay(&self) -> Option<u16> {
        self.slow_mode_delay
    }

    pub fn message_auto_delete_time(&self) -> Option<u32> {
        self.message_auto_delete_time
    }

    pub fn has_protected_content(&self) -> Option<bool> {
        self.has_protected_content
    }

    pub fn sticker_set_name(&self) -> Option<String> {
        self.sticker_set_name.clone()
    }

    pub fn can_set_sticker_set(&self) -> Option<bool> {
        self.can_set_sticker_set
    }

    pub fn linked_chat_id(&self) -> Option<i64> {
        self.linked_chat_id
    }

    pub fn location(&self) -> Option<ChatLocation> {
        self.location.clone()
    }
}

impl Message {
    pub fn new(message_id: i32, date: u64, chat: Chat) -> Self {
        Self {
            message_id,
            date,
            chat,
            from: None,
            sender_chat: None,
            forward_from: None,
            forward_from_chat: None,
            forward_from_message_id: None,
            forward_signature: None,
            forward_sender_name: None,
            forward_date: None,
            is_automatic_forward: None,
            reply_to_message: None,
            via_bot: None,
            edit_date: None,
            has_protected_content: None,
            media_group_id: None,
            author_signature: None,
            text: None,
            entities: None,
            animation: None,
            audio: None,
            document: None,
            photo: None,
            sticker: None,
            video: None,
            video_note: None,
            voice: None,
            caption: None,
            caption_entities: None,
            contact: None,
            dice: None,
            game: None,
            poll: None,
            venue: None,
            location: None,
            new_chat_members: None,
            left_chat_member: None,
            new_chat_title: None,
            new_chat_photo: None,
            delete_chat_photo: None,
            group_chat_created: None,
            supergroup_chat_created: None,
            channel_chat_created: None,
            message_auto_delete_timer_changed: None,
            migrate_to_chat_id: None,
            migrate_from_chat_id: None,
            pinned_message: None,
            invoice: None,
            successful_payment: None,
            connected_website: None,
            passport_data: None,
            proximity_alert_triggered: None,
            voice_chat_started: None,
            voice_chat_ended: None,
            voice_chat_scheduled: None,
            voice_chat_participants_invited: None,
            reply_markup: None,
        }
    }

    pub fn set_message_id(&mut self, message_id: i32) {
        self.message_id = message_id;
    }

    pub fn set_date(&mut self, date: u64) {
        self.date = date;
    }

    pub fn set_chat(&mut self, chat: Chat) {
        self.chat = chat;
    }

    pub fn set_from(&mut self, from: Option<User>) {
        self.from = from;
    }

    pub fn set_sender_chat(&mut self, sender_chat: Option<Chat>) {
        self.sender_chat = sender_chat;
    }

    pub fn set_forward_from(&mut self, forward_from: Option<User>) {
        self.forward_from = forward_from;
    }

    pub fn set_forward_from_chat(&mut self, forward_from_chat: Option<Chat>) {
        self.forward_from_chat = forward_from_chat;
    }

    pub fn set_forward_from_message_id(&mut self, forward_from_message_id: Option<i32>) {
        self.forward_from_message_id = forward_from_message_id;
    }

    pub fn set_forward_signature(&mut self, forward_signature: Option<String>) {
        self.forward_signature = forward_signature;
    }

    pub fn set_forward_sender_name(&mut self, forward_sender_name: Option<String>) {
        self.forward_sender_name = forward_sender_name;
    }

    pub fn set_forward_date(&mut self, forward_date: Option<u64>) {
        self.forward_date = forward_date;
    }

    pub fn set_is_automatic_forward(&mut self, is_automatic_forward: Option<bool>) {
        self.is_automatic_forward = is_automatic_forward;
    }

    pub fn set_reply_to_message(&mut self, reply_to_message: Option<Box<Message>>) {
        self.reply_to_message = reply_to_message;
    }

    pub fn set_via_bot(&mut self, via_bot: Option<User>) {
        self.via_bot = via_bot;
    }

    pub fn set_edit_date(&mut self, edit_date: Option<u64>) {
        self.edit_date = edit_date;
    }

    pub fn set_has_protected_content(&mut self, has_protected_content: Option<bool>) {
        self.has_protected_content = has_protected_content;
    }

    pub fn set_media_group_id(&mut self, media_group_id: Option<String>) {
        self.media_group_id = media_group_id;
    }

    pub fn set_author_signature(&mut self, author_signature: Option<String>) {
        self.author_signature = author_signature;
    }

    pub fn set_text(&mut self, text: Option<String>) {
        self.text = text;
    }

    pub fn set_entities(&mut self, entities: Option<Vec<MessageEntity>>) {
        self.entities = entities;
    }

    pub fn set_animation(&mut self, animation: Option<Animation>) {
        self.animation = animation;
    }

    pub fn set_audio(&mut self, audio: Option<Audio>) {
        self.audio = audio;
    }

    pub fn set_document(&mut self, document: Option<Document>) {
        self.document = document;
    }

    pub fn set_photo(&mut self, photo: Option<Vec<PhotoSize>>) {
        self.photo = photo;
    }

    pub fn set_sticker(&mut self, sticker: Option<Sticker>) {
        self.sticker = sticker;
    }

    pub fn set_video(&mut self, video: Option<Video>) {
        self.video = video;
    }

    pub fn set_video_note(&mut self, video_note: Option<VideoNote>) {
        self.video_note = video_note;
    }

    pub fn set_voice(&mut self, voice: Option<Voice>) {
        self.voice = voice;
    }

    pub fn set_caption(&mut self, caption: Option<String>) {
        self.caption = caption;
    }

    pub fn set_caption_entities(&mut self, caption_entities: Option<Vec<MessageEntity>>) {
        self.caption_entities = caption_entities;
    }

    pub fn set_contact(&mut self, contact: Option<Contact>) {
        self.contact = contact;
    }

    pub fn set_dice(&mut self, dice: Option<Dice>) {
        self.dice = dice;
    }

    pub fn set_game(&mut self, game: Option<Game>) {
        self.game = game;
    }

    pub fn set_poll(&mut self, poll: Option<Poll>) {
        self.poll = poll;
    }

    pub fn set_venue(&mut self, venue: Option<Venue>) {
        self.venue = venue;
    }

    pub fn set_location(&mut self, location: Option<Location>) {
        self.location = location;
    }

    pub fn set_new_chat_members(&mut self, new_chat_members: Option<Vec<User>>) {
        self.new_chat_members = new_chat_members;
    }

    pub fn set_left_chat_member(&mut self, left_chat_member: Option<User>) {
        self.left_chat_member = left_chat_member;
    }

    pub fn set_new_chat_title(&mut self, new_chat_title: Option<String>) {
        self.new_chat_title = new_chat_title;
    }

    pub fn set_new_chat_photo(&mut self, new_chat_photo: Option<Vec<PhotoSize>>) {
        self.new_chat_photo = new_chat_photo;
    }

    pub fn set_delete_chat_photo(&mut self, delete_chat_photo: Option<bool>) {
        self.delete_chat_photo = delete_chat_photo;
    }

    pub fn set_group_chat_created(&mut self, group_chat_created: Option<bool>) {
        self.group_chat_created = group_chat_created;
    }

    pub fn set_supergroup_chat_created(&mut self, supergroup_chat_created: Option<bool>) {
        self.supergroup_chat_created = supergroup_chat_created;
    }

    pub fn set_channel_chat_created(&mut self, channel_chat_created: Option<bool>) {
        self.channel_chat_created = channel_chat_created;
    }

    pub fn set_message_auto_delete_timer_changed(
        &mut self,
        message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    ) {
        self.message_auto_delete_timer_changed = message_auto_delete_timer_changed;
    }

    pub fn set_migrate_to_chat_id(&mut self, migrate_to_chat_id: Option<i64>) {
        self.migrate_to_chat_id = migrate_to_chat_id;
    }

    pub fn set_migrate_from_chat_id(&mut self, migrate_from_chat_id: Option<i64>) {
        self.migrate_from_chat_id = migrate_from_chat_id;
    }

    pub fn set_pinned_message(&mut self, pinned_message: Option<Box<Message>>) {
        self.pinned_message = pinned_message;
    }

    pub fn set_invoice(&mut self, invoice: Option<Invoice>) {
        self.invoice = invoice;
    }

    pub fn set_successful_payment(&mut self, successful_payment: Option<SuccessfulPayment>) {
        self.successful_payment = successful_payment;
    }

    pub fn set_connected_website(&mut self, connected_website: Option<String>) {
        self.connected_website = connected_website;
    }

    pub fn set_passport_data(&mut self, passport_data: Option<PassportData>) {
        self.passport_data = passport_data;
    }

    pub fn set_proximity_alert_triggered(
        &mut self,
        proximity_alert_triggered: Option<ProximityAlertTriggered>,
    ) {
        self.proximity_alert_triggered = proximity_alert_triggered;
    }

    pub fn set_voice_chat_started(&mut self, voice_chat_started: Option<VoiceChatStarted>) {
        self.voice_chat_started = voice_chat_started;
    }

    pub fn set_voice_chat_ended(&mut self, voice_chat_ended: Option<VoiceChatEnded>) {
        self.voice_chat_ended = voice_chat_ended;
    }

    pub fn set_voice_chat_participants_invited(
        &mut self,
        voice_chat_participants_invited: Option<VoiceChatParticipantsInvited>,
    ) {
        self.voice_chat_participants_invited = voice_chat_participants_invited;
    }

    pub fn set_voice_chat_scheduled(&mut self, voice_chat_scheduled: Option<VoiceChatScheduled>) {
        self.voice_chat_scheduled = voice_chat_scheduled;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<InlineKeyboardMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn voice_chat_scheduled(&self) -> Option<VoiceChatScheduled> {
        self.voice_chat_scheduled.clone()
    }

    pub fn message_id(&self) -> i32 {
        self.message_id
    }

    pub fn date(&self) -> u64 {
        self.date
    }

    pub fn chat(&self) -> Chat {
        self.chat.clone()
    }

    pub fn from(&self) -> Option<User> {
        self.from.clone()
    }

    pub fn sender_chat(&self) -> Option<Chat> {
        self.sender_chat.clone()
    }

    pub fn forward_from(&self) -> Option<User> {
        self.forward_from.clone()
    }

    pub fn forward_from_chat(&self) -> Option<Chat> {
        self.forward_from_chat.clone()
    }

    pub fn forward_from_message_id(&self) -> Option<i32> {
        self.forward_from_message_id
    }

    pub fn forward_signature(&self) -> Option<String> {
        self.forward_signature.clone()
    }

    pub fn forward_sender_name(&self) -> Option<String> {
        self.forward_sender_name.clone()
    }

    pub fn forward_date(&self) -> Option<u64> {
        self.forward_date
    }

    pub fn is_automatic_forward(&mut self) -> Option<bool> {
        self.is_automatic_forward
    }

    pub fn reply_to_message(&self) -> Option<Box<Message>> {
        self.reply_to_message.clone()
    }

    pub fn via_bot(&self) -> Option<User> {
        self.via_bot.clone()
    }

    pub fn edit_date(&self) -> Option<u64> {
        self.edit_date
    }

    pub fn has_protected_content(&self) -> Option<bool> {
        self.has_protected_content
    }

    pub fn media_group_id(&self) -> Option<String> {
        self.media_group_id.clone()
    }

    pub fn author_signature(&self) -> Option<String> {
        self.author_signature.clone()
    }

    pub fn text(&self) -> Option<String> {
        self.text.clone()
    }

    pub fn entities(&self) -> Option<Vec<MessageEntity>> {
        self.entities.clone()
    }

    pub fn animation(&self) -> Option<Animation> {
        self.animation.clone()
    }

    pub fn audio(&self) -> Option<Audio> {
        self.audio.clone()
    }

    pub fn document(&self) -> Option<Document> {
        self.document.clone()
    }

    pub fn photo(&self) -> Option<Vec<PhotoSize>> {
        self.photo.clone()
    }

    pub fn sticker(&self) -> Option<Sticker> {
        self.sticker.clone()
    }

    pub fn video(&self) -> Option<Video> {
        self.video.clone()
    }

    pub fn video_note(&self) -> Option<VideoNote> {
        self.video_note.clone()
    }

    pub fn voice(&self) -> Option<Voice> {
        self.voice.clone()
    }

    pub fn caption(&self) -> Option<String> {
        self.caption.clone()
    }

    pub fn caption_entities(&self) -> Option<Vec<MessageEntity>> {
        self.caption_entities.clone()
    }

    pub fn contact(&self) -> Option<Contact> {
        self.contact.clone()
    }

    pub fn dice(&self) -> Option<Dice> {
        self.dice.clone()
    }

    pub fn game(&self) -> Option<Game> {
        self.game.clone()
    }

    pub fn poll(&self) -> Option<Poll> {
        self.poll.clone()
    }

    pub fn venue(&self) -> Option<Venue> {
        self.venue.clone()
    }

    pub fn location(&self) -> Option<Location> {
        self.location.clone()
    }

    pub fn new_chat_members(&self) -> Option<Vec<User>> {
        self.new_chat_members.clone()
    }

    pub fn left_chat_member(&self) -> Option<User> {
        self.left_chat_member.clone()
    }

    pub fn new_chat_title(&self) -> Option<String> {
        self.new_chat_title.clone()
    }

    pub fn new_chat_photo(&self) -> Option<Vec<PhotoSize>> {
        self.new_chat_photo.clone()
    }

    pub fn delete_chat_photo(&self) -> Option<bool> {
        self.delete_chat_photo
    }

    pub fn group_chat_created(&self) -> Option<bool> {
        self.group_chat_created
    }

    pub fn supergroup_chat_created(&self) -> Option<bool> {
        self.supergroup_chat_created
    }

    pub fn channel_chat_created(&self) -> Option<bool> {
        self.channel_chat_created
    }

    pub fn message_auto_delete_timer_changed(&self) -> Option<MessageAutoDeleteTimerChanged> {
        self.message_auto_delete_timer_changed.clone()
    }

    pub fn migrate_to_chat_id(&self) -> Option<i64> {
        self.migrate_to_chat_id
    }

    pub fn migrate_from_chat_id(&self) -> Option<i64> {
        self.migrate_from_chat_id
    }

    pub fn pinned_message(&self) -> Option<Box<Message>> {
        self.pinned_message.clone()
    }

    pub fn invoice(&self) -> Option<Invoice> {
        self.invoice.clone()
    }

    pub fn successful_payment(&self) -> Option<SuccessfulPayment> {
        self.successful_payment.clone()
    }

    pub fn connected_website(&self) -> Option<String> {
        self.connected_website.clone()
    }

    pub fn passport_data(&self) -> Option<PassportData> {
        self.passport_data.clone()
    }

    pub fn proximity_alert_triggered(&self) -> Option<ProximityAlertTriggered> {
        self.proximity_alert_triggered.clone()
    }

    pub fn voice_chat_started(&self) -> Option<VoiceChatStarted> {
        self.voice_chat_started.clone()
    }

    pub fn voice_chat_ended(&self) -> Option<VoiceChatEnded> {
        self.voice_chat_ended.clone()
    }

    pub fn voice_chat_participants_invited(&self) -> Option<VoiceChatParticipantsInvited> {
        self.voice_chat_participants_invited.clone()
    }

    pub fn reply_markup(&self) -> Option<InlineKeyboardMarkup> {
        self.reply_markup.clone()
    }
}

impl MessageId {
    pub fn new(message_id: i32) -> Self {
        Self { message_id }
    }

    pub fn set_message_id(&mut self, message_id: i32) {
        self.message_id = message_id;
    }

    pub fn message_id(&self) -> i32 {
        self.message_id
    }
}

impl MessageEntity {
    pub fn new(type_field: MessageEntityType, offset: u16, length: u16) -> Self {
        Self {
            type_field,
            offset,
            length,
            url: None,
            user: None,
            language: None,
        }
    }

    pub fn set_type_field(&mut self, type_field: MessageEntityType) {
        self.type_field = type_field;
    }

    pub fn set_offset(&mut self, offset: u16) {
        self.offset = offset;
    }

    pub fn set_length(&mut self, length: u16) {
        self.length = length;
    }

    pub fn set_url(&mut self, url: Option<String>) {
        self.url = url;
    }

    pub fn set_user(&mut self, user: Option<User>) {
        self.user = user;
    }

    pub fn set_language(&mut self, language: Option<String>) {
        self.language = language;
    }

    pub fn type_field(&self) -> MessageEntityType {
        self.type_field.clone()
    }

    pub fn offset(&self) -> u16 {
        self.offset
    }

    pub fn length(&self) -> u16 {
        self.length
    }

    pub fn url(&self) -> Option<String> {
        self.url.clone()
    }

    pub fn user(&self) -> Option<User> {
        self.user.clone()
    }

    pub fn language(&self) -> Option<String> {
        self.language.clone()
    }
}

impl PhotoSize {
    pub fn new(file_id: String, file_unique_id: String, width: u32, height: u32) -> Self {
        Self {
            file_id,
            file_unique_id,
            width,
            height,
            file_size: None,
        }
    }

    pub fn set_file_id(&mut self, file_id: String) {
        self.file_id = file_id;
    }

    pub fn set_file_unique_id(&mut self, file_unique_id: String) {
        self.file_unique_id = file_unique_id;
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn set_file_size(&mut self, file_size: Option<u32>) {
        self.file_size = file_size;
    }

    pub fn file_id(&self) -> String {
        self.file_id.clone()
    }

    pub fn file_unique_id(&self) -> String {
        self.file_unique_id.clone()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn file_size(&self) -> Option<u32> {
        self.file_size
    }
}

impl Animation {
    pub fn new(
        file_id: String,
        file_unique_id: String,
        width: u32,
        height: u32,
        duration: u32,
    ) -> Self {
        Self {
            file_id,
            file_unique_id,
            width,
            height,
            duration,
            thumb: None,
            file_name: None,
            mime_type: None,
            file_size: None,
        }
    }

    pub fn set_file_id(&mut self, file_id: String) {
        self.file_id = file_id;
    }

    pub fn set_file_unique_id(&mut self, file_unique_id: String) {
        self.file_unique_id = file_unique_id;
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn set_duration(&mut self, duration: u32) {
        self.duration = duration;
    }

    pub fn set_thumb(&mut self, thumb: Option<PhotoSize>) {
        self.thumb = thumb;
    }

    pub fn set_file_name(&mut self, file_name: Option<String>) {
        self.file_name = file_name;
    }

    pub fn set_mime_type(&mut self, mime_type: Option<String>) {
        self.mime_type = mime_type;
    }

    pub fn set_file_size(&mut self, file_size: Option<u32>) {
        self.file_size = file_size;
    }

    pub fn file_id(&self) -> String {
        self.file_id.clone()
    }

    pub fn file_unique_id(&self) -> String {
        self.file_unique_id.clone()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn duration(&self) -> u32 {
        self.duration
    }

    pub fn thumb(&self) -> Option<PhotoSize> {
        self.thumb.clone()
    }

    pub fn file_name(&self) -> Option<String> {
        self.file_name.clone()
    }

    pub fn mime_type(&self) -> Option<String> {
        self.mime_type.clone()
    }

    pub fn file_size(&self) -> Option<u32> {
        self.file_size
    }
}

impl Audio {
    pub fn new(file_id: String, file_unique_id: String, duration: u32) -> Self {
        Self {
            file_id,
            file_unique_id,
            duration,
            performer: None,
            title: None,
            file_name: None,
            mime_type: None,
            file_size: None,
            thumb: None,
        }
    }

    pub fn set_file_id(&mut self, file_id: String) {
        self.file_id = file_id;
    }

    pub fn set_file_unique_id(&mut self, file_unique_id: String) {
        self.file_unique_id = file_unique_id;
    }

    pub fn set_duration(&mut self, duration: u32) {
        self.duration = duration;
    }

    pub fn set_performer(&mut self, performer: Option<String>) {
        self.performer = performer;
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    pub fn set_file_name(&mut self, file_name: Option<String>) {
        self.file_name = file_name;
    }

    pub fn set_mime_type(&mut self, mime_type: Option<String>) {
        self.mime_type = mime_type;
    }

    pub fn set_file_size(&mut self, file_size: Option<u32>) {
        self.file_size = file_size;
    }

    pub fn set_thumb(&mut self, thumb: Option<PhotoSize>) {
        self.thumb = thumb;
    }

    pub fn file_id(&self) -> String {
        self.file_id.clone()
    }

    pub fn file_unique_id(&self) -> String {
        self.file_unique_id.clone()
    }

    pub fn duration(&self) -> u32 {
        self.duration
    }

    pub fn performer(&self) -> Option<String> {
        self.performer.clone()
    }

    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }

    pub fn file_name(&self) -> Option<String> {
        self.file_name.clone()
    }

    pub fn mime_type(&self) -> Option<String> {
        self.mime_type.clone()
    }

    pub fn file_size(&self) -> Option<u32> {
        self.file_size
    }

    pub fn thumb(&self) -> Option<PhotoSize> {
        self.thumb.clone()
    }
}

impl Document {
    pub fn new(file_id: String, file_unique_id: String) -> Self {
        Self {
            file_id,
            file_unique_id,
            thumb: None,
            file_name: None,
            mime_type: None,
            file_size: None,
        }
    }

    pub fn set_file_id(&mut self, file_id: String) {
        self.file_id = file_id;
    }

    pub fn set_file_unique_id(&mut self, file_unique_id: String) {
        self.file_unique_id = file_unique_id;
    }

    pub fn set_thumb(&mut self, thumb: Option<PhotoSize>) {
        self.thumb = thumb;
    }

    pub fn set_file_name(&mut self, file_name: Option<String>) {
        self.file_name = file_name;
    }

    pub fn set_mime_type(&mut self, mime_type: Option<String>) {
        self.mime_type = mime_type;
    }

    pub fn set_file_size(&mut self, file_size: Option<u32>) {
        self.file_size = file_size;
    }

    pub fn file_id(&self) -> String {
        self.file_id.clone()
    }

    pub fn file_unique_id(&self) -> String {
        self.file_unique_id.clone()
    }

    pub fn thumb(&self) -> Option<PhotoSize> {
        self.thumb.clone()
    }

    pub fn file_name(&self) -> Option<String> {
        self.file_name.clone()
    }

    pub fn mime_type(&self) -> Option<String> {
        self.mime_type.clone()
    }

    pub fn file_size(&self) -> Option<u32> {
        self.file_size
    }
}

impl Video {
    pub fn new(
        file_id: String,
        file_unique_id: String,
        width: u32,
        height: u32,
        duration: u32,
    ) -> Self {
        Self {
            file_id,
            file_unique_id,
            width,
            height,
            duration,
            thumb: None,
            file_name: None,
            mime_type: None,
            file_size: None,
        }
    }

    pub fn set_file_id(&mut self, file_id: String) {
        self.file_id = file_id;
    }

    pub fn set_file_unique_id(&mut self, file_unique_id: String) {
        self.file_unique_id = file_unique_id;
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn set_duration(&mut self, duration: u32) {
        self.duration = duration;
    }

    pub fn set_thumb(&mut self, thumb: Option<PhotoSize>) {
        self.thumb = thumb;
    }

    pub fn set_file_name(&mut self, file_name: Option<String>) {
        self.file_name = file_name;
    }

    pub fn set_mime_type(&mut self, mime_type: Option<String>) {
        self.mime_type = mime_type;
    }

    pub fn set_file_size(&mut self, file_size: Option<u32>) {
        self.file_size = file_size;
    }

    pub fn file_id(&self) -> String {
        self.file_id.clone()
    }

    pub fn file_unique_id(&self) -> String {
        self.file_unique_id.clone()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn duration(&self) -> u32 {
        self.duration
    }

    pub fn thumb(&self) -> Option<PhotoSize> {
        self.thumb.clone()
    }

    pub fn file_name(&self) -> Option<String> {
        self.file_name.clone()
    }

    pub fn mime_type(&self) -> Option<String> {
        self.mime_type.clone()
    }

    pub fn file_size(&self) -> Option<u32> {
        self.file_size
    }
}

impl VideoNote {
    pub fn new(file_id: String, file_unique_id: String, length: u32, duration: u32) -> Self {
        Self {
            file_id,
            file_unique_id,
            length,
            duration,
            thumb: None,
            file_size: None,
        }
    }

    pub fn set_file_id(&mut self, file_id: String) {
        self.file_id = file_id;
    }

    pub fn set_file_unique_id(&mut self, file_unique_id: String) {
        self.file_unique_id = file_unique_id;
    }

    pub fn set_length(&mut self, length: u32) {
        self.length = length;
    }

    pub fn set_duration(&mut self, duration: u32) {
        self.duration = duration;
    }

    pub fn set_thumb(&mut self, thumb: Option<PhotoSize>) {
        self.thumb = thumb;
    }

    pub fn set_file_size(&mut self, file_size: Option<u32>) {
        self.file_size = file_size;
    }

    pub fn file_id(&self) -> String {
        self.file_id.clone()
    }

    pub fn file_unique_id(&self) -> String {
        self.file_unique_id.clone()
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn duration(&self) -> u32 {
        self.duration
    }

    pub fn thumb(&self) -> Option<PhotoSize> {
        self.thumb.clone()
    }

    pub fn file_size(&self) -> Option<u32> {
        self.file_size
    }
}

impl Voice {
    pub fn new(file_id: String, file_unique_id: String, duration: u32) -> Self {
        Self {
            file_id,
            file_unique_id,
            duration,
            mime_type: None,
            file_size: None,
        }
    }

    pub fn set_file_id(&mut self, file_id: String) {
        self.file_id = file_id;
    }

    pub fn set_file_unique_id(&mut self, file_unique_id: String) {
        self.file_unique_id = file_unique_id;
    }

    pub fn set_duration(&mut self, duration: u32) {
        self.duration = duration;
    }

    pub fn set_mime_type(&mut self, mime_type: Option<String>) {
        self.mime_type = mime_type;
    }

    pub fn set_file_size(&mut self, file_size: Option<u32>) {
        self.file_size = file_size;
    }

    pub fn file_id(&self) -> String {
        self.file_id.clone()
    }

    pub fn file_unique_id(&self) -> String {
        self.file_unique_id.clone()
    }

    pub fn duration(&self) -> u32 {
        self.duration
    }

    pub fn mime_type(&self) -> Option<String> {
        self.mime_type.clone()
    }

    pub fn file_size(&self) -> Option<u32> {
        self.file_size
    }
}

impl Contact {
    pub fn new(phone_number: String, first_name: String) -> Self {
        Self {
            phone_number,
            first_name,
            last_name: None,
            user_id: None,
            vcard: None,
        }
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

    pub fn set_user_id(&mut self, user_id: Option<u64>) {
        self.user_id = user_id;
    }

    pub fn set_vcard(&mut self, vcard: Option<String>) {
        self.vcard = vcard;
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

    pub fn user_id(&self) -> Option<u64> {
        self.user_id
    }

    pub fn vcard(&self) -> Option<String> {
        self.vcard.clone()
    }
}

impl Dice {
    pub fn new(emoji: String, value: u8) -> Self {
        Self { emoji, value }
    }

    pub fn set_emoji(&mut self, emoji: String) {
        self.emoji = emoji;
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }

    pub fn emoji(&self) -> String {
        self.emoji.clone()
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

impl PollOption {
    pub fn new(text: String, voter_count: u32) -> Self {
        Self { text, voter_count }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn set_voter_count(&mut self, voter_count: u32) {
        self.voter_count = voter_count;
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn voter_count(&self) -> u32 {
        self.voter_count
    }
}

impl PollAnswer {
    pub fn new(poll_id: String, user: User, option_ids: Vec<u8>) -> Self {
        Self {
            poll_id,
            user,
            option_ids,
        }
    }

    pub fn set_poll_id(&mut self, poll_id: String) {
        self.poll_id = poll_id;
    }

    pub fn set_user(&mut self, user: User) {
        self.user = user;
    }

    pub fn set_option_ids(&mut self, option_ids: Vec<u8>) {
        self.option_ids = option_ids;
    }

    pub fn poll_id(&self) -> String {
        self.poll_id.clone()
    }

    pub fn user(&self) -> User {
        self.user.clone()
    }

    pub fn option_ids(&self) -> Vec<u8> {
        self.option_ids.clone()
    }
}

impl Poll {
    pub fn new(
        id: String,
        question: String,
        options: Vec<PollOption>,
        total_voter_count: u32,
        is_closed: bool,
        is_anonymous: bool,
        type_field: PollType,
        allows_multiple_answers: bool,
    ) -> Self {
        Self {
            id,
            question,
            options,
            total_voter_count,
            is_closed,
            is_anonymous,
            type_field,
            allows_multiple_answers,
            correct_option_id: None,
            explanation: None,
            explanation_entities: None,
            open_period: None,
            close_date: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_question(&mut self, question: String) {
        self.question = question;
    }

    pub fn set_options(&mut self, options: Vec<PollOption>) {
        self.options = options;
    }

    pub fn set_total_voter_count(&mut self, total_voter_count: u32) {
        self.total_voter_count = total_voter_count;
    }

    pub fn set_is_closed(&mut self, is_closed: bool) {
        self.is_closed = is_closed;
    }

    pub fn set_is_anonymous(&mut self, is_anonymous: bool) {
        self.is_anonymous = is_anonymous;
    }

    pub fn set_type_field(&mut self, type_field: PollType) {
        self.type_field = type_field;
    }

    pub fn set_allows_multiple_answers(&mut self, allows_multiple_answers: bool) {
        self.allows_multiple_answers = allows_multiple_answers;
    }

    pub fn set_correct_option_id(&mut self, correct_option_id: Option<u8>) {
        self.correct_option_id = correct_option_id;
    }

    pub fn set_explanation(&mut self, explanation: Option<String>) {
        self.explanation = explanation;
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

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn question(&self) -> String {
        self.question.clone()
    }

    pub fn options(&self) -> Vec<PollOption> {
        self.options.clone()
    }

    pub fn total_voter_count(&self) -> u32 {
        self.total_voter_count
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    pub fn is_anonymous(&self) -> bool {
        self.is_anonymous
    }

    pub fn type_field(&self) -> PollType {
        self.type_field.clone()
    }

    pub fn allows_multiple_answers(&self) -> bool {
        self.allows_multiple_answers
    }

    pub fn correct_option_id(&self) -> Option<u8> {
        self.correct_option_id
    }

    pub fn explanation(&self) -> Option<String> {
        self.explanation.clone()
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
}

impl Location {
    pub fn new(longitude: f64, latitude: f64) -> Self {
        Self {
            longitude,
            latitude,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
        }
    }

    pub fn set_longitude(&mut self, longitude: f64) {
        self.longitude = longitude;
    }

    pub fn set_latitude(&mut self, latitude: f64) {
        self.latitude = latitude;
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

    pub fn longitude(&self) -> f64 {
        self.longitude
    }

    pub fn latitude(&self) -> f64 {
        self.latitude
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
}

impl Venue {
    pub fn new(location: Location, title: String, address: String) -> Self {
        Self {
            location,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        }
    }

    pub fn set_location(&mut self, location: Location) {
        self.location = location;
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

    pub fn location(&self) -> Location {
        self.location.clone()
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
}

impl ProximityAlertTriggered {
    pub fn new(traveler: User, watcher: User, distance: u32) -> Self {
        Self {
            traveler,
            watcher,
            distance,
        }
    }

    pub fn set_traveler(&mut self, traveler: User) {
        self.traveler = traveler;
    }

    pub fn set_watcher(&mut self, watcher: User) {
        self.watcher = watcher;
    }

    pub fn set_distance(&mut self, distance: u32) {
        self.distance = distance;
    }

    pub fn traveler(&self) -> User {
        self.traveler.clone()
    }

    pub fn watcher(&self) -> User {
        self.watcher.clone()
    }

    pub fn distance(&self) -> u32 {
        self.distance
    }
}

impl MessageAutoDeleteTimerChanged {
    pub fn new(message_auto_delete_time: u32) -> Self {
        Self {
            message_auto_delete_time,
        }
    }

    pub fn set_message_auto_delete_time(&mut self, message_auto_delete_time: u32) {
        self.message_auto_delete_time = message_auto_delete_time;
    }

    pub fn message_auto_delete_time(&self) -> u32 {
        self.message_auto_delete_time
    }
}

impl VoiceChatEnded {
    pub fn new(duration: u32) -> Self {
        Self { duration }
    }

    pub fn set_duration(&mut self, duration: u32) {
        self.duration = duration;
    }

    pub fn duration(&self) -> u32 {
        self.duration
    }
}

impl VoiceChatScheduled {
    pub fn new(start_date: u64) -> Self {
        Self { start_date }
    }

    pub fn set_start_date(&mut self, start_date: u64) {
        self.start_date = start_date;
    }

    pub fn start_date(&self) -> u64 {
        self.start_date
    }
}

impl VoiceChatParticipantsInvited {
    pub fn new() -> Self {
        Self { users: None }
    }

    pub fn set_users(&mut self, users: Option<Vec<User>>) {
        self.users = users;
    }

    pub fn users(&self) -> Option<Vec<User>> {
        self.users.clone()
    }
}

impl UserProfilePhotos {
    pub fn new(total_count: u32, photos: Vec<Vec<PhotoSize>>) -> Self {
        Self {
            total_count,
            photos,
        }
    }

    pub fn set_total_count(&mut self, total_count: u32) {
        self.total_count = total_count;
    }

    pub fn set_photos(&mut self, photos: Vec<Vec<PhotoSize>>) {
        self.photos = photos;
    }

    pub fn total_count(&self) -> u32 {
        self.total_count
    }

    pub fn photos(&self) -> Vec<Vec<PhotoSize>> {
        self.photos.clone()
    }
}

impl File {
    pub fn new(file_id: String, file_unique_id: String) -> Self {
        Self {
            file_id,
            file_unique_id,
            file_size: None,
            file_path: None,
        }
    }

    pub fn set_file_id(&mut self, file_id: String) {
        self.file_id = file_id;
    }

    pub fn set_file_unique_id(&mut self, file_unique_id: String) {
        self.file_unique_id = file_unique_id;
    }

    pub fn set_file_size(&mut self, file_size: Option<u32>) {
        self.file_size = file_size;
    }

    pub fn set_file_path(&mut self, file_path: Option<String>) {
        self.file_path = file_path;
    }

    pub fn file_id(&self) -> String {
        self.file_id.clone()
    }

    pub fn file_unique_id(&self) -> String {
        self.file_unique_id.clone()
    }

    pub fn file_size(&self) -> Option<u32> {
        self.file_size
    }

    pub fn file_path(&self) -> Option<String> {
        self.file_path.clone()
    }
}

impl ReplyKeyboardMarkup {
    pub fn new(keyboard: Vec<Vec<KeyboardButton>>) -> Self {
        Self {
            keyboard,
            resize_keyboard: None,
            one_time_keyboard: None,
            input_field_placeholder: None,
            selective: None,
        }
    }

    pub fn set_keyboard(&mut self, keyboard: Vec<Vec<KeyboardButton>>) {
        self.keyboard = keyboard;
    }

    pub fn set_resize_keyboard(&mut self, resize_keyboard: Option<bool>) {
        self.resize_keyboard = resize_keyboard;
    }

    pub fn set_one_time_keyboard(&mut self, one_time_keyboard: Option<bool>) {
        self.one_time_keyboard = one_time_keyboard;
    }

    pub fn set_input_field_placeholder(&mut self, input_field_placeholder: Option<String>) {
        self.input_field_placeholder = input_field_placeholder;
    }

    pub fn set_selective(&mut self, selective: Option<bool>) {
        self.selective = selective;
    }

    pub fn keyboard(&self) -> Vec<Vec<KeyboardButton>> {
        self.keyboard.clone()
    }

    pub fn resize_keyboard(&self) -> Option<bool> {
        self.resize_keyboard
    }

    pub fn one_time_keyboard(&self) -> Option<bool> {
        self.one_time_keyboard
    }

    pub fn input_field_placeholder(&self) -> Option<String> {
        self.input_field_placeholder.clone()
    }

    pub fn selective(&self) -> Option<bool> {
        self.selective
    }
}

impl KeyboardButton {
    pub fn new(text: String) -> Self {
        Self {
            text,
            request_contact: None,
            request_location: None,
            request_poll: None,
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn set_request_contact(&mut self, request_contact: Option<bool>) {
        self.request_contact = request_contact;
    }

    pub fn set_request_location(&mut self, request_location: Option<bool>) {
        self.request_location = request_location;
    }

    pub fn set_request_poll(&mut self, request_poll: Option<KeyboardButtonPollType>) {
        self.request_poll = request_poll;
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn request_contact(&self) -> Option<bool> {
        self.request_contact
    }

    pub fn request_location(&self) -> Option<bool> {
        self.request_location
    }

    pub fn request_poll(&self) -> Option<KeyboardButtonPollType> {
        self.request_poll.clone()
    }
}

impl KeyboardButtonPollType {
    pub fn new() -> Self {
        Self { type_field: None }
    }

    pub fn set_type_field(&mut self, type_field: Option<PollType>) {
        self.type_field = type_field;
    }

    pub fn type_field(&self) -> Option<PollType> {
        self.type_field.clone()
    }
}

impl ReplyKeyboardRemove {
    pub fn new(remove_keyboard: bool) -> Self {
        Self {
            remove_keyboard,
            selective: None,
        }
    }

    pub fn set_remove_keyboard(&mut self, remove_keyboard: bool) {
        self.remove_keyboard = remove_keyboard;
    }

    pub fn set_selective(&mut self, selective: Option<bool>) {
        self.selective = selective;
    }

    pub fn remove_keyboard(&self) -> bool {
        self.remove_keyboard
    }

    pub fn selective(&self) -> Option<bool> {
        self.selective
    }
}

impl InlineKeyboardMarkup {
    pub fn new(inline_keyboard: Vec<Vec<InlineKeyboardButton>>) -> Self {
        Self { inline_keyboard }
    }

    pub fn set_inline_keyboard(&mut self, inline_keyboard: Vec<Vec<InlineKeyboardButton>>) {
        self.inline_keyboard = inline_keyboard;
    }

    pub fn inline_keyboard(&self) -> Vec<Vec<InlineKeyboardButton>> {
        self.inline_keyboard.clone()
    }
}

impl InlineKeyboardButton {
    pub fn new(text: String) -> Self {
        Self {
            text,
            url: None,
            login_url: None,
            callback_data: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: None,
            pay: None,
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn set_url(&mut self, url: Option<String>) {
        self.url = url;
    }

    pub fn set_login_url(&mut self, login_url: Option<LoginUrl>) {
        self.login_url = login_url;
    }

    pub fn set_callback_data(&mut self, callback_data: Option<String>) {
        self.callback_data = callback_data;
    }

    pub fn set_switch_inline_query(&mut self, switch_inline_query: Option<String>) {
        self.switch_inline_query = switch_inline_query;
    }

    pub fn set_switch_inline_query_current_chat(
        &mut self,
        switch_inline_query_current_chat: Option<String>,
    ) {
        self.switch_inline_query_current_chat = switch_inline_query_current_chat;
    }

    pub fn set_callback_game(&mut self, callback_game: Option<CallbackGame>) {
        self.callback_game = callback_game;
    }

    pub fn set_pay(&mut self, pay: Option<bool>) {
        self.pay = pay;
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn url(&self) -> Option<String> {
        self.url.clone()
    }

    pub fn login_url(&self) -> Option<LoginUrl> {
        self.login_url.clone()
    }

    pub fn callback_data(&self) -> Option<String> {
        self.callback_data.clone()
    }

    pub fn switch_inline_query(&self) -> Option<String> {
        self.switch_inline_query.clone()
    }

    pub fn switch_inline_query_current_chat(&self) -> Option<String> {
        self.switch_inline_query_current_chat.clone()
    }

    pub fn callback_game(&self) -> Option<CallbackGame> {
        self.callback_game.clone()
    }

    pub fn pay(&self) -> Option<bool> {
        self.pay
    }
}

impl LoginUrl {
    pub fn new(url: String) -> Self {
        Self {
            url,
            forward_text: None,
            bot_username: None,
            request_write_access: None,
        }
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn set_forward_text(&mut self, forward_text: Option<String>) {
        self.forward_text = forward_text;
    }

    pub fn set_bot_username(&mut self, bot_username: Option<String>) {
        self.bot_username = bot_username;
    }

    pub fn set_request_write_access(&mut self, request_write_access: Option<bool>) {
        self.request_write_access = request_write_access;
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn forward_text(&self) -> Option<String> {
        self.forward_text.clone()
    }

    pub fn bot_username(&self) -> Option<String> {
        self.bot_username.clone()
    }

    pub fn request_write_access(&self) -> Option<bool> {
        self.request_write_access
    }
}

impl CallbackQuery {
    pub fn new(id: String, from: User, chat_instance: String) -> Self {
        Self {
            id,
            from,
            chat_instance,
            message: None,
            inline_message_id: None,
            data: None,
            game_short_name: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_from(&mut self, from: User) {
        self.from = from;
    }

    pub fn set_chat_instance(&mut self, chat_instance: String) {
        self.chat_instance = chat_instance;
    }

    pub fn set_message(&mut self, message: Option<Message>) {
        self.message = message;
    }

    pub fn set_inline_message_id(&mut self, inline_message_id: Option<String>) {
        self.inline_message_id = inline_message_id;
    }

    pub fn set_data(&mut self, data: Option<String>) {
        self.data = data;
    }

    pub fn set_game_short_name(&mut self, game_short_name: Option<String>) {
        self.game_short_name = game_short_name;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn from(&self) -> User {
        self.from.clone()
    }

    pub fn chat_instance(&self) -> String {
        self.chat_instance.clone()
    }

    pub fn message(&self) -> Option<Message> {
        self.message.clone()
    }

    pub fn inline_message_id(&self) -> Option<String> {
        self.inline_message_id.clone()
    }

    pub fn data(&self) -> Option<String> {
        self.data.clone()
    }

    pub fn game_short_name(&self) -> Option<String> {
        self.game_short_name.clone()
    }
}

impl ForceReply {
    pub fn new(force_reply: bool) -> Self {
        Self {
            force_reply,
            input_field_placeholder: None,
            selective: None,
        }
    }

    pub fn set_force_reply(&mut self, force_reply: bool) {
        self.force_reply = force_reply;
    }

    pub fn set_selective(&mut self, selective: Option<bool>) {
        self.selective = selective;
    }

    pub fn set_input_field_placeholder(&mut self, input_field_placeholder: Option<String>) {
        self.input_field_placeholder = input_field_placeholder;
    }

    pub fn input_field_placeholder(&self) -> Option<String> {
        self.input_field_placeholder.clone()
    }

    pub fn force_reply(&self) -> bool {
        self.force_reply
    }

    pub fn selective(&self) -> Option<bool> {
        self.selective
    }
}

impl ChatPhoto {
    pub fn new(
        small_file_id: String,
        small_file_unique_id: String,
        big_file_id: String,
        big_file_unique_id: String,
    ) -> Self {
        Self {
            small_file_id,
            small_file_unique_id,
            big_file_id,
            big_file_unique_id,
        }
    }

    pub fn set_small_file_id(&mut self, small_file_id: String) {
        self.small_file_id = small_file_id;
    }

    pub fn set_small_file_unique_id(&mut self, small_file_unique_id: String) {
        self.small_file_unique_id = small_file_unique_id;
    }

    pub fn set_big_file_id(&mut self, big_file_id: String) {
        self.big_file_id = big_file_id;
    }

    pub fn set_big_file_unique_id(&mut self, big_file_unique_id: String) {
        self.big_file_unique_id = big_file_unique_id;
    }

    pub fn small_file_id(&self) -> String {
        self.small_file_id.clone()
    }

    pub fn small_file_unique_id(&self) -> String {
        self.small_file_unique_id.clone()
    }

    pub fn big_file_id(&self) -> String {
        self.big_file_id.clone()
    }

    pub fn big_file_unique_id(&self) -> String {
        self.big_file_unique_id.clone()
    }
}

impl ChatInviteLink {
    pub fn new(
        invite_link: String,
        creator: User,
        creates_join_request: bool,
        is_primary: bool,
        is_revoked: bool,
    ) -> Self {
        Self {
            invite_link,
            creator,
            creates_join_request,
            is_primary,
            is_revoked,
            name: None,
            expire_date: None,
            member_limit: None,
            pending_join_request_count: None,
        }
    }

    pub fn set_invite_link(&mut self, invite_link: String) {
        self.invite_link = invite_link;
    }

    pub fn set_creator(&mut self, creator: User) {
        self.creator = creator;
    }

    pub fn set_creates_join_request(&mut self, creates_join_request: bool) {
        self.creates_join_request = creates_join_request;
    }

    pub fn set_is_primary(&mut self, is_primary: bool) {
        self.is_primary = is_primary;
    }

    pub fn set_is_revoked(&mut self, is_revoked: bool) {
        self.is_revoked = is_revoked;
    }

    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    pub fn set_expire_date(&mut self, expire_date: Option<u64>) {
        self.expire_date = expire_date;
    }

    pub fn set_member_limit(&mut self, member_limit: Option<u32>) {
        self.member_limit = member_limit;
    }

    pub fn set_pending_join_request_count(&mut self, pending_join_request_count: Option<u32>) {
        self.pending_join_request_count = pending_join_request_count;
    }

    pub fn invite_link(&self) -> String {
        self.invite_link.clone()
    }

    pub fn creator(&self) -> User {
        self.creator.clone()
    }

    pub fn creates_join_request(&mut self) -> bool {
        self.creates_join_request
    }

    pub fn is_primary(&self) -> bool {
        self.is_primary
    }

    pub fn is_revoked(&self) -> bool {
        self.is_revoked
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn expire_date(&self) -> Option<u64> {
        self.expire_date
    }

    pub fn member_limit(&self) -> Option<u32> {
        self.member_limit
    }

    pub fn pending_join_request_count(&mut self) -> Option<u32> {
        self.pending_join_request_count
    }
}

impl ChatMemberUpdated {
    pub fn new(
        chat: Chat,
        from: User,
        date: u64,
        old_chat_member: ChatMember,
        new_chat_member: ChatMember,
    ) -> Self {
        Self {
            chat,
            from,
            date,
            old_chat_member,
            new_chat_member,
            invite_link: None,
        }
    }

    pub fn set_chat(&mut self, chat: Chat) {
        self.chat = chat;
    }

    pub fn set_from(&mut self, from: User) {
        self.from = from;
    }

    pub fn set_date(&mut self, date: u64) {
        self.date = date;
    }

    pub fn set_old_chat_member(&mut self, old_chat_member: ChatMember) {
        self.old_chat_member = old_chat_member;
    }

    pub fn set_new_chat_member(&mut self, new_chat_member: ChatMember) {
        self.new_chat_member = new_chat_member;
    }

    pub fn set_invite_link(&mut self, invite_link: Option<ChatInviteLink>) {
        self.invite_link = invite_link;
    }

    pub fn chat(&self) -> Chat {
        self.chat.clone()
    }

    pub fn from(&self) -> User {
        self.from.clone()
    }

    pub fn date(&self) -> u64 {
        self.date
    }

    pub fn old_chat_member(&self) -> ChatMember {
        self.old_chat_member.clone()
    }

    pub fn new_chat_member(&self) -> ChatMember {
        self.new_chat_member.clone()
    }

    pub fn invite_link(&self) -> Option<ChatInviteLink> {
        self.invite_link.clone()
    }
}

impl ChatJoinRequest {
    pub fn new(chat: Chat, from: User, date: u64) -> Self {
        Self {
            chat,
            from,
            date,
            bio: None,
            invite_link: None,
        }
    }

    pub fn set_chat(&mut self, chat: Chat) {
        self.chat = chat;
    }

    pub fn set_from(&mut self, from: User) {
        self.from = from;
    }

    pub fn set_date(&mut self, date: u64) {
        self.date = date;
    }

    pub fn set_bio(&mut self, bio: Option<String>) {
        self.bio = bio;
    }

    pub fn set_invite_link(&mut self, invite_link: Option<ChatInviteLink>) {
        self.invite_link = invite_link;
    }

    pub fn chat(&self) -> Chat {
        self.chat.clone()
    }

    pub fn from(&self) -> User {
        self.from.clone()
    }

    pub fn date(&self) -> u64 {
        self.date
    }

    pub fn bio(&self) -> Option<String> {
        self.bio.clone()
    }

    pub fn invite_link(&self) -> Option<ChatInviteLink> {
        self.invite_link.clone()
    }
}

impl ChatPermissions {
    pub fn new() -> Self {
        Self {
            can_send_messages: None,
            can_send_media_messages: None,
            can_send_polls: None,
            can_send_other_messages: None,
            can_add_web_page_previews: None,
            can_change_info: None,
            can_invite_users: None,
            can_pin_messages: None,
        }
    }

    pub fn set_can_send_messages(&mut self, can_send_messages: Option<bool>) {
        self.can_send_messages = can_send_messages;
    }

    pub fn set_can_send_media_messages(&mut self, can_send_media_messages: Option<bool>) {
        self.can_send_media_messages = can_send_media_messages;
    }

    pub fn set_can_send_polls(&mut self, can_send_polls: Option<bool>) {
        self.can_send_polls = can_send_polls;
    }

    pub fn set_can_send_other_messages(&mut self, can_send_other_messages: Option<bool>) {
        self.can_send_other_messages = can_send_other_messages;
    }

    pub fn set_can_add_web_page_previews(&mut self, can_add_web_page_previews: Option<bool>) {
        self.can_add_web_page_previews = can_add_web_page_previews;
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

    pub fn can_send_messages(&self) -> Option<bool> {
        self.can_send_messages
    }

    pub fn can_send_media_messages(&self) -> Option<bool> {
        self.can_send_media_messages
    }

    pub fn can_send_polls(&self) -> Option<bool> {
        self.can_send_polls
    }

    pub fn can_send_other_messages(&self) -> Option<bool> {
        self.can_send_other_messages
    }

    pub fn can_add_web_page_previews(&self) -> Option<bool> {
        self.can_add_web_page_previews
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

impl ChatLocation {
    pub fn new(location: Location, address: String) -> Self {
        Self { location, address }
    }

    pub fn set_location(&mut self, location: Location) {
        self.location = location;
    }

    pub fn set_address(&mut self, address: String) {
        self.address = address;
    }

    pub fn location(&self) -> Location {
        self.location.clone()
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }
}

impl BotCommand {
    pub fn new(command: String, description: String) -> Self {
        Self {
            command,
            description,
        }
    }

    pub fn set_command(&mut self, command: String) {
        self.command = command;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn command(&self) -> String {
        self.command.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }
}

impl ResponseParameters {
    pub fn new() -> Self {
        Self {
            migrate_to_chat_id: None,
            retry_after: None,
        }
    }

    pub fn set_migrate_to_chat_id(&mut self, migrate_to_chat_id: Option<i64>) {
        self.migrate_to_chat_id = migrate_to_chat_id;
    }

    pub fn set_retry_after(&mut self, retry_after: Option<u16>) {
        self.retry_after = retry_after;
    }

    pub fn migrate_to_chat_id(&self) -> Option<i64> {
        self.migrate_to_chat_id
    }

    pub fn retry_after(&self) -> Option<u16> {
        self.retry_after
    }
}

impl Sticker {
    pub fn new(
        file_id: String,
        file_unique_id: String,
        width: u32,
        height: u32,
        is_animated: bool,
    ) -> Self {
        Self {
            file_id,
            file_unique_id,
            width,
            height,
            is_animated,
            thumb: None,
            emoji: None,
            set_name: None,
            mask_position: None,
            file_size: None,
        }
    }

    pub fn set_file_id(&mut self, file_id: String) {
        self.file_id = file_id;
    }

    pub fn set_file_unique_id(&mut self, file_unique_id: String) {
        self.file_unique_id = file_unique_id;
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn set_is_animated(&mut self, is_animated: bool) {
        self.is_animated = is_animated;
    }

    pub fn set_thumb(&mut self, thumb: Option<PhotoSize>) {
        self.thumb = thumb;
    }

    pub fn set_emoji(&mut self, emoji: Option<String>) {
        self.emoji = emoji;
    }

    pub fn set_set_name(&mut self, set_name: Option<String>) {
        self.set_name = set_name;
    }

    pub fn set_mask_position(&mut self, mask_position: Option<MaskPosition>) {
        self.mask_position = mask_position;
    }

    pub fn set_file_size(&mut self, file_size: Option<u32>) {
        self.file_size = file_size;
    }

    pub fn file_id(&self) -> String {
        self.file_id.clone()
    }

    pub fn file_unique_id(&self) -> String {
        self.file_unique_id.clone()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn is_animated(&self) -> bool {
        self.is_animated
    }

    pub fn thumb(&self) -> Option<PhotoSize> {
        self.thumb.clone()
    }

    pub fn emoji(&self) -> Option<String> {
        self.emoji.clone()
    }

    pub fn set_name(&self) -> Option<String> {
        self.set_name.clone()
    }

    pub fn mask_position(&self) -> Option<MaskPosition> {
        self.mask_position.clone()
    }

    pub fn file_size(&self) -> Option<u32> {
        self.file_size
    }
}

impl StickerSet {
    pub fn new(
        name: String,
        title: String,
        is_animated: bool,
        contains_masks: bool,
        stickers: Vec<Sticker>,
    ) -> Self {
        Self {
            name,
            title,
            is_animated,
            contains_masks,
            stickers,
            thumb: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_is_animated(&mut self, is_animated: bool) {
        self.is_animated = is_animated;
    }

    pub fn set_contains_masks(&mut self, contains_masks: bool) {
        self.contains_masks = contains_masks;
    }

    pub fn set_stickers(&mut self, stickers: Vec<Sticker>) {
        self.stickers = stickers;
    }

    pub fn set_thumb(&mut self, thumb: Option<PhotoSize>) {
        self.thumb = thumb;
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn is_animated(&self) -> bool {
        self.is_animated
    }

    pub fn contains_masks(&self) -> bool {
        self.contains_masks
    }

    pub fn stickers(&self) -> Vec<Sticker> {
        self.stickers.clone()
    }

    pub fn thumb(&self) -> Option<PhotoSize> {
        self.thumb.clone()
    }
}

impl MaskPosition {
    pub fn new(point: String, x_shift: f64, y_shift: f64, scale: f64) -> Self {
        Self {
            point,
            x_shift,
            y_shift,
            scale,
        }
    }

    pub fn set_point(&mut self, point: String) {
        self.point = point;
    }

    pub fn set_x_shift(&mut self, x_shift: f64) {
        self.x_shift = x_shift;
    }

    pub fn set_y_shift(&mut self, y_shift: f64) {
        self.y_shift = y_shift;
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }

    pub fn point(&self) -> String {
        self.point.clone()
    }

    pub fn x_shift(&self) -> f64 {
        self.x_shift
    }

    pub fn y_shift(&self) -> f64 {
        self.y_shift
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }
}

impl InlineQuery {
    pub fn new(id: String, from: User, query: String, offset: String) -> Self {
        Self {
            id,
            from,
            query,
            offset,
            location: None,
            chat_type: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_from(&mut self, from: User) {
        self.from = from;
    }

    pub fn set_query(&mut self, query: String) {
        self.query = query;
    }

    pub fn set_offset(&mut self, offset: String) {
        self.offset = offset;
    }

    pub fn set_location(&mut self, location: Option<Location>) {
        self.location = location;
    }

    pub fn set_chat_type(&mut self, chat_type: Option<String>) {
        self.chat_type = chat_type;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn from(&self) -> User {
        self.from.clone()
    }

    pub fn query(&self) -> String {
        self.query.clone()
    }

    pub fn offset(&self) -> String {
        self.offset.clone()
    }

    pub fn location(&self) -> Option<Location> {
        self.location.clone()
    }

    pub fn chat_type(&self) -> Option<String> {
        self.chat_type.clone()
    }
}

impl InlineQueryResultArticle {
    pub fn new(id: String, title: String, input_message_content: InputMessageContent) -> Self {
        Self {
            id,
            title,
            input_message_content,
            reply_markup: None,
            url: None,
            hide_url: None,
            description: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_input_message_content(&mut self, input_message_content: InputMessageContent) {
        self.input_message_content = input_message_content;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<InlineKeyboardMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn set_url(&mut self, url: Option<String>) {
        self.url = url;
    }

    pub fn set_hide_url(&mut self, hide_url: Option<bool>) {
        self.hide_url = hide_url;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn set_thumb_url(&mut self, thumb_url: Option<String>) {
        self.thumb_url = thumb_url;
    }

    pub fn set_thumb_width(&mut self, thumb_width: Option<u32>) {
        self.thumb_width = thumb_width;
    }

    pub fn set_thumb_height(&mut self, thumb_height: Option<u32>) {
        self.thumb_height = thumb_height;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn input_message_content(&self) -> InputMessageContent {
        self.input_message_content.clone()
    }

    pub fn reply_markup(&self) -> Option<InlineKeyboardMarkup> {
        self.reply_markup.clone()
    }

    pub fn url(&self) -> Option<String> {
        self.url.clone()
    }

    pub fn hide_url(&self) -> Option<bool> {
        self.hide_url
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn thumb_url(&self) -> Option<String> {
        self.thumb_url.clone()
    }

    pub fn thumb_width(&self) -> Option<u32> {
        self.thumb_width
    }

    pub fn thumb_height(&self) -> Option<u32> {
        self.thumb_height
    }
}

impl InlineQueryResultPhoto {
    pub fn new(id: String, photo_url: String, thumb_url: String) -> Self {
        Self {
            id,
            photo_url,
            thumb_url,
            photo_width: None,
            photo_height: None,
            title: None,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_photo_url(&mut self, photo_url: String) {
        self.photo_url = photo_url;
    }

    pub fn set_thumb_url(&mut self, thumb_url: String) {
        self.thumb_url = thumb_url;
    }

    pub fn set_photo_width(&mut self, photo_width: Option<u32>) {
        self.photo_width = photo_width;
    }

    pub fn set_photo_height(&mut self, photo_height: Option<u32>) {
        self.photo_height = photo_height;
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
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

    pub fn set_input_message_content(
        &mut self,
        input_message_content: Option<InputMessageContent>,
    ) {
        self.input_message_content = input_message_content;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn photo_url(&self) -> String {
        self.photo_url.clone()
    }

    pub fn thumb_url(&self) -> String {
        self.thumb_url.clone()
    }

    pub fn photo_width(&self) -> Option<u32> {
        self.photo_width
    }

    pub fn photo_height(&self) -> Option<u32> {
        self.photo_height
    }

    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
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

    pub fn input_message_content(&self) -> Option<InputMessageContent> {
        self.input_message_content.clone()
    }
}

impl InlineQueryResultGif {
    pub fn new(id: String, gif_url: String, thumb_url: String) -> Self {
        Self {
            id,
            gif_url,
            thumb_url,
            gif_width: None,
            gif_height: None,
            gif_duration: None,
            thumb_mime_type: None,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_gif_url(&mut self, gif_url: String) {
        self.gif_url = gif_url;
    }

    pub fn set_thumb_url(&mut self, thumb_url: String) {
        self.thumb_url = thumb_url;
    }

    pub fn set_gif_width(&mut self, gif_width: Option<u32>) {
        self.gif_width = gif_width;
    }

    pub fn set_gif_height(&mut self, gif_height: Option<u32>) {
        self.gif_height = gif_height;
    }

    pub fn set_gif_duration(&mut self, gif_duration: Option<u32>) {
        self.gif_duration = gif_duration;
    }

    pub fn set_thumb_mime_type(&mut self, thumb_mime_type: Option<String>) {
        self.thumb_mime_type = thumb_mime_type;
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
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

    pub fn set_input_message_content(
        &mut self,
        input_message_content: Option<InputMessageContent>,
    ) {
        self.input_message_content = input_message_content;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn gif_url(&self) -> String {
        self.gif_url.clone()
    }

    pub fn thumb_url(&self) -> String {
        self.thumb_url.clone()
    }

    pub fn gif_width(&self) -> Option<u32> {
        self.gif_width
    }

    pub fn gif_height(&self) -> Option<u32> {
        self.gif_height
    }

    pub fn gif_duration(&self) -> Option<u32> {
        self.gif_duration
    }

    pub fn thumb_mime_type(&self) -> Option<String> {
        self.thumb_mime_type.clone()
    }

    pub fn title(&self) -> Option<String> {
        self.title.clone()
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

    pub fn input_message_content(&self) -> Option<InputMessageContent> {
        self.input_message_content.clone()
    }
}

impl InlineQueryResultMpeg4Gif {
    pub fn new(id: String, mpeg4_url: String, thumb_url: String) -> Self {
        Self {
            id,
            mpeg4_url,
            thumb_url,
            mpeg4_width: None,
            mpeg4_height: None,
            mpeg4_duration: None,
            thumb_mime_type: None,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_mpeg4_url(&mut self, mpeg4_url: String) {
        self.mpeg4_url = mpeg4_url;
    }

    pub fn set_thumb_url(&mut self, thumb_url: String) {
        self.thumb_url = thumb_url;
    }

    pub fn set_mpeg4_width(&mut self, mpeg4_width: Option<u32>) {
        self.mpeg4_width = mpeg4_width;
    }

    pub fn set_mpeg4_height(&mut self, mpeg4_height: Option<u32>) {
        self.mpeg4_height = mpeg4_height;
    }

    pub fn set_mpeg4_duration(&mut self, mpeg4_duration: Option<u32>) {
        self.mpeg4_duration = mpeg4_duration;
    }

    pub fn set_thumb_mime_type(&mut self, thumb_mime_type: Option<String>) {
        self.thumb_mime_type = thumb_mime_type;
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
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

    pub fn set_input_message_content(
        &mut self,
        input_message_content: Option<InputMessageContent>,
    ) {
        self.input_message_content = input_message_content;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn mpeg4_url(&self) -> String {
        self.mpeg4_url.clone()
    }

    pub fn thumb_url(&self) -> String {
        self.thumb_url.clone()
    }

    pub fn mpeg4_width(&self) -> Option<u32> {
        self.mpeg4_width
    }

    pub fn mpeg4_height(&self) -> Option<u32> {
        self.mpeg4_height
    }

    pub fn mpeg4_duration(&self) -> Option<u32> {
        self.mpeg4_duration
    }

    pub fn thumb_mime_type(&self) -> Option<String> {
        self.thumb_mime_type.clone()
    }

    pub fn title(&self) -> Option<String> {
        self.title.clone()
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

    pub fn input_message_content(&self) -> Option<InputMessageContent> {
        self.input_message_content.clone()
    }
}

impl InlineQueryResultVideo {
    pub fn new(
        id: String,
        video_url: String,
        mime_type: String,
        thumb_url: String,
        title: String,
    ) -> Self {
        Self {
            id,
            video_url,
            mime_type,
            thumb_url,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            video_width: None,
            video_height: None,
            video_duration: None,
            description: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_video_url(&mut self, video_url: String) {
        self.video_url = video_url;
    }

    pub fn set_mime_type(&mut self, mime_type: String) {
        self.mime_type = mime_type;
    }

    pub fn set_thumb_url(&mut self, thumb_url: String) {
        self.thumb_url = thumb_url;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
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

    pub fn set_video_width(&mut self, video_width: Option<u32>) {
        self.video_width = video_width;
    }

    pub fn set_video_height(&mut self, video_height: Option<u32>) {
        self.video_height = video_height;
    }

    pub fn set_video_duration(&mut self, video_duration: Option<u32>) {
        self.video_duration = video_duration;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<InlineKeyboardMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn set_input_message_content(
        &mut self,
        input_message_content: Option<InputMessageContent>,
    ) {
        self.input_message_content = input_message_content;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn video_url(&self) -> String {
        self.video_url.clone()
    }

    pub fn mime_type(&self) -> String {
        self.mime_type.clone()
    }

    pub fn thumb_url(&self) -> String {
        self.thumb_url.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
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

    pub fn video_width(&self) -> Option<u32> {
        self.video_width
    }

    pub fn video_height(&self) -> Option<u32> {
        self.video_height
    }

    pub fn video_duration(&self) -> Option<u32> {
        self.video_duration
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn reply_markup(&self) -> Option<InlineKeyboardMarkup> {
        self.reply_markup.clone()
    }

    pub fn input_message_content(&self) -> Option<InputMessageContent> {
        self.input_message_content.clone()
    }
}

impl InlineQueryResultAudio {
    pub fn new(id: String, audio_url: String, title: String) -> Self {
        Self {
            id,
            audio_url,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            performer: None,
            audio_duration: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_audio_url(&mut self, audio_url: String) {
        self.audio_url = audio_url;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
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

    pub fn set_performer(&mut self, performer: Option<String>) {
        self.performer = performer;
    }

    pub fn set_audio_duration(&mut self, audio_duration: Option<u32>) {
        self.audio_duration = audio_duration;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<InlineKeyboardMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn set_input_message_content(
        &mut self,
        input_message_content: Option<InputMessageContent>,
    ) {
        self.input_message_content = input_message_content;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn audio_url(&self) -> String {
        self.audio_url.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
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

    pub fn performer(&self) -> Option<String> {
        self.performer.clone()
    }

    pub fn audio_duration(&self) -> Option<u32> {
        self.audio_duration
    }

    pub fn reply_markup(&self) -> Option<InlineKeyboardMarkup> {
        self.reply_markup.clone()
    }

    pub fn input_message_content(&self) -> Option<InputMessageContent> {
        self.input_message_content.clone()
    }
}

impl InlineQueryResultVoice {
    pub fn new(id: String, voice_url: String, title: String) -> Self {
        Self {
            id,
            voice_url,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            voice_duration: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_voice_url(&mut self, voice_url: String) {
        self.voice_url = voice_url;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
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

    pub fn set_voice_duration(&mut self, voice_duration: Option<u32>) {
        self.voice_duration = voice_duration;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<InlineKeyboardMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn set_input_message_content(
        &mut self,
        input_message_content: Option<InputMessageContent>,
    ) {
        self.input_message_content = input_message_content;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn voice_url(&self) -> String {
        self.voice_url.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
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

    pub fn voice_duration(&self) -> Option<u32> {
        self.voice_duration
    }

    pub fn reply_markup(&self) -> Option<InlineKeyboardMarkup> {
        self.reply_markup.clone()
    }

    pub fn input_message_content(&self) -> Option<InputMessageContent> {
        self.input_message_content.clone()
    }
}

impl InlineQueryResultDocument {
    pub fn new(id: String, title: String, document_url: String, mime_type: String) -> Self {
        Self {
            id,
            title,
            document_url,
            mime_type,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            description: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_document_url(&mut self, document_url: String) {
        self.document_url = document_url;
    }

    pub fn set_mime_type(&mut self, mime_type: String) {
        self.mime_type = mime_type;
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

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<InlineKeyboardMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn set_input_message_content(
        &mut self,
        input_message_content: Option<InputMessageContent>,
    ) {
        self.input_message_content = input_message_content;
    }

    pub fn set_thumb_url(&mut self, thumb_url: Option<String>) {
        self.thumb_url = thumb_url;
    }

    pub fn set_thumb_width(&mut self, thumb_width: Option<u32>) {
        self.thumb_width = thumb_width;
    }

    pub fn set_thumb_height(&mut self, thumb_height: Option<u32>) {
        self.thumb_height = thumb_height;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn document_url(&self) -> String {
        self.document_url.clone()
    }

    pub fn mime_type(&self) -> String {
        self.mime_type.clone()
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

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn reply_markup(&self) -> Option<InlineKeyboardMarkup> {
        self.reply_markup.clone()
    }

    pub fn input_message_content(&self) -> Option<InputMessageContent> {
        self.input_message_content.clone()
    }

    pub fn thumb_url(&self) -> Option<String> {
        self.thumb_url.clone()
    }

    pub fn thumb_width(&self) -> Option<u32> {
        self.thumb_width
    }

    pub fn thumb_height(&self) -> Option<u32> {
        self.thumb_height
    }
}

impl InlineQueryResultLocation {
    pub fn new(id: String, latitude: f64, longitude: f64, title: String) -> Self {
        Self {
            id,
            latitude,
            longitude,
            title,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
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

    pub fn set_reply_markup(&mut self, reply_markup: Option<InlineKeyboardMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn set_input_message_content(
        &mut self,
        input_message_content: Option<InputMessageContent>,
    ) {
        self.input_message_content = input_message_content;
    }

    pub fn set_thumb_url(&mut self, thumb_url: Option<String>) {
        self.thumb_url = thumb_url;
    }

    pub fn set_thumb_width(&mut self, thumb_width: Option<u32>) {
        self.thumb_width = thumb_width;
    }

    pub fn set_thumb_height(&mut self, thumb_height: Option<u32>) {
        self.thumb_height = thumb_height;
    }

    pub fn id(&self) -> String {
        self.id.clone()
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

    pub fn reply_markup(&self) -> Option<InlineKeyboardMarkup> {
        self.reply_markup.clone()
    }

    pub fn input_message_content(&self) -> Option<InputMessageContent> {
        self.input_message_content.clone()
    }

    pub fn thumb_url(&self) -> Option<String> {
        self.thumb_url.clone()
    }

    pub fn thumb_width(&self) -> Option<u32> {
        self.thumb_width
    }

    pub fn thumb_height(&self) -> Option<u32> {
        self.thumb_height
    }
}

impl InlineQueryResultVenue {
    pub fn new(id: String, latitude: f64, longitude: f64, title: String, address: String) -> Self {
        Self {
            id,
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
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

    pub fn set_reply_markup(&mut self, reply_markup: Option<InlineKeyboardMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn set_input_message_content(
        &mut self,
        input_message_content: Option<InputMessageContent>,
    ) {
        self.input_message_content = input_message_content;
    }

    pub fn set_thumb_url(&mut self, thumb_url: Option<String>) {
        self.thumb_url = thumb_url;
    }

    pub fn set_thumb_width(&mut self, thumb_width: Option<u32>) {
        self.thumb_width = thumb_width;
    }

    pub fn set_thumb_height(&mut self, thumb_height: Option<u32>) {
        self.thumb_height = thumb_height;
    }

    pub fn id(&self) -> String {
        self.id.clone()
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

    pub fn reply_markup(&self) -> Option<InlineKeyboardMarkup> {
        self.reply_markup.clone()
    }

    pub fn input_message_content(&self) -> Option<InputMessageContent> {
        self.input_message_content.clone()
    }

    pub fn thumb_url(&self) -> Option<String> {
        self.thumb_url.clone()
    }

    pub fn thumb_width(&self) -> Option<u32> {
        self.thumb_width
    }

    pub fn thumb_height(&self) -> Option<u32> {
        self.thumb_height
    }
}

impl InlineQueryResultContact {
    pub fn new(id: String, phone_number: String, first_name: String) -> Self {
        Self {
            id,
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
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

    pub fn set_reply_markup(&mut self, reply_markup: Option<InlineKeyboardMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn set_input_message_content(
        &mut self,
        input_message_content: Option<InputMessageContent>,
    ) {
        self.input_message_content = input_message_content;
    }

    pub fn set_thumb_url(&mut self, thumb_url: Option<String>) {
        self.thumb_url = thumb_url;
    }

    pub fn set_thumb_width(&mut self, thumb_width: Option<u32>) {
        self.thumb_width = thumb_width;
    }

    pub fn set_thumb_height(&mut self, thumb_height: Option<u32>) {
        self.thumb_height = thumb_height;
    }

    pub fn id(&self) -> String {
        self.id.clone()
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

    pub fn reply_markup(&self) -> Option<InlineKeyboardMarkup> {
        self.reply_markup.clone()
    }

    pub fn input_message_content(&self) -> Option<InputMessageContent> {
        self.input_message_content.clone()
    }

    pub fn thumb_url(&self) -> Option<String> {
        self.thumb_url.clone()
    }

    pub fn thumb_width(&self) -> Option<u32> {
        self.thumb_width
    }

    pub fn thumb_height(&self) -> Option<u32> {
        self.thumb_height
    }
}

impl InlineQueryResultGame {
    pub fn new(id: String, game_short_name: String) -> Self {
        Self {
            id,
            game_short_name,
            reply_markup: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_game_short_name(&mut self, game_short_name: String) {
        self.game_short_name = game_short_name;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<InlineKeyboardMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn game_short_name(&self) -> String {
        self.game_short_name.clone()
    }

    pub fn reply_markup(&self) -> Option<InlineKeyboardMarkup> {
        self.reply_markup.clone()
    }
}

impl InlineQueryResultCachedPhoto {
    pub fn new(id: String, photo_file_id: String) -> Self {
        Self {
            id,
            photo_file_id,
            title: None,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_photo_file_id(&mut self, photo_file_id: String) {
        self.photo_file_id = photo_file_id;
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
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

    pub fn set_input_message_content(
        &mut self,
        input_message_content: Option<InputMessageContent>,
    ) {
        self.input_message_content = input_message_content;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn photo_file_id(&self) -> String {
        self.photo_file_id.clone()
    }

    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
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

    pub fn input_message_content(&self) -> Option<InputMessageContent> {
        self.input_message_content.clone()
    }
}

impl InlineQueryResultCachedGif {
    pub fn new(id: String, gif_file_id: String) -> Self {
        Self {
            id,
            gif_file_id,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_gif_file_id(&mut self, gif_file_id: String) {
        self.gif_file_id = gif_file_id;
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
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

    pub fn set_input_message_content(
        &mut self,
        input_message_content: Option<InputMessageContent>,
    ) {
        self.input_message_content = input_message_content;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn gif_file_id(&self) -> String {
        self.gif_file_id.clone()
    }

    pub fn title(&self) -> Option<String> {
        self.title.clone()
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

    pub fn input_message_content(&self) -> Option<InputMessageContent> {
        self.input_message_content.clone()
    }
}

impl InlineQueryResultCachedMpeg4Gif {
    pub fn new(id: String, mpeg4_file_id: String) -> Self {
        Self {
            id,
            mpeg4_file_id,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_mpeg4_file_id(&mut self, mpeg4_file_id: String) {
        self.mpeg4_file_id = mpeg4_file_id;
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
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

    pub fn set_input_message_content(
        &mut self,
        input_message_content: Option<InputMessageContent>,
    ) {
        self.input_message_content = input_message_content;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn mpeg4_file_id(&self) -> String {
        self.mpeg4_file_id.clone()
    }

    pub fn title(&self) -> Option<String> {
        self.title.clone()
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

    pub fn input_message_content(&self) -> Option<InputMessageContent> {
        self.input_message_content.clone()
    }
}

impl InlineQueryResultCachedSticker {
    pub fn new(id: String, sticker_file_id: String) -> Self {
        Self {
            id,
            sticker_file_id,
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_sticker_file_id(&mut self, sticker_file_id: String) {
        self.sticker_file_id = sticker_file_id;
    }

    pub fn set_reply_markup(&mut self, reply_markup: Option<InlineKeyboardMarkup>) {
        self.reply_markup = reply_markup;
    }

    pub fn set_input_message_content(
        &mut self,
        input_message_content: Option<InputMessageContent>,
    ) {
        self.input_message_content = input_message_content;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn sticker_file_id(&self) -> String {
        self.sticker_file_id.clone()
    }

    pub fn reply_markup(&self) -> Option<InlineKeyboardMarkup> {
        self.reply_markup.clone()
    }

    pub fn input_message_content(&self) -> Option<InputMessageContent> {
        self.input_message_content.clone()
    }
}

impl InlineQueryResultCachedDocument {
    pub fn new(id: String, title: String, document_file_id: String) -> Self {
        Self {
            id,
            title,
            document_file_id,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_document_file_id(&mut self, document_file_id: String) {
        self.document_file_id = document_file_id;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
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

    pub fn set_input_message_content(
        &mut self,
        input_message_content: Option<InputMessageContent>,
    ) {
        self.input_message_content = input_message_content;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn document_file_id(&self) -> String {
        self.document_file_id.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
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

    pub fn input_message_content(&self) -> Option<InputMessageContent> {
        self.input_message_content.clone()
    }
}

impl InlineQueryResultCachedVideo {
    pub fn new(id: String, video_file_id: String, title: String) -> Self {
        Self {
            id,
            video_file_id,
            title,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_video_file_id(&mut self, video_file_id: String) {
        self.video_file_id = video_file_id;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
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

    pub fn set_input_message_content(
        &mut self,
        input_message_content: Option<InputMessageContent>,
    ) {
        self.input_message_content = input_message_content;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn video_file_id(&self) -> String {
        self.video_file_id.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
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

    pub fn input_message_content(&self) -> Option<InputMessageContent> {
        self.input_message_content.clone()
    }
}

impl InlineQueryResultCachedVoice {
    pub fn new(id: String, voice_file_id: String, title: String) -> Self {
        Self {
            id,
            voice_file_id,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_voice_file_id(&mut self, voice_file_id: String) {
        self.voice_file_id = voice_file_id;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
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

    pub fn set_input_message_content(
        &mut self,
        input_message_content: Option<InputMessageContent>,
    ) {
        self.input_message_content = input_message_content;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn voice_file_id(&self) -> String {
        self.voice_file_id.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
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

    pub fn input_message_content(&self) -> Option<InputMessageContent> {
        self.input_message_content.clone()
    }
}

impl InlineQueryResultCachedAudio {
    pub fn new(id: String, audio_file_id: String) -> Self {
        Self {
            id,
            audio_file_id,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_audio_file_id(&mut self, audio_file_id: String) {
        self.audio_file_id = audio_file_id;
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

    pub fn set_input_message_content(
        &mut self,
        input_message_content: Option<InputMessageContent>,
    ) {
        self.input_message_content = input_message_content;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn audio_file_id(&self) -> String {
        self.audio_file_id.clone()
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

    pub fn input_message_content(&self) -> Option<InputMessageContent> {
        self.input_message_content.clone()
    }
}

impl InputTextMessageContent {
    pub fn new(message_text: String) -> Self {
        Self {
            message_text,
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
        }
    }

    pub fn set_message_text(&mut self, message_text: String) {
        self.message_text = message_text;
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

    pub fn message_text(&self) -> String {
        self.message_text.clone()
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
}

impl InputLocationMessageContent {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
        }
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
}

impl InputVenueMessageContent {
    pub fn new(latitude: f64, longitude: f64, title: String, address: String) -> Self {
        Self {
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        }
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
}

impl InputContactMessageContent {
    pub fn new(phone_number: String, first_name: String) -> Self {
        Self {
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
        }
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
}

impl InputInvoiceMessageContent {
    pub fn new(
        title: String,
        description: String,
        payload: String,
        provider_token: String,
        currency: String,
        prices: Vec<LabeledPrice>,
    ) -> Self {
        Self {
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
            is_flexible: None,
        }
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

    pub fn set_currency(&mut self, currency: String) {
        self.currency = currency;
    }

    pub fn set_prices(&mut self, prices: Vec<LabeledPrice>) {
        self.prices = prices;
    }

    pub fn set_max_tip_amount(&mut self, max_tip_amount: Option<u32>) {
        self.max_tip_amount = max_tip_amount;
    }

    pub fn set_suggested_tip_amounts(&mut self, suggested_tip_amounts: Option<Vec<u32>>) {
        self.suggested_tip_amounts = suggested_tip_amounts;
    }

    pub fn set_provider_data(&mut self, provider_data: Option<String>) {
        self.provider_data = provider_data;
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
}

impl ChosenInlineResult {
    pub fn new(result_id: String, from: User, query: String) -> Self {
        Self {
            result_id,
            from,
            query,
            location: None,
            inline_message_id: None,
        }
    }

    pub fn set_result_id(&mut self, result_id: String) {
        self.result_id = result_id;
    }

    pub fn set_from(&mut self, from: User) {
        self.from = from;
    }

    pub fn set_query(&mut self, query: String) {
        self.query = query;
    }

    pub fn set_location(&mut self, location: Option<Location>) {
        self.location = location;
    }

    pub fn set_inline_message_id(&mut self, inline_message_id: Option<String>) {
        self.inline_message_id = inline_message_id;
    }

    pub fn result_id(&self) -> String {
        self.result_id.clone()
    }

    pub fn from(&self) -> User {
        self.from.clone()
    }

    pub fn query(&self) -> String {
        self.query.clone()
    }

    pub fn location(&self) -> Option<Location> {
        self.location.clone()
    }

    pub fn inline_message_id(&self) -> Option<String> {
        self.inline_message_id.clone()
    }
}

impl LabeledPrice {
    pub fn new(label: String, amount: u32) -> Self {
        Self { label, amount }
    }

    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }

    pub fn set_amount(&mut self, amount: u32) {
        self.amount = amount;
    }

    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn amount(&self) -> u32 {
        self.amount
    }
}

impl Invoice {
    pub fn new(
        title: String,
        description: String,
        start_parameter: String,
        currency: String,
        total_amount: u32,
    ) -> Self {
        Self {
            title,
            description,
            start_parameter,
            currency,
            total_amount,
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set_start_parameter(&mut self, start_parameter: String) {
        self.start_parameter = start_parameter;
    }

    pub fn set_currency(&mut self, currency: String) {
        self.currency = currency;
    }

    pub fn set_total_amount(&mut self, total_amount: u32) {
        self.total_amount = total_amount;
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn start_parameter(&self) -> String {
        self.start_parameter.clone()
    }

    pub fn currency(&self) -> String {
        self.currency.clone()
    }

    pub fn total_amount(&self) -> u32 {
        self.total_amount
    }
}

impl ShippingAddress {
    pub fn new(
        country_code: String,
        state: String,
        city: String,
        street_line1: String,
        street_line2: String,
        post_code: String,
    ) -> Self {
        Self {
            country_code,
            state,
            city,
            street_line1,
            street_line2,
            post_code,
        }
    }

    pub fn set_country_code(&mut self, country_code: String) {
        self.country_code = country_code;
    }

    pub fn set_state(&mut self, state: String) {
        self.state = state;
    }

    pub fn set_city(&mut self, city: String) {
        self.city = city;
    }

    pub fn set_street_line1(&mut self, street_line1: String) {
        self.street_line1 = street_line1;
    }

    pub fn set_street_line2(&mut self, street_line2: String) {
        self.street_line2 = street_line2;
    }

    pub fn set_post_code(&mut self, post_code: String) {
        self.post_code = post_code;
    }

    pub fn country_code(&self) -> String {
        self.country_code.clone()
    }

    pub fn state(&self) -> String {
        self.state.clone()
    }

    pub fn city(&self) -> String {
        self.city.clone()
    }

    pub fn street_line1(&self) -> String {
        self.street_line1.clone()
    }

    pub fn street_line2(&self) -> String {
        self.street_line2.clone()
    }

    pub fn post_code(&self) -> String {
        self.post_code.clone()
    }
}

impl OrderInfo {
    pub fn new() -> Self {
        Self {
            name: None,
            phone_number: None,
            email: None,
            shipping_address: None,
        }
    }

    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    pub fn set_phone_number(&mut self, phone_number: Option<String>) {
        self.phone_number = phone_number;
    }

    pub fn set_email(&mut self, email: Option<String>) {
        self.email = email;
    }

    pub fn set_shipping_address(&mut self, shipping_address: Option<ShippingAddress>) {
        self.shipping_address = shipping_address;
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn phone_number(&self) -> Option<String> {
        self.phone_number.clone()
    }

    pub fn email(&self) -> Option<String> {
        self.email.clone()
    }

    pub fn shipping_address(&self) -> Option<ShippingAddress> {
        self.shipping_address.clone()
    }
}

impl ShippingOption {
    pub fn new(id: String, title: String, prices: Vec<LabeledPrice>) -> Self {
        Self { id, title, prices }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_prices(&mut self, prices: Vec<LabeledPrice>) {
        self.prices = prices;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn prices(&self) -> Vec<LabeledPrice> {
        self.prices.clone()
    }
}

impl SuccessfulPayment {
    pub fn new(
        currency: String,
        total_amount: u32,
        invoice_payload: String,
        telegram_payment_charge_id: String,
        provider_payment_charge_id: String,
    ) -> Self {
        Self {
            currency,
            total_amount,
            invoice_payload,
            telegram_payment_charge_id,
            provider_payment_charge_id,
            shipping_option_id: None,
            order_info: None,
        }
    }

    pub fn set_currency(&mut self, currency: String) {
        self.currency = currency;
    }

    pub fn set_total_amount(&mut self, total_amount: u32) {
        self.total_amount = total_amount;
    }

    pub fn set_invoice_payload(&mut self, invoice_payload: String) {
        self.invoice_payload = invoice_payload;
    }

    pub fn set_telegram_payment_charge_id(&mut self, telegram_payment_charge_id: String) {
        self.telegram_payment_charge_id = telegram_payment_charge_id;
    }

    pub fn set_provider_payment_charge_id(&mut self, provider_payment_charge_id: String) {
        self.provider_payment_charge_id = provider_payment_charge_id;
    }

    pub fn set_shipping_option_id(&mut self, shipping_option_id: Option<String>) {
        self.shipping_option_id = shipping_option_id;
    }

    pub fn set_order_info(&mut self, order_info: Option<OrderInfo>) {
        self.order_info = order_info;
    }

    pub fn currency(&self) -> String {
        self.currency.clone()
    }

    pub fn total_amount(&self) -> u32 {
        self.total_amount
    }

    pub fn invoice_payload(&self) -> String {
        self.invoice_payload.clone()
    }

    pub fn telegram_payment_charge_id(&self) -> String {
        self.telegram_payment_charge_id.clone()
    }

    pub fn provider_payment_charge_id(&self) -> String {
        self.provider_payment_charge_id.clone()
    }

    pub fn shipping_option_id(&self) -> Option<String> {
        self.shipping_option_id.clone()
    }

    pub fn order_info(&self) -> Option<OrderInfo> {
        self.order_info.clone()
    }
}

impl ShippingQuery {
    pub fn new(
        id: String,
        from: User,
        invoice_payload: String,
        shipping_address: ShippingAddress,
    ) -> Self {
        Self {
            id,
            from,
            invoice_payload,
            shipping_address,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_from(&mut self, from: User) {
        self.from = from;
    }

    pub fn set_invoice_payload(&mut self, invoice_payload: String) {
        self.invoice_payload = invoice_payload;
    }

    pub fn set_shipping_address(&mut self, shipping_address: ShippingAddress) {
        self.shipping_address = shipping_address;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn from(&self) -> User {
        self.from.clone()
    }

    pub fn invoice_payload(&self) -> String {
        self.invoice_payload.clone()
    }

    pub fn shipping_address(&self) -> ShippingAddress {
        self.shipping_address.clone()
    }
}

impl PreCheckoutQuery {
    pub fn new(
        id: String,
        from: User,
        currency: String,
        total_amount: u32,
        invoice_payload: String,
    ) -> Self {
        Self {
            id,
            from,
            currency,
            total_amount,
            invoice_payload,
            shipping_option_id: None,
            order_info: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_from(&mut self, from: User) {
        self.from = from;
    }

    pub fn set_currency(&mut self, currency: String) {
        self.currency = currency;
    }

    pub fn set_total_amount(&mut self, total_amount: u32) {
        self.total_amount = total_amount;
    }

    pub fn set_invoice_payload(&mut self, invoice_payload: String) {
        self.invoice_payload = invoice_payload;
    }

    pub fn set_shipping_option_id(&mut self, shipping_option_id: Option<String>) {
        self.shipping_option_id = shipping_option_id;
    }

    pub fn set_order_info(&mut self, order_info: Option<OrderInfo>) {
        self.order_info = order_info;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn from(&self) -> User {
        self.from.clone()
    }

    pub fn currency(&self) -> String {
        self.currency.clone()
    }

    pub fn total_amount(&self) -> u32 {
        self.total_amount
    }

    pub fn invoice_payload(&self) -> String {
        self.invoice_payload.clone()
    }

    pub fn shipping_option_id(&self) -> Option<String> {
        self.shipping_option_id.clone()
    }

    pub fn order_info(&self) -> Option<OrderInfo> {
        self.order_info.clone()
    }
}

impl PassportData {
    pub fn new(data: Vec<EncryptedPassportElement>, credentials: EncryptedCredentials) -> Self {
        Self { data, credentials }
    }

    pub fn set_data(&mut self, data: Vec<EncryptedPassportElement>) {
        self.data = data;
    }

    pub fn set_credentials(&mut self, credentials: EncryptedCredentials) {
        self.credentials = credentials;
    }

    pub fn data(&self) -> Vec<EncryptedPassportElement> {
        self.data.clone()
    }

    pub fn credentials(&self) -> EncryptedCredentials {
        self.credentials.clone()
    }
}

impl PassportFile {
    pub fn new(file_id: String, file_unique_id: String, file_size: u32, file_date: u64) -> Self {
        Self {
            file_id,
            file_unique_id,
            file_size,
            file_date,
        }
    }

    pub fn set_file_id(&mut self, file_id: String) {
        self.file_id = file_id;
    }

    pub fn set_file_unique_id(&mut self, file_unique_id: String) {
        self.file_unique_id = file_unique_id;
    }

    pub fn set_file_size(&mut self, file_size: u32) {
        self.file_size = file_size;
    }

    pub fn set_file_date(&mut self, file_date: u64) {
        self.file_date = file_date;
    }

    pub fn file_id(&self) -> String {
        self.file_id.clone()
    }

    pub fn file_unique_id(&self) -> String {
        self.file_unique_id.clone()
    }

    pub fn file_size(&self) -> u32 {
        self.file_size
    }

    pub fn file_date(&self) -> u64 {
        self.file_date
    }
}

impl EncryptedPassportElement {
    pub fn new(type_field: EncryptedPassportElementType, hash: String) -> Self {
        Self {
            type_field,
            hash,
            data: None,
            phone_number: None,
            email: None,
            files: None,
            front_side: None,
            reverse_side: None,
            selfie: None,
            translation: None,
        }
    }

    pub fn set_type_field(&mut self, type_field: EncryptedPassportElementType) {
        self.type_field = type_field;
    }

    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }

    pub fn set_data(&mut self, data: Option<String>) {
        self.data = data;
    }

    pub fn set_phone_number(&mut self, phone_number: Option<String>) {
        self.phone_number = phone_number;
    }

    pub fn set_email(&mut self, email: Option<String>) {
        self.email = email;
    }

    pub fn set_files(&mut self, files: Option<Vec<PassportFile>>) {
        self.files = files;
    }

    pub fn set_front_side(&mut self, front_side: Option<PassportFile>) {
        self.front_side = front_side;
    }

    pub fn set_reverse_side(&mut self, reverse_side: Option<PassportFile>) {
        self.reverse_side = reverse_side;
    }

    pub fn set_selfie(&mut self, selfie: Option<PassportFile>) {
        self.selfie = selfie;
    }

    pub fn set_translation(&mut self, translation: Option<Vec<PassportFile>>) {
        self.translation = translation;
    }

    pub fn type_field(&self) -> EncryptedPassportElementType {
        self.type_field.clone()
    }

    pub fn hash(&self) -> String {
        self.hash.clone()
    }

    pub fn data(&self) -> Option<String> {
        self.data.clone()
    }

    pub fn phone_number(&self) -> Option<String> {
        self.phone_number.clone()
    }

    pub fn email(&self) -> Option<String> {
        self.email.clone()
    }

    pub fn files(&self) -> Option<Vec<PassportFile>> {
        self.files.clone()
    }

    pub fn front_side(&self) -> Option<PassportFile> {
        self.front_side.clone()
    }

    pub fn reverse_side(&self) -> Option<PassportFile> {
        self.reverse_side.clone()
    }

    pub fn selfie(&self) -> Option<PassportFile> {
        self.selfie.clone()
    }

    pub fn translation(&self) -> Option<Vec<PassportFile>> {
        self.translation.clone()
    }
}

impl EncryptedCredentials {
    pub fn new(data: String, hash: String, secret: String) -> Self {
        Self { data, hash, secret }
    }

    pub fn set_data(&mut self, data: String) {
        self.data = data;
    }

    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }

    pub fn set_secret(&mut self, secret: String) {
        self.secret = secret;
    }

    pub fn data(&self) -> String {
        self.data.clone()
    }

    pub fn hash(&self) -> String {
        self.hash.clone()
    }

    pub fn secret(&self) -> String {
        self.secret.clone()
    }
}

impl PassportElementErrorDataField {
    pub fn new(
        type_field: PassportElementErrorDataFieldType,
        field_name: String,
        data_hash: String,
        message: String,
    ) -> Self {
        Self {
            type_field,
            field_name,
            data_hash,
            message,
        }
    }

    pub fn set_type_field(&mut self, type_field: PassportElementErrorDataFieldType) {
        self.type_field = type_field;
    }

    pub fn set_field_name(&mut self, field_name: String) {
        self.field_name = field_name;
    }

    pub fn set_data_hash(&mut self, data_hash: String) {
        self.data_hash = data_hash;
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn type_field(&self) -> PassportElementErrorDataFieldType {
        self.type_field.clone()
    }

    pub fn field_name(&self) -> String {
        self.field_name.clone()
    }

    pub fn data_hash(&self) -> String {
        self.data_hash.clone()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl PassportElementErrorFrontSide {
    pub fn new(
        type_field: PassportElementErrorFrontSideType,
        file_hash: String,
        message: String,
    ) -> Self {
        Self {
            type_field,
            file_hash,
            message,
        }
    }

    pub fn set_type_field(&mut self, type_field: PassportElementErrorFrontSideType) {
        self.type_field = type_field;
    }

    pub fn set_file_hash(&mut self, file_hash: String) {
        self.file_hash = file_hash;
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn type_field(&self) -> PassportElementErrorFrontSideType {
        self.type_field.clone()
    }

    pub fn file_hash(&self) -> String {
        self.file_hash.clone()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl PassportElementErrorReverseSide {
    pub fn new(
        type_field: PassportElementErrorReverseSideType,
        file_hash: String,
        message: String,
    ) -> Self {
        Self {
            type_field,
            file_hash,
            message,
        }
    }

    pub fn set_type_field(&mut self, type_field: PassportElementErrorReverseSideType) {
        self.type_field = type_field;
    }

    pub fn set_file_hash(&mut self, file_hash: String) {
        self.file_hash = file_hash;
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn type_field(&self) -> PassportElementErrorReverseSideType {
        self.type_field.clone()
    }

    pub fn file_hash(&self) -> String {
        self.file_hash.clone()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl PassportElementErrorSelfie {
    pub fn new(
        type_field: PassportElementErrorSelfieType,
        file_hash: String,
        message: String,
    ) -> Self {
        Self {
            type_field,
            file_hash,
            message,
        }
    }

    pub fn set_type_field(&mut self, type_field: PassportElementErrorSelfieType) {
        self.type_field = type_field;
    }

    pub fn set_file_hash(&mut self, file_hash: String) {
        self.file_hash = file_hash;
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn type_field(&self) -> PassportElementErrorSelfieType {
        self.type_field.clone()
    }

    pub fn file_hash(&self) -> String {
        self.file_hash.clone()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl PassportElementErrorFile {
    pub fn new(
        type_field: PassportElementErrorFileType,
        file_hash: String,
        message: String,
    ) -> Self {
        Self {
            type_field,
            file_hash,
            message,
        }
    }

    pub fn set_type_field(&mut self, type_field: PassportElementErrorFileType) {
        self.type_field = type_field;
    }

    pub fn set_file_hash(&mut self, file_hash: String) {
        self.file_hash = file_hash;
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn type_field(&self) -> PassportElementErrorFileType {
        self.type_field.clone()
    }

    pub fn file_hash(&self) -> String {
        self.file_hash.clone()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl PassportElementErrorFiles {
    pub fn new(
        type_field: PassportElementErrorFileType,
        file_hashes: Vec<String>,
        message: String,
    ) -> Self {
        Self {
            type_field,
            file_hashes,
            message,
        }
    }

    pub fn set_type_field(&mut self, type_field: PassportElementErrorFileType) {
        self.type_field = type_field;
    }

    pub fn set_file_hashes(&mut self, file_hashes: Vec<String>) {
        self.file_hashes = file_hashes;
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn type_field(&self) -> PassportElementErrorFileType {
        self.type_field.clone()
    }

    pub fn file_hashes(&self) -> Vec<String> {
        self.file_hashes.clone()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl PassportElementErrorTranslationFile {
    pub fn new(
        type_field: PassportElementErrorTranslationFileType,
        file_hash: String,
        message: String,
    ) -> Self {
        Self {
            type_field,
            file_hash,
            message,
        }
    }

    pub fn set_type_field(&mut self, type_field: PassportElementErrorTranslationFileType) {
        self.type_field = type_field;
    }

    pub fn set_file_hash(&mut self, file_hash: String) {
        self.file_hash = file_hash;
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn type_field(&self) -> PassportElementErrorTranslationFileType {
        self.type_field.clone()
    }

    pub fn file_hash(&self) -> String {
        self.file_hash.clone()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl PassportElementErrorTranslationFiles {
    pub fn new(
        type_field: PassportElementErrorTranslationFileType,
        file_hashes: Vec<String>,
        message: String,
    ) -> Self {
        Self {
            type_field,
            file_hashes,
            message,
        }
    }

    pub fn set_type_field(&mut self, type_field: PassportElementErrorTranslationFileType) {
        self.type_field = type_field;
    }

    pub fn set_file_hashes(&mut self, file_hashes: Vec<String>) {
        self.file_hashes = file_hashes;
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn type_field(&self) -> PassportElementErrorTranslationFileType {
        self.type_field.clone()
    }

    pub fn file_hashes(&self) -> Vec<String> {
        self.file_hashes.clone()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl PassportElementErrorUnspecified {
    pub fn new(
        type_field: EncryptedPassportElementType,
        element_hash: String,
        message: String,
    ) -> Self {
        Self {
            type_field,
            element_hash,
            message,
        }
    }

    pub fn set_type_field(&mut self, type_field: EncryptedPassportElementType) {
        self.type_field = type_field;
    }

    pub fn set_element_hash(&mut self, element_hash: String) {
        self.element_hash = element_hash;
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn type_field(&self) -> EncryptedPassportElementType {
        self.type_field.clone()
    }

    pub fn element_hash(&self) -> String {
        self.element_hash.clone()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl Game {
    pub fn new(title: String, description: String, photo: Vec<PhotoSize>) -> Self {
        Self {
            title,
            description,
            photo,
            text: None,
            text_entities: None,
            animation: None,
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set_photo(&mut self, photo: Vec<PhotoSize>) {
        self.photo = photo;
    }

    pub fn set_text(&mut self, text: Option<String>) {
        self.text = text;
    }

    pub fn set_text_entities(&mut self, text_entities: Option<Vec<MessageEntity>>) {
        self.text_entities = text_entities;
    }

    pub fn set_animation(&mut self, animation: Option<Animation>) {
        self.animation = animation;
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn photo(&self) -> Vec<PhotoSize> {
        self.photo.clone()
    }

    pub fn text(&self) -> Option<String> {
        self.text.clone()
    }

    pub fn text_entities(&self) -> Option<Vec<MessageEntity>> {
        self.text_entities.clone()
    }

    pub fn animation(&self) -> Option<Animation> {
        self.animation.clone()
    }
}

impl GameHighScore {
    pub fn new(position: u32, user: User, score: i32) -> Self {
        Self {
            position,
            user,
            score,
        }
    }

    pub fn set_position(&mut self, position: u32) {
        self.position = position;
    }

    pub fn set_user(&mut self, user: User) {
        self.user = user;
    }

    pub fn set_score(&mut self, score: i32) {
        self.score = score;
    }

    pub fn position(&self) -> u32 {
        self.position
    }

    pub fn user(&self) -> User {
        self.user.clone()
    }

    pub fn score(&self) -> i32 {
        self.score
    }
}

impl ChatMemberOwner {
    pub fn new(user: User, is_anonymous: bool) -> Self {
        Self {
            user,
            is_anonymous,
            custom_title: None,
        }
    }

    pub fn set_user(&mut self, user: User) {
        self.user = user;
    }

    pub fn set_custom_title(&mut self, custom_title: Option<String>) {
        self.custom_title = custom_title;
    }

    pub fn set_is_anonymous(&mut self, is_anonymous: bool) {
        self.is_anonymous = is_anonymous;
    }

    pub fn user(&self) -> User {
        self.user.clone()
    }

    pub fn custom_title(&self) -> Option<String> {
        self.custom_title.clone()
    }

    pub fn is_anonymous(&self) -> bool {
        self.is_anonymous
    }
}

impl ChatMemberAdministrator {
    pub fn new(user: User) -> Self {
        Self {
            user,
            can_be_edited: true,
            is_anonymous: true,
            can_manage_chat: true,
            can_delete_messages: true,
            can_manage_voice_chats: true,
            can_restrict_members: true,
            can_promote_members: true,
            can_change_info: true,
            can_invite_users: true,
            can_post_messages: None,
            can_edit_messages: None,
            can_pin_messages: None,
            custom_title: None,
        }
    }

    pub fn set_user(&mut self, user: User) {
        self.user = user;
    }

    pub fn set_can_be_edited(&mut self, can_be_edited: bool) {
        self.can_be_edited = can_be_edited;
    }

    pub fn set_custom_title(&mut self, custom_title: Option<String>) {
        self.custom_title = custom_title;
    }

    pub fn set_is_anonymous(&mut self, is_anonymous: bool) {
        self.is_anonymous = is_anonymous;
    }

    pub fn set_can_manage_chat(&mut self, can_manage_chat: bool) {
        self.can_manage_chat = can_manage_chat;
    }

    pub fn set_can_post_messages(&mut self, can_post_messages: Option<bool>) {
        self.can_post_messages = can_post_messages;
    }

    pub fn set_can_edit_messages(&mut self, can_edit_messages: Option<bool>) {
        self.can_edit_messages = can_edit_messages;
    }

    pub fn set_can_delete_messages(&mut self, can_delete_messages: bool) {
        self.can_delete_messages = can_delete_messages;
    }

    pub fn set_can_manage_voice_chats(&mut self, can_manage_voice_chats: bool) {
        self.can_manage_voice_chats = can_manage_voice_chats;
    }

    pub fn set_can_restrict_members(&mut self, can_restrict_members: bool) {
        self.can_restrict_members = can_restrict_members;
    }

    pub fn set_can_promote_members(&mut self, can_promote_members: bool) {
        self.can_promote_members = can_promote_members;
    }

    pub fn set_can_change_info(&mut self, can_change_info: bool) {
        self.can_change_info = can_change_info;
    }

    pub fn set_can_invite_users(&mut self, can_invite_users: bool) {
        self.can_invite_users = can_invite_users;
    }

    pub fn set_can_pin_messages(&mut self, can_pin_messages: Option<bool>) {
        self.can_pin_messages = can_pin_messages;
    }

    pub fn user(&self) -> User {
        self.user.clone()
    }

    pub fn can_be_edited(&self) -> bool {
        self.can_be_edited
    }

    pub fn custom_title(&self) -> Option<String> {
        self.custom_title.clone()
    }

    pub fn is_anonymous(&self) -> bool {
        self.is_anonymous
    }

    pub fn can_manage_chat(&self) -> bool {
        self.can_manage_chat
    }

    pub fn can_post_messages(&self) -> Option<bool> {
        self.can_post_messages
    }

    pub fn can_edit_messages(&self) -> Option<bool> {
        self.can_edit_messages
    }

    pub fn can_delete_messages(&self) -> bool {
        self.can_delete_messages
    }

    pub fn can_manage_voice_chats(&self) -> bool {
        self.can_manage_voice_chats
    }

    pub fn can_restrict_members(&self) -> bool {
        self.can_restrict_members
    }

    pub fn can_promote_members(&self) -> bool {
        self.can_promote_members
    }

    pub fn can_change_info(&self) -> bool {
        self.can_change_info
    }

    pub fn can_invite_users(&self) -> bool {
        self.can_invite_users
    }

    pub fn can_pin_messages(&self) -> Option<bool> {
        self.can_pin_messages
    }
}

impl ChatMemberMember {
    pub fn new(user: User) -> Self {
        Self { user }
    }

    pub fn set_user(&mut self, user: User) {
        self.user = user;
    }

    pub fn user(&self) -> User {
        self.user.clone()
    }
}

impl ChatMemberRestricted {
    pub fn new(user: User, until_date: u64) -> Self {
        Self {
            user,
            until_date,
            is_member: true,
            can_change_info: true,
            can_invite_users: true,
            can_send_messages: true,
            can_send_media_messages: true,
            can_send_polls: true,
            can_send_other_messages: true,
            can_add_web_page_previews: true,
            can_pin_messages: None,
        }
    }

    pub fn set_user(&mut self, user: User) {
        self.user = user;
    }

    pub fn set_is_member(&mut self, is_member: bool) {
        self.is_member = is_member;
    }

    pub fn set_can_change_info(&mut self, can_change_info: bool) {
        self.can_change_info = can_change_info;
    }

    pub fn set_can_invite_users(&mut self, can_invite_users: bool) {
        self.can_invite_users = can_invite_users;
    }

    pub fn set_can_pin_messages(&mut self, can_pin_messages: Option<bool>) {
        self.can_pin_messages = can_pin_messages;
    }

    pub fn set_can_send_messages(&mut self, can_send_messages: bool) {
        self.can_send_messages = can_send_messages;
    }

    pub fn set_can_send_media_messages(&mut self, can_send_media_messages: bool) {
        self.can_send_media_messages = can_send_media_messages;
    }

    pub fn set_can_send_polls(&mut self, can_send_polls: bool) {
        self.can_send_polls = can_send_polls;
    }

    pub fn set_can_send_other_messages(&mut self, can_send_other_messages: bool) {
        self.can_send_other_messages = can_send_other_messages;
    }

    pub fn set_can_add_web_page_previews(&mut self, can_add_web_page_previews: bool) {
        self.can_add_web_page_previews = can_add_web_page_previews;
    }

    pub fn set_until_date(&mut self, until_date: u64) {
        self.until_date = until_date;
    }

    pub fn user(&self) -> User {
        self.user.clone()
    }

    pub fn is_member(&self) -> bool {
        self.is_member
    }

    pub fn can_change_info(&self) -> bool {
        self.can_change_info
    }

    pub fn can_invite_users(&self) -> bool {
        self.can_invite_users
    }

    pub fn can_pin_messages(&self) -> Option<bool> {
        self.can_pin_messages
    }

    pub fn can_send_messages(&self) -> bool {
        self.can_send_messages
    }

    pub fn can_send_media_messages(&self) -> bool {
        self.can_send_media_messages
    }

    pub fn can_send_polls(&self) -> bool {
        self.can_send_polls
    }

    pub fn can_send_other_messages(&self) -> bool {
        self.can_send_other_messages
    }

    pub fn can_add_web_page_previews(&self) -> bool {
        self.can_add_web_page_previews
    }

    pub fn until_date(&self) -> u64 {
        self.until_date
    }
}

impl ChatMemberLeft {
    pub fn new(user: User) -> Self {
        Self { user }
    }

    pub fn set_user(&mut self, user: User) {
        self.user = user;
    }

    pub fn user(&self) -> User {
        self.user.clone()
    }
}

impl ChatMemberBanned {
    pub fn new(user: User, until_date: u64) -> Self {
        Self { user, until_date }
    }

    pub fn set_user(&mut self, user: User) {
        self.user = user;
    }

    pub fn set_until_date(&mut self, until_date: u64) {
        self.until_date = until_date;
    }

    pub fn user(&self) -> User {
        self.user.clone()
    }

    pub fn until_date(&self) -> u64 {
        self.until_date
    }
}
