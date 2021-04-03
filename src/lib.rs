#[derive(Debug)]
enum ThumbEnum {
    InputFileVariant(InputFile),
    StringVariant(String),
}

#[derive(Debug)]
enum ChatIdEnum {
    IsizeVariant(isize),
    StringVariant(String),
}

#[derive(Debug)]
enum ReplyMarkupEnum {
    InlineKeyboardMarkupVariant(InlineKeyboardMarkup),
    ReplyKeyboardMarkupVariant(ReplyKeyboardMarkup),
    ReplyKeyboardRemoveVariant(ReplyKeyboardRemove),
    ForceReplyVariant(ForceReply),
}

#[derive(Debug)]
enum FromChatIdEnum {
    IsizeVariant(isize),
    StringVariant(String),
}

#[derive(Debug)]
enum PhotoEnum {
    InputFileVariant(InputFile),
    StringVariant(String),
}

#[derive(Debug)]
enum AudioEnum {
    InputFileVariant(InputFile),
    StringVariant(String),
}

#[derive(Debug)]
enum DocumentEnum {
    InputFileVariant(InputFile),
    StringVariant(String),
}

#[derive(Debug)]
enum VideoEnum {
    InputFileVariant(InputFile),
    StringVariant(String),
}

#[derive(Debug)]
enum AnimationEnum {
    InputFileVariant(InputFile),
    StringVariant(String),
}

#[derive(Debug)]
enum VoiceEnum {
    InputFileVariant(InputFile),
    StringVariant(String),
}

#[derive(Debug)]
enum VideoNoteEnum {
    InputFileVariant(InputFile),
    StringVariant(String),
}

#[derive(Debug)]
enum MediaEnum {
    InputMediaAudioVariant(InputMediaAudio),
    InputMediaDocumentVariant(InputMediaDocument),
    InputMediaPhotoVariant(InputMediaPhoto),
    InputMediaVideoVariant(InputMediaVideo),
}

#[derive(Debug)]
enum InputMessageContent {
    InputTextMessageContentVariant(InputTextMessageContent),
    InputLocationMessageContentVariant(InputLocationMessageContent),
    InputVenueMessageContentVariant(InputLocationMessageContent),
    InputContactMessageContentVariant(InputLocationMessageContent),
}

#[derive(Debug)]
enum StickerEnum {
    InputFileVariant(InputFile),
    StringVariant(String),
}

#[derive(Debug)]
enum PngStickerEnum {
    InputFileVariant(InputFile),
    StringVariant(String),
}

#[derive(Debug)]
enum InlineQueryResult {
    InlineQueryResultCachedAudioVariant(InlineQueryResultCachedAudio),
    InlineQueryResultCachedDocumentVariant(InlineQueryResultCachedDocument),
    InlineQueryResultCachedGifVariant(InlineQueryResultCachedGif),
    InlineQueryResultCachedMpeg4GifVariant(InlineQueryResultCachedMpeg4Gif),
    InlineQueryResultCachedPhotoVariant(InlineQueryResultCachedPhoto),
    InlineQueryResultCachedStickerVariant(InlineQueryResultCachedSticker),
    InlineQueryResultCachedVideoVariant(InlineQueryResultCachedVideo),
    InlineQueryResultCachedVoiceVariant(InlineQueryResultCachedVoice),
    InlineQueryResultArticleVariant(InlineQueryResultArticle),
    InlineQueryResultAudioVariant(InlineQueryResultAudio),
    InlineQueryResultContactVariant(InlineQueryResultContact),
    InlineQueryResultGameVariant(InlineQueryResultGame),
    InlineQueryResultDocumentVariant(InlineQueryResultDocument),
    InlineQueryResultGifVariant(InlineQueryResultGif),
    InlineQueryResultLocationVariant(InlineQueryResultLocation),
    InlineQueryResultMpeg4GifVariant(InlineQueryResultMpeg4Gif),
    InlineQueryResultPhotoVariant(InlineQueryResultPhoto),
    InlineQueryResultVenueVariant(InlineQueryResultVenue),
    InlineQueryResultVideoVariant(InlineQueryResultVideo),
    InlineQueryResultVoiceVariant(InlineQueryResultVoice),
}

#[derive(Debug)]
enum InputMedia {
    InputMediaAnimationVariant(InputMediaAnimation),
    InputMediaDocumentVariant(InputMediaDocument),
    InputMediaAudioVariant(InputMediaAudio),
    InputMediaPhotoVariant(InputMediaPhoto),
    InputMediaVideoVariant(InputMediaVideo),
}

#[derive(Debug)]
enum PassportElementError {
    PassportElementErrorDataFieldVariant(PassportElementErrorDataField),
    PassportElementErrorFrontSideVariant(PassportElementErrorFrontSide),
    PassportElementErrorReverseSideVariant(PassportElementErrorReverseSide),
    PassportElementErrorSelfieVariant(PassportElementErrorSelfie),
    PassportElementErrorFileVariant(PassportElementErrorFile),
    PassportElementErrorFilesVariant(PassportElementErrorFiles),
    PassportElementErrorTranslationFileVariant(PassportElementErrorTranslationFile),
    PassportElementErrorTranslationFilesVariant(PassportElementErrorTranslationFiles),
    PassportElementErrorUnspecifiedVariant(PassportElementErrorUnspecified),
}

#[derive(Debug)]
struct InputFile {}

#[derive(Debug)]
struct VoiceChatStarted {}

#[derive(Debug)]
struct CallbackGame {}

#[derive(Debug)]
struct Update {
    update_id: isize,
    message: Option<Message>,
    edited_message: Option<Message>,
    channel_post: Option<Message>,
    edited_channel_post: Option<Message>,
    inline_query: Option<InlineQuery>,
    chosen_inline_result: Option<ChosenInlineResult>,
    callback_query: Option<CallbackQuery>,
    shipping_query: Option<ShippingQuery>,
    pre_checkout_query: Option<PreCheckoutQuery>,
    poll: Option<Poll>,
    poll_answer: Option<PollAnswer>,
    my_chat_member: Option<ChatMemberUpdated>,
    chat_member: Option<ChatMemberUpdated>,
}

#[derive(Debug)]
struct WebhookInfo {
    url: String,
    has_custom_certificate: bool,
    pending_update_count: isize,
    ip_address: Option<String>,
    last_error_date: Option<isize>,
    last_error_message: Option<String>,
    max_connections: Option<isize>,
    allowed_updates: Option<Vec<String>>,
}

#[derive(Debug)]
struct User {
    id: isize,
    is_bot: bool,
    first_name: String,
    last_name: Option<String>,
    username: Option<String>,
    language_code: Option<String>,
    can_join_groups: Option<bool>,
    can_read_all_group_messages: Option<bool>,
    supports_inline_queries: Option<bool>,
}

#[derive(Debug)]
struct Chat {
    id: isize,
    type_field: String,
    title: Option<String>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    photo: Option<ChatPhoto>,
    bio: Option<String>,
    description: Option<String>,
    invite_link: Option<String>,
    pinned_message: Option<Message>,
    permissions: Option<ChatPermissions>,
    slow_mode_delay: Option<isize>,
    message_auto_delete_time: Option<isize>,
    sticker_set_name: Option<String>,
    can_set_sticker_set: Option<bool>,
    linked_chat_id: Option<isize>,
    location: Option<ChatLocation>,
}

#[derive(Debug)]
struct Message {
    message_id: isize,
    from: Option<User>,
    sender_chat: Option<Box<Chat>>,
    date: isize,
    chat: Box<Chat>,
    forward_from: Option<User>,
    forward_from_chat: Option<Box<Chat>>,
    forward_from_message_id: Option<isize>,
    forward_signature: Option<String>,
    forward_sender_name: Option<String>,
    forward_date: Option<isize>,
    reply_to_message: Option<Box<Message>>,
    via_bot: Option<User>,
    edit_date: Option<isize>,
    media_group_id: Option<String>,
    author_signature: Option<String>,
    text: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    animation: Option<Animation>,
    audio: Option<Audio>,
    document: Option<Document>,
    photo: Option<Vec<PhotoSize>>,
    sticker: Option<Sticker>,
    video: Option<Video>,
    video_note: Option<VideoNote>,
    voice: Option<Voice>,
    caption: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    contact: Option<Contact>,
    dice: Option<Dice>,
    game: Option<Game>,
    poll: Option<Poll>,
    venue: Option<Venue>,
    location: Option<Location>,
    new_chat_members: Option<Vec<User>>,
    left_chat_member: Option<User>,
    new_chat_title: Option<String>,
    new_chat_photo: Option<Vec<PhotoSize>>,
    delete_chat_photo: Option<bool>,
    group_chat_created: Option<bool>,
    supergroup_chat_created: Option<bool>,
    channel_chat_created: Option<bool>,
    message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    migrate_to_chat_id: Option<isize>,
    migrate_from_chat_id: Option<isize>,
    pinned_message: Option<Box<Message>>,
    invoice: Option<Invoice>,
    successful_payment: Option<SuccessfulPayment>,
    connected_website: Option<String>,
    passport_data: Option<PassportData>,
    proximity_alert_triggered: Option<ProximityAlertTriggered>,
    voice_chat_started: Option<VoiceChatStarted>,
    voice_chat_ended: Option<VoiceChatEnded>,
    voice_chat_participants_invited: Option<VoiceChatParticipantsInvited>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
struct MessageId {
    message_id: isize,
}

#[derive(Debug)]
struct MessageEntity {
    type_field: String,
    offset: isize,
    length: isize,
    url: Option<String>,
    user: Option<User>,
    language: Option<String>,
}

#[derive(Debug)]
struct PhotoSize {
    file_id: String,
    file_unique_id: String,
    width: isize,
    height: isize,
    file_size: Option<isize>,
}

#[derive(Debug)]
struct Animation {
    file_id: String,
    file_unique_id: String,
    width: isize,
    height: isize,
    duration: isize,
    thumb: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<isize>,
}

#[derive(Debug)]
struct Audio {
    file_id: String,
    file_unique_id: String,
    duration: isize,
    performer: Option<String>,
    title: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<isize>,
    thumb: Option<PhotoSize>,
}

#[derive(Debug)]
struct Document {
    file_id: String,
    file_unique_id: String,
    thumb: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<isize>,
}

#[derive(Debug)]
struct Video {
    file_id: String,
    file_unique_id: String,
    width: isize,
    height: isize,
    duration: isize,
    thumb: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<isize>,
}

#[derive(Debug)]
struct VideoNote {
    file_id: String,
    file_unique_id: String,
    length: isize,
    duration: isize,
    thumb: Option<PhotoSize>,
    file_size: Option<isize>,
}

#[derive(Debug)]
struct Voice {
    file_id: String,
    file_unique_id: String,
    duration: isize,
    mime_type: Option<String>,
    file_size: Option<isize>,
}

#[derive(Debug)]
struct Contact {
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    user_id: Option<isize>,
    vcard: Option<String>,
}

#[derive(Debug)]
struct Dice {
    emoji: String,
    value: isize,
}

#[derive(Debug)]
struct PollOption {
    text: String,
    voter_count: isize,
}

#[derive(Debug)]
struct PollAnswer {
    poll_id: String,
    user: User,
    option_ids: Vec<isize>,
}

#[derive(Debug)]
struct Poll {
    id: String,
    question: String,
    options: Vec<PollOption>,
    total_voter_count: isize,
    is_closed: bool,
    is_anonymous: bool,
    type_field: String,
    allows_multiple_answers: bool,
    correct_option_id: Option<isize>,
    explanation: Option<String>,
    explanation_entities: Option<Vec<MessageEntity>>,
    open_period: Option<isize>,
    close_date: Option<isize>,
}

#[derive(Debug)]
struct Location {
    longitude: f64,
    latitude: f64,
    horizontal_accuracy: Option<f64>,
    live_period: Option<isize>,
    heading: Option<isize>,
    proximity_alert_radius: Option<isize>,
}

#[derive(Debug)]
struct Venue {
    location: Location,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
}

#[derive(Debug)]
struct ProximityAlertTriggered {
    traveler: User,
    watcher: User,
    distance: isize,
}

#[derive(Debug)]
struct MessageAutoDeleteTimerChanged {
    message_auto_delete_time: isize,
}

#[derive(Debug)]
struct VoiceChatEnded {
    duration: isize,
}

#[derive(Debug)]
struct VoiceChatParticipantsInvited {
    users: Option<Vec<User>>,
}

#[derive(Debug)]
struct UserProfilePhotos {
    total_count: isize,
    photos: Vec<PhotoSize>,
}

#[derive(Debug)]
struct File {
    file_id: String,
    file_unique_id: String,
    file_size: Option<isize>,
    file_path: Option<String>,
}

#[derive(Debug)]
struct ReplyKeyboardMarkup {
    keyboard: Vec<KeyboardButton>,
    resize_keyboard: Option<bool>,
    one_time_keyboard: Option<bool>,
    selective: Option<bool>,
}

#[derive(Debug)]
struct KeyboardButton {
    text: String,
    request_contact: Option<bool>,
    request_location: Option<bool>,
    request_poll: Option<KeyboardButtonPollType>,
}

#[derive(Debug)]
struct KeyboardButtonPollType {
    type_field: Option<String>,
}

#[derive(Debug)]
struct ReplyKeyboardRemove {
    remove_keyboard: bool,
    selective: Option<bool>,
}

#[derive(Debug)]
struct InlineKeyboardMarkup {
    inline_keyboard: Vec<InlineKeyboardButton>,
}

#[derive(Debug)]
struct InlineKeyboardButton {
    text: String,
    url: Option<String>,
    login_url: Option<LoginUrl>,
    callback_data: Option<String>,
    switch_inline_query: Option<String>,
    switch_inline_query_current_chat: Option<String>,
    callback_game: Option<CallbackGame>,
    pay: Option<bool>,
}

#[derive(Debug)]
struct LoginUrl {
    url: String,
    forward_text: Option<String>,
    bot_username: Option<String>,
    request_write_access: Option<bool>,
}

#[derive(Debug)]
struct CallbackQuery {
    id: String,
    from: User,
    message: Option<Message>,
    inline_message_id: Option<String>,
    chat_instance: String,
    data: Option<String>,
    game_short_name: Option<String>,
}

#[derive(Debug)]
struct ForceReply {
    force_reply: bool,
    selective: Option<bool>,
}

#[derive(Debug)]
struct ChatPhoto {
    small_file_id: String,
    small_file_unique_id: String,
    big_file_id: String,
    big_file_unique_id: String,
}

#[derive(Debug)]
struct ChatInviteLink {
    invite_link: String,
    creator: User,
    is_primary: bool,
    is_revoked: bool,
    expire_date: Option<isize>,
    member_limit: Option<isize>,
}

#[derive(Debug)]
struct ChatMember {
    user: User,
    status: String,
    custom_title: Option<String>,
    is_anonymous: Option<bool>,
    can_be_edited: Option<bool>,
    can_manage_chat: Option<bool>,
    can_post_messages: Option<bool>,
    can_edit_messages: Option<bool>,
    can_delete_messages: Option<bool>,
    can_manage_voice_chats: Option<bool>,
    can_restrict_members: Option<bool>,
    can_promote_members: Option<bool>,
    can_change_info: Option<bool>,
    can_invite_users: Option<bool>,
    can_pin_messages: Option<bool>,
    is_member: Option<bool>,
    can_send_messages: Option<bool>,
    can_send_media_messages: Option<bool>,
    can_send_polls: Option<bool>,
    can_send_other_messages: Option<bool>,
    can_add_web_page_previews: Option<bool>,
    until_date: Option<isize>,
}

#[derive(Debug)]
struct ChatMemberUpdated {
    chat: Chat,
    from: User,
    date: isize,
    old_chat_member: ChatMember,
    new_chat_member: ChatMember,
    invite_link: Option<ChatInviteLink>,
}

#[derive(Debug)]
struct ChatPermissions {
    can_send_messages: Option<bool>,
    can_send_media_messages: Option<bool>,
    can_send_polls: Option<bool>,
    can_send_other_messages: Option<bool>,
    can_add_web_page_previews: Option<bool>,
    can_change_info: Option<bool>,
    can_invite_users: Option<bool>,
    can_pin_messages: Option<bool>,
}

#[derive(Debug)]
struct ChatLocation {
    location: Location,
    address: String,
}

#[derive(Debug)]
struct BotCommand {
    command: String,
    description: String,
}

#[derive(Debug)]
struct ResponseParameters {
    migrate_to_chat_id: Option<isize>,
    retry_after: Option<isize>,
}

#[derive(Debug)]
struct InputMediaPhoto {
    type_field: String,
    media: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
}

#[derive(Debug)]
struct InputMediaVideo {
    type_field: String,
    media: String,
    thumb: Option<ThumbEnum>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    width: Option<isize>,
    height: Option<isize>,
    duration: Option<isize>,
    supports_streaming: Option<bool>,
}

#[derive(Debug)]
struct InputMediaAnimation {
    type_field: String,
    media: String,
    thumb: Option<ThumbEnum>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    width: Option<isize>,
    height: Option<isize>,
    duration: Option<isize>,
}

#[derive(Debug)]
struct InputMediaAudio {
    type_field: String,
    media: String,
    thumb: Option<ThumbEnum>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    duration: Option<isize>,
    performer: Option<String>,
    title: Option<String>,
}

#[derive(Debug)]
struct InputMediaDocument {
    type_field: String,
    media: String,
    thumb: Option<ThumbEnum>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    disable_content_type_detection: Option<bool>,
}

#[derive(Debug)]
struct Sticker {
    file_id: String,
    file_unique_id: String,
    width: isize,
    height: isize,
    is_animated: bool,
    thumb: Option<PhotoSize>,
    emoji: Option<String>,
    set_name: Option<String>,
    mask_position: Option<MaskPosition>,
    file_size: Option<isize>,
}

#[derive(Debug)]
struct StickerSet {
    name: String,
    title: String,
    is_animated: bool,
    contains_masks: bool,
    stickers: Vec<Sticker>,
    thumb: Option<PhotoSize>,
}

#[derive(Debug)]
struct MaskPosition {
    point: String,
    x_shift: f64,
    y_shift: f64,
    scale: f64,
}

#[derive(Debug)]
struct InlineQuery {
    id: String,
    from: User,
    location: Option<Location>,
    query: String,
    offset: String,
}

#[derive(Debug)]
struct InlineQueryResultArticle {
    type_field: String,
    id: String,
    title: String,
    input_message_content: InputMessageContent,
    reply_markup: Option<InlineKeyboardMarkup>,
    url: Option<String>,
    hide_url: Option<bool>,
    description: Option<String>,
    thumb_url: Option<String>,
    thumb_width: Option<isize>,
    thumb_height: Option<isize>,
}

#[derive(Debug)]
struct InlineQueryResultPhoto {
    type_field: String,
    id: String,
    photo_url: String,
    thumb_url: String,
    photo_width: Option<isize>,
    photo_height: Option<isize>,
    title: Option<String>,
    description: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
struct InlineQueryResultGif {
    type_field: String,
    id: String,
    gif_url: String,
    gif_width: Option<isize>,
    gif_height: Option<isize>,
    gif_duration: Option<isize>,
    thumb_url: String,
    thumb_mime_type: Option<String>,
    title: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
struct InlineQueryResultMpeg4Gif {
    type_field: String,
    id: String,
    mpeg4_url: String,
    mpeg4_width: Option<isize>,
    mpeg4_height: Option<isize>,
    mpeg4_duration: Option<isize>,
    thumb_url: String,
    thumb_mime_type: Option<String>,
    title: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
struct InlineQueryResultVideo {
    type_field: String,
    id: String,
    video_url: String,
    mime_type: String,
    thumb_url: String,
    title: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    video_width: Option<isize>,
    video_height: Option<isize>,
    video_duration: Option<isize>,
    description: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
struct InlineQueryResultAudio {
    type_field: String,
    id: String,
    audio_url: String,
    title: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    performer: Option<String>,
    audio_duration: Option<isize>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
struct InlineQueryResultVoice {
    type_field: String,
    id: String,
    voice_url: String,
    title: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    voice_duration: Option<isize>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
struct InlineQueryResultDocument {
    type_field: String,
    id: String,
    title: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    document_url: String,
    mime_type: String,
    description: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
    thumb_url: Option<String>,
    thumb_width: Option<isize>,
    thumb_height: Option<isize>,
}

#[derive(Debug)]
struct InlineQueryResultLocation {
    type_field: String,
    id: String,
    latitude: f64,
    longitude: f64,
    title: String,
    horizontal_accuracy: Option<f64>,
    live_period: Option<isize>,
    heading: Option<isize>,
    proximity_alert_radius: Option<isize>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
    thumb_url: Option<String>,
    thumb_width: Option<isize>,
    thumb_height: Option<isize>,
}

#[derive(Debug)]
struct InlineQueryResultVenue {
    type_field: String,
    id: String,
    latitude: f64,
    longitude: f64,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
    thumb_url: Option<String>,
    thumb_width: Option<isize>,
    thumb_height: Option<isize>,
}

#[derive(Debug)]
struct InlineQueryResultContact {
    type_field: String,
    id: String,
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    vcard: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
    thumb_url: Option<String>,
    thumb_width: Option<isize>,
    thumb_height: Option<isize>,
}

#[derive(Debug)]
struct InlineQueryResultGame {
    type_field: String,
    id: String,
    game_short_name: String,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
struct InlineQueryResultCachedPhoto {
    type_field: String,
    id: String,
    photo_file_id: String,
    title: Option<String>,
    description: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
struct InlineQueryResultCachedGif {
    type_field: String,
    id: String,
    gif_file_id: String,
    title: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
struct InlineQueryResultCachedMpeg4Gif {
    type_field: String,
    id: String,
    mpeg4_file_id: String,
    title: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
struct InlineQueryResultCachedSticker {
    type_field: String,
    id: String,
    sticker_file_id: String,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
struct InlineQueryResultCachedDocument {
    type_field: String,
    id: String,
    title: String,
    document_file_id: String,
    description: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
struct InlineQueryResultCachedVideo {
    type_field: String,
    id: String,
    video_file_id: String,
    title: String,
    description: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
struct InlineQueryResultCachedVoice {
    type_field: String,
    id: String,
    voice_file_id: String,
    title: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
struct InlineQueryResultCachedAudio {
    type_field: String,
    id: String,
    audio_file_id: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
struct InputTextMessageContent {
    message_text: String,
    parse_mode: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    disable_web_page_preview: Option<bool>,
}

#[derive(Debug)]
struct InputLocationMessageContent {
    latitude: f64,
    longitude: f64,
    horizontal_accuracy: Option<f64>,
    live_period: Option<isize>,
    heading: Option<isize>,
    proximity_alert_radius: Option<isize>,
}

#[derive(Debug)]
struct InputVenueMessageContent {
    latitude: f64,
    longitude: f64,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
}

#[derive(Debug)]
struct InputContactMessageContent {
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    vcard: Option<String>,
}

#[derive(Debug)]
struct ChosenInlineResult {
    result_id: String,
    from: User,
    location: Option<Location>,
    inline_message_id: Option<String>,
    query: String,
}

#[derive(Debug)]
struct LabeledPrice {
    label: String,
    amount: isize,
}

#[derive(Debug)]
struct Invoice {
    title: String,
    description: String,
    start_parameter: String,
    currency: String,
    total_amount: isize,
}

#[derive(Debug)]
struct ShippingAddress {
    country_code: String,
    state: String,
    city: String,
    street_line1: String,
    street_line2: String,
    post_code: String,
}

#[derive(Debug)]
struct OrderInfo {
    name: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    shipping_address: Option<ShippingAddress>,
}

#[derive(Debug)]
struct ShippingOption {
    id: String,
    title: String,
    prices: Vec<LabeledPrice>,
}

#[derive(Debug)]
struct SuccessfulPayment {
    currency: String,
    total_amount: isize,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
    telegram_payment_charge_id: String,
    provider_payment_charge_id: String,
}

#[derive(Debug)]
struct ShippingQuery {
    id: String,
    from: User,
    invoice_payload: String,
    shipping_address: ShippingAddress,
}

#[derive(Debug)]
struct PreCheckoutQuery {
    id: String,
    from: User,
    currency: String,
    total_amount: isize,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
}

#[derive(Debug)]
struct PassportData {
    data: Vec<EncryptedPassportElement>,
    credentials: EncryptedCredentials,
}

#[derive(Debug)]
struct PassportFile {
    file_id: String,
    file_unique_id: String,
    file_size: isize,
    file_date: isize,
}

#[derive(Debug)]
struct EncryptedPassportElement {
    type_field: String,
    data: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    files: Option<Vec<PassportFile>>,
    front_side: Option<PassportFile>,
    reverse_side: Option<PassportFile>,
    selfie: Option<PassportFile>,
    translation: Option<Vec<PassportFile>>,
    hash: String,
}

#[derive(Debug)]
struct EncryptedCredentials {
    data: String,
    hash: String,
    secret: String,
}

#[derive(Debug)]
struct PassportElementErrorDataField {
    source: String,
    type_field: String,
    field_name: String,
    data_hash: String,
    message: String,
}

#[derive(Debug)]
struct PassportElementErrorFrontSide {
    source: String,
    type_field: String,
    file_hash: String,
    message: String,
}

#[derive(Debug)]
struct PassportElementErrorReverseSide {
    source: String,
    type_field: String,
    file_hash: String,
    message: String,
}

#[derive(Debug)]
struct PassportElementErrorSelfie {
    source: String,
    type_field: String,
    file_hash: String,
    message: String,
}

#[derive(Debug)]
struct PassportElementErrorFile {
    source: String,
    type_field: String,
    file_hash: String,
    message: String,
}

#[derive(Debug)]
struct PassportElementErrorFiles {
    source: String,
    type_field: String,
    file_hashes: Vec<String>,
    message: String,
}

#[derive(Debug)]
struct PassportElementErrorTranslationFile {
    source: String,
    type_field: String,
    file_hash: String,
    message: String,
}

#[derive(Debug)]
struct PassportElementErrorTranslationFiles {
    source: String,
    type_field: String,
    file_hashes: Vec<String>,
    message: String,
}

#[derive(Debug)]
struct PassportElementErrorUnspecified {
    source: String,
    type_field: String,
    element_hash: String,
    message: String,
}

#[derive(Debug)]
struct Game {
    title: String,
    description: String,
    photo: Vec<PhotoSize>,
    text: Option<String>,
    text_entities: Option<Vec<MessageEntity>>,
    animation: Option<Animation>,
}

#[derive(Debug)]
struct GameHighScore {
    position: isize,
    user: User,
    score: isize,
}

#[derive(Debug)]
struct GetUpdatesParams {
    offset: Option<isize>,
    limit: Option<isize>,
    timeout: Option<isize>,
    allowed_updates: Option<Vec<String>>,
}

#[derive(Debug)]
struct SetWebhookParams {
    url: String,
    certificate: Option<InputFile>,
    ip_address: Option<String>,
    max_connections: Option<isize>,
    allowed_updates: Option<Vec<String>>,
    drop_pending_updates: Option<bool>,
}

#[derive(Debug)]
struct DeleteWebhookParams {
    drop_pending_updates: Option<bool>,
}

#[derive(Debug)]
struct SendMessageParams {
    chat_id: ChatIdEnum,
    text: String,
    parse_mode: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    disable_web_page_preview: Option<bool>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
struct ForwardMessageParams {
    chat_id: ChatIdEnum,
    from_chat_id: FromChatIdEnum,
    disable_notification: Option<bool>,
    message_id: isize,
}

#[derive(Debug)]
struct CopyMessageParams {
    chat_id: ChatIdEnum,
    from_chat_id: FromChatIdEnum,
    message_id: isize,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
struct SendPhotoParams {
    chat_id: ChatIdEnum,
    photo: PhotoEnum,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
struct SendAudioParams {
    chat_id: ChatIdEnum,
    audio: AudioEnum,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    duration: Option<isize>,
    performer: Option<String>,
    title: Option<String>,
    thumb: Option<ThumbEnum>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
struct SendDocumentParams {
    chat_id: ChatIdEnum,
    document: DocumentEnum,
    thumb: Option<ThumbEnum>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    disable_content_type_detection: Option<bool>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
struct SendVideoParams {
    chat_id: ChatIdEnum,
    video: VideoEnum,
    duration: Option<isize>,
    width: Option<isize>,
    height: Option<isize>,
    thumb: Option<ThumbEnum>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    supports_streaming: Option<bool>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
struct SendAnimationParams {
    chat_id: ChatIdEnum,
    animation: AnimationEnum,
    duration: Option<isize>,
    width: Option<isize>,
    height: Option<isize>,
    thumb: Option<ThumbEnum>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
struct SendVoiceParams {
    chat_id: ChatIdEnum,
    voice: VoiceEnum,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    duration: Option<isize>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
struct SendVideoNoteParams {
    chat_id: ChatIdEnum,
    video_note: VideoNoteEnum,
    duration: Option<isize>,
    length: Option<isize>,
    thumb: Option<ThumbEnum>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
struct SendMediaGroupParams {
    chat_id: ChatIdEnum,
    media: Vec<MediaEnum>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
}

#[derive(Debug)]
struct SendLocationParams {
    chat_id: ChatIdEnum,
    latitude: f64,
    longitude: f64,
    horizontal_accuracy: Option<f64>,
    live_period: Option<isize>,
    heading: Option<isize>,
    proximity_alert_radius: Option<isize>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
struct EditMessageLiveLocationParams {
    chat_id: Option<ChatIdEnum>,
    message_id: Option<isize>,
    inline_message_id: Option<String>,
    latitude: f64,
    longitude: f64,
    horizontal_accuracy: Option<f64>,
    heading: Option<isize>,
    proximity_alert_radius: Option<isize>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
struct StopMessageLiveLocationParams {
    chat_id: Option<ChatIdEnum>,
    message_id: Option<isize>,
    inline_message_id: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
struct SendVenueParams {
    chat_id: ChatIdEnum,
    latitude: f64,
    longitude: f64,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
struct SendContactParams {
    chat_id: ChatIdEnum,
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    vcard: Option<String>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
struct SendPollParams {
    chat_id: ChatIdEnum,
    question: String,
    options: Vec<String>,
    is_anonymous: Option<bool>,
    type_field: Option<String>,
    allows_multiple_answers: Option<bool>,
    correct_option_id: Option<isize>,
    explanation: Option<String>,
    explanation_parse_mode: Option<String>,
    explanation_entities: Option<Vec<MessageEntity>>,
    open_period: Option<isize>,
    close_date: Option<isize>,
    is_closed: Option<bool>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
struct SendDiceParams {
    chat_id: ChatIdEnum,
    emoji: Option<String>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
struct SendChatActionParams {
    chat_id: ChatIdEnum,
    action: String,
}

#[derive(Debug)]
struct GetUserProfilePhotosParams {
    user_id: isize,
    offset: Option<isize>,
    limit: Option<isize>,
}

#[derive(Debug)]
struct GetFileParams {
    file_id: String,
}

#[derive(Debug)]
struct KickChatMemberParams {
    chat_id: ChatIdEnum,
    user_id: isize,
    until_date: Option<isize>,
    revoke_messages: Option<bool>,
}

#[derive(Debug)]
struct UnbanChatMemberParams {
    chat_id: ChatIdEnum,
    user_id: isize,
    only_if_banned: Option<bool>,
}

#[derive(Debug)]
struct RestrictChatMemberParams {
    chat_id: ChatIdEnum,
    user_id: isize,
    permissions: ChatPermissions,
    until_date: Option<isize>,
}

#[derive(Debug)]
struct PromoteChatMemberParams {
    chat_id: ChatIdEnum,
    user_id: isize,
    is_anonymous: Option<bool>,
    can_manage_chat: Option<bool>,
    can_post_messages: Option<bool>,
    can_edit_messages: Option<bool>,
    can_delete_messages: Option<bool>,
    can_manage_voice_chats: Option<bool>,
    can_restrict_members: Option<bool>,
    can_promote_members: Option<bool>,
    can_change_info: Option<bool>,
    can_invite_users: Option<bool>,
    can_pin_messages: Option<bool>,
}

#[derive(Debug)]
struct SetChatAdministratorCustomTitleParams {
    chat_id: ChatIdEnum,
    user_id: isize,
    custom_title: String,
}

#[derive(Debug)]
struct SetChatPermissionsParams {
    chat_id: ChatIdEnum,
    permissions: ChatPermissions,
}

#[derive(Debug)]
struct ExportChatInviteLinkParams {
    chat_id: ChatIdEnum,
}

#[derive(Debug)]
struct CreateChatInviteLinkParams {
    chat_id: ChatIdEnum,
    expire_date: Option<isize>,
    member_limit: Option<isize>,
}

#[derive(Debug)]
struct EditChatInviteLinkParams {
    chat_id: ChatIdEnum,
    invite_link: String,
    expire_date: Option<isize>,
    member_limit: Option<isize>,
}

#[derive(Debug)]
struct RevokeChatInviteLinkParams {
    chat_id: ChatIdEnum,
    invite_link: String,
}

#[derive(Debug)]
struct SetChatPhotoParams {
    chat_id: ChatIdEnum,
    photo: InputFile,
}

#[derive(Debug)]
struct DeleteChatPhotoParams {
    chat_id: ChatIdEnum,
}

#[derive(Debug)]
struct SetChatTitleParams {
    chat_id: ChatIdEnum,
    title: String,
}

#[derive(Debug)]
struct SetChatDescriptionParams {
    chat_id: ChatIdEnum,
    description: Option<String>,
}

#[derive(Debug)]
struct PinChatMessageParams {
    chat_id: ChatIdEnum,
    message_id: isize,
    disable_notification: Option<bool>,
}

#[derive(Debug)]
struct UnpinChatMessageParams {
    chat_id: ChatIdEnum,
    message_id: Option<isize>,
}

#[derive(Debug)]
struct UnpinAllChatMessagesParams {
    chat_id: ChatIdEnum,
}

#[derive(Debug)]
struct LeaveChatParams {
    chat_id: ChatIdEnum,
}

#[derive(Debug)]
struct GetChatParams {
    chat_id: ChatIdEnum,
}

#[derive(Debug)]
struct GetChatAdministratorsParams {
    chat_id: ChatIdEnum,
}

#[derive(Debug)]
struct GetChatMembersCountParams {
    chat_id: ChatIdEnum,
}

#[derive(Debug)]
struct GetChatMemberParams {
    chat_id: ChatIdEnum,
    user_id: isize,
}

#[derive(Debug)]
struct SetChatStickerSetParams {
    chat_id: ChatIdEnum,
    sticker_set_name: String,
}

#[derive(Debug)]
struct DeleteChatStickerSetParams {
    chat_id: ChatIdEnum,
}

#[derive(Debug)]
struct AnswerCallbackQueryParams {
    callback_query_id: String,
    text: Option<String>,
    show_alert: Option<bool>,
    url: Option<String>,
    cache_time: Option<isize>,
}

#[derive(Debug)]
struct SetMyCommandsParams {
    commands: Vec<BotCommand>,
}

#[derive(Debug)]
struct EditMessageTextParams {
    chat_id: Option<ChatIdEnum>,
    message_id: Option<isize>,
    inline_message_id: Option<String>,
    text: String,
    parse_mode: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    disable_web_page_preview: Option<bool>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
struct EditMessageCaptionParams {
    chat_id: Option<ChatIdEnum>,
    message_id: Option<isize>,
    inline_message_id: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
struct EditMessageMediaParams {
    chat_id: Option<ChatIdEnum>,
    message_id: Option<isize>,
    inline_message_id: Option<String>,
    media: InputMedia,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
struct EditMessageReplyMarkupParams {
    chat_id: Option<ChatIdEnum>,
    message_id: Option<isize>,
    inline_message_id: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
struct StopPollParams {
    chat_id: ChatIdEnum,
    message_id: isize,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
struct DeleteMessageParams {
    chat_id: ChatIdEnum,
    message_id: isize,
}

#[derive(Debug)]
struct SendStickerParams {
    chat_id: ChatIdEnum,
    sticker: StickerEnum,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
struct GetStickerSetParams {
    name: String,
}

#[derive(Debug)]
struct UploadStickerFileParams {
    user_id: isize,
    png_sticker: InputFile,
}

#[derive(Debug)]
struct CreateNewStickerSetParams {
    user_id: isize,
    name: String,
    title: String,
    png_sticker: Option<PngStickerEnum>,
    tgs_sticker: Option<InputFile>,
    emojis: String,
    contains_masks: Option<bool>,
    mask_position: Option<MaskPosition>,
}

#[derive(Debug)]
struct AddStickerToSetParams {
    user_id: isize,
    name: String,
    png_sticker: Option<PngStickerEnum>,
    tgs_sticker: Option<InputFile>,
    emojis: String,
    mask_position: Option<MaskPosition>,
}

#[derive(Debug)]
struct SetStickerPositionInSetParams {
    sticker: String,
    position: isize,
}

#[derive(Debug)]
struct DeleteStickerFromSetParams {
    sticker: String,
}

#[derive(Debug)]
struct SetStickerSetThumbParams {
    name: String,
    user_id: isize,
    thumb: Option<ThumbEnum>,
}

#[derive(Debug)]
struct AnswerInlineQueryParams {
    inline_query_id: String,
    results: Vec<InlineQueryResult>,
    cache_time: Option<isize>,
    is_personal: Option<bool>,
    next_offset: Option<String>,
    switch_pm_text: Option<String>,
    switch_pm_parameter: Option<String>,
}

#[derive(Debug)]
struct SendInvoiceParams {
    chat_id: isize,
    title: String,
    description: String,
    payload: String,
    provider_token: String,
    start_parameter: String,
    currency: String,
    prices: Vec<LabeledPrice>,
    provider_data: Option<String>,
    photo_url: Option<String>,
    photo_size: Option<isize>,
    photo_width: Option<isize>,
    photo_height: Option<isize>,
    need_name: Option<bool>,
    need_phone_number: Option<bool>,
    need_email: Option<bool>,
    need_shipping_address: Option<bool>,
    send_phone_number_to_provider: Option<bool>,
    send_email_to_provider: Option<bool>,
    is_flexible: Option<bool>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
struct AnswerShippingQueryParams {
    shipping_query_id: String,
    ok: bool,
    shipping_options: Option<Vec<ShippingOption>>,
    error_message: Option<String>,
}

#[derive(Debug)]
struct AnswerPreCheckoutQueryParams {
    pre_checkout_query_id: String,
    ok: bool,
    error_message: Option<String>,
}

#[derive(Debug)]
struct SetPassportDataErrorsParams {
    user_id: isize,
    errors: Vec<PassportElementError>,
}

#[derive(Debug)]
struct SendGameParams {
    chat_id: isize,
    game_short_name: String,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
struct SetGameScoreParams {
    user_id: isize,
    score: isize,
    force: Option<bool>,
    disable_edit_message: Option<bool>,
    chat_id: Option<isize>,
    message_id: Option<isize>,
    inline_message_id: Option<String>,
}

#[derive(Debug)]
struct GetGameHighScoresParams {
    user_id: isize,
    chat_id: Option<isize>,
    message_id: Option<isize>,
    inline_message_id: Option<String>,
}
