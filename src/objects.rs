#[derive(Debug)]
pub enum ThumbEnum {
    InputFileVariant(InputFile),
    StringVariant(String),
}

#[derive(Debug)]
pub enum InputMessageContent {
    InputTextMessageContentVariant(InputTextMessageContent),
    InputLocationMessageContentVariant(InputLocationMessageContent),
    InputVenueMessageContentVariant(InputLocationMessageContent),
    InputContactMessageContentVariant(InputLocationMessageContent),
}

#[derive(Debug)]
pub struct InputFile {}

#[derive(Debug)]
pub struct VoiceChatStarted {}

#[derive(Debug)]
pub struct CallbackGame {}

#[derive(Debug)]
pub struct Update {
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
pub struct WebhookInfo {
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
pub struct User {
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
pub struct Chat {
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
pub struct Message {
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
pub struct MessageId {
    message_id: isize,
}

#[derive(Debug)]
pub struct MessageEntity {
    type_field: String,
    offset: isize,
    length: isize,
    url: Option<String>,
    user: Option<User>,
    language: Option<String>,
}

#[derive(Debug)]
pub struct PhotoSize {
    file_id: String,
    file_unique_id: String,
    width: isize,
    height: isize,
    file_size: Option<isize>,
}

#[derive(Debug)]
pub struct Animation {
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
pub struct Audio {
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
pub struct Document {
    file_id: String,
    file_unique_id: String,
    thumb: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<isize>,
}

#[derive(Debug)]
pub struct Video {
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
pub struct VideoNote {
    file_id: String,
    file_unique_id: String,
    length: isize,
    duration: isize,
    thumb: Option<PhotoSize>,
    file_size: Option<isize>,
}

#[derive(Debug)]
pub struct Voice {
    file_id: String,
    file_unique_id: String,
    duration: isize,
    mime_type: Option<String>,
    file_size: Option<isize>,
}

#[derive(Debug)]
pub struct Contact {
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    user_id: Option<isize>,
    vcard: Option<String>,
}

#[derive(Debug)]
pub struct Dice {
    emoji: String,
    value: isize,
}

#[derive(Debug)]
pub struct PollOption {
    text: String,
    voter_count: isize,
}

#[derive(Debug)]
pub struct PollAnswer {
    poll_id: String,
    user: User,
    option_ids: Vec<isize>,
}

#[derive(Debug)]
pub struct Poll {
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
pub struct Location {
    longitude: f64,
    latitude: f64,
    horizontal_accuracy: Option<f64>,
    live_period: Option<isize>,
    heading: Option<isize>,
    proximity_alert_radius: Option<isize>,
}

#[derive(Debug)]
pub struct Venue {
    location: Location,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
}

#[derive(Debug)]
pub struct ProximityAlertTriggered {
    traveler: User,
    watcher: User,
    distance: isize,
}

#[derive(Debug)]
pub struct MessageAutoDeleteTimerChanged {
    message_auto_delete_time: isize,
}

#[derive(Debug)]
pub struct VoiceChatEnded {
    duration: isize,
}

#[derive(Debug)]
pub struct VoiceChatParticipantsInvited {
    users: Option<Vec<User>>,
}

#[derive(Debug)]
pub struct UserProfilePhotos {
    total_count: isize,
    photos: Vec<PhotoSize>,
}

#[derive(Debug)]
pub struct File {
    file_id: String,
    file_unique_id: String,
    file_size: Option<isize>,
    file_path: Option<String>,
}

#[derive(Debug)]
pub struct ReplyKeyboardMarkup {
    keyboard: Vec<KeyboardButton>,
    resize_keyboard: Option<bool>,
    one_time_keyboard: Option<bool>,
    selective: Option<bool>,
}

#[derive(Debug)]
pub struct KeyboardButton {
    text: String,
    request_contact: Option<bool>,
    request_location: Option<bool>,
    request_poll: Option<KeyboardButtonPollType>,
}

#[derive(Debug)]
pub struct KeyboardButtonPollType {
    type_field: Option<String>,
}

#[derive(Debug)]
pub struct ReplyKeyboardRemove {
    remove_keyboard: bool,
    selective: Option<bool>,
}

#[derive(Debug)]
pub struct InlineKeyboardMarkup {
    inline_keyboard: Vec<InlineKeyboardButton>,
}

#[derive(Debug)]
pub struct InlineKeyboardButton {
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
pub struct LoginUrl {
    url: String,
    forward_text: Option<String>,
    bot_username: Option<String>,
    request_write_access: Option<bool>,
}

#[derive(Debug)]
pub struct CallbackQuery {
    id: String,
    from: User,
    message: Option<Message>,
    inline_message_id: Option<String>,
    chat_instance: String,
    data: Option<String>,
    game_short_name: Option<String>,
}

#[derive(Debug)]
pub struct ForceReply {
    force_reply: bool,
    selective: Option<bool>,
}

#[derive(Debug)]
pub struct ChatPhoto {
    small_file_id: String,
    small_file_unique_id: String,
    big_file_id: String,
    big_file_unique_id: String,
}

#[derive(Debug)]
pub struct ChatInviteLink {
    invite_link: String,
    creator: User,
    is_primary: bool,
    is_revoked: bool,
    expire_date: Option<isize>,
    member_limit: Option<isize>,
}

#[derive(Debug)]
pub struct ChatMember {
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
pub struct ChatMemberUpdated {
    chat: Chat,
    from: User,
    date: isize,
    old_chat_member: ChatMember,
    new_chat_member: ChatMember,
    invite_link: Option<ChatInviteLink>,
}

#[derive(Debug)]
pub struct ChatPermissions {
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
pub struct ChatLocation {
    location: Location,
    address: String,
}

#[derive(Debug)]
pub struct BotCommand {
    command: String,
    description: String,
}

#[derive(Debug)]
pub struct ResponseParameters {
    migrate_to_chat_id: Option<isize>,
    retry_after: Option<isize>,
}

#[derive(Debug)]
pub struct InputMediaPhoto {
    type_field: String,
    media: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
}

#[derive(Debug)]
pub struct InputMediaVideo {
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
pub struct InputMediaAnimation {
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
pub struct InputMediaAudio {
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
pub struct InputMediaDocument {
    type_field: String,
    media: String,
    thumb: Option<ThumbEnum>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    disable_content_type_detection: Option<bool>,
}

#[derive(Debug)]
pub struct Sticker {
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
pub struct StickerSet {
    name: String,
    title: String,
    is_animated: bool,
    contains_masks: bool,
    stickers: Vec<Sticker>,
    thumb: Option<PhotoSize>,
}

#[derive(Debug)]
pub struct MaskPosition {
    point: String,
    x_shift: f64,
    y_shift: f64,
    scale: f64,
}

#[derive(Debug)]
pub struct InlineQuery {
    id: String,
    from: User,
    location: Option<Location>,
    query: String,
    offset: String,
}

#[derive(Debug)]
pub struct InlineQueryResultArticle {
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
pub struct InlineQueryResultPhoto {
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
pub struct InlineQueryResultGif {
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
pub struct InlineQueryResultMpeg4Gif {
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
pub struct InlineQueryResultVideo {
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
pub struct InlineQueryResultAudio {
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
pub struct InlineQueryResultVoice {
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
pub struct InlineQueryResultDocument {
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
pub struct InlineQueryResultLocation {
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
pub struct InlineQueryResultVenue {
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
pub struct InlineQueryResultContact {
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
pub struct InlineQueryResultGame {
    type_field: String,
    id: String,
    game_short_name: String,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
pub struct InlineQueryResultCachedPhoto {
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
pub struct InlineQueryResultCachedGif {
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
pub struct InlineQueryResultCachedMpeg4Gif {
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
pub struct InlineQueryResultCachedSticker {
    type_field: String,
    id: String,
    sticker_file_id: String,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
pub struct InlineQueryResultCachedDocument {
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
pub struct InlineQueryResultCachedVideo {
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
pub struct InlineQueryResultCachedVoice {
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
pub struct InlineQueryResultCachedAudio {
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
pub struct InputTextMessageContent {
    message_text: String,
    parse_mode: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    disable_web_page_preview: Option<bool>,
}

#[derive(Debug)]
pub struct InputLocationMessageContent {
    latitude: f64,
    longitude: f64,
    horizontal_accuracy: Option<f64>,
    live_period: Option<isize>,
    heading: Option<isize>,
    proximity_alert_radius: Option<isize>,
}

#[derive(Debug)]
pub struct InputVenueMessageContent {
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
pub struct InputContactMessageContent {
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    vcard: Option<String>,
}

#[derive(Debug)]
pub struct ChosenInlineResult {
    result_id: String,
    from: User,
    location: Option<Location>,
    inline_message_id: Option<String>,
    query: String,
}

#[derive(Debug)]
pub struct LabeledPrice {
    label: String,
    amount: isize,
}

#[derive(Debug)]
pub struct Invoice {
    title: String,
    description: String,
    start_parameter: String,
    currency: String,
    total_amount: isize,
}

#[derive(Debug)]
pub struct ShippingAddress {
    country_code: String,
    state: String,
    city: String,
    street_line1: String,
    street_line2: String,
    post_code: String,
}

#[derive(Debug)]
pub struct OrderInfo {
    name: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    shipping_address: Option<ShippingAddress>,
}

#[derive(Debug)]
pub struct ShippingOption {
    id: String,
    title: String,
    prices: Vec<LabeledPrice>,
}

#[derive(Debug)]
pub struct SuccessfulPayment {
    currency: String,
    total_amount: isize,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
    telegram_payment_charge_id: String,
    provider_payment_charge_id: String,
}

#[derive(Debug)]
pub struct ShippingQuery {
    id: String,
    from: User,
    invoice_payload: String,
    shipping_address: ShippingAddress,
}

#[derive(Debug)]
pub struct PreCheckoutQuery {
    id: String,
    from: User,
    currency: String,
    total_amount: isize,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
}

#[derive(Debug)]
pub struct PassportData {
    data: Vec<EncryptedPassportElement>,
    credentials: EncryptedCredentials,
}

#[derive(Debug)]
pub struct PassportFile {
    file_id: String,
    file_unique_id: String,
    file_size: isize,
    file_date: isize,
}

#[derive(Debug)]
pub struct EncryptedPassportElement {
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
pub struct EncryptedCredentials {
    data: String,
    hash: String,
    secret: String,
}

#[derive(Debug)]
pub struct PassportElementErrorDataField {
    source: String,
    type_field: String,
    field_name: String,
    data_hash: String,
    message: String,
}

#[derive(Debug)]
pub struct PassportElementErrorFrontSide {
    source: String,
    type_field: String,
    file_hash: String,
    message: String,
}

#[derive(Debug)]
pub struct PassportElementErrorReverseSide {
    source: String,
    type_field: String,
    file_hash: String,
    message: String,
}

#[derive(Debug)]
pub struct PassportElementErrorSelfie {
    source: String,
    type_field: String,
    file_hash: String,
    message: String,
}

#[derive(Debug)]
pub struct PassportElementErrorFile {
    source: String,
    type_field: String,
    file_hash: String,
    message: String,
}

#[derive(Debug)]
pub struct PassportElementErrorFiles {
    source: String,
    type_field: String,
    file_hashes: Vec<String>,
    message: String,
}

#[derive(Debug)]
pub struct PassportElementErrorTranslationFile {
    source: String,
    type_field: String,
    file_hash: String,
    message: String,
}

#[derive(Debug)]
pub struct PassportElementErrorTranslationFiles {
    source: String,
    type_field: String,
    file_hashes: Vec<String>,
    message: String,
}

#[derive(Debug)]
pub struct PassportElementErrorUnspecified {
    source: String,
    type_field: String,
    element_hash: String,
    message: String,
}

#[derive(Debug)]
pub struct Game {
    title: String,
    description: String,
    photo: Vec<PhotoSize>,
    text: Option<String>,
    text_entities: Option<Vec<MessageEntity>>,
    animation: Option<Animation>,
}

#[derive(Debug)]
pub struct GameHighScore {
    position: isize,
    user: User,
    score: isize,
}
