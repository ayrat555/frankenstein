//! API Objects to be used with [Inline Mode](https://core.telegram.org/bots/api#inline-mode).

#![allow(deprecated)]

use serde::{Deserialize, Serialize};

use crate::macros::{apistruct, apply};
use crate::parse_mode::ParseMode;
use crate::payments::LabeledPrice;
use crate::types::{
    InlineKeyboardMarkup, LinkPreviewOptions, Location, MessageEntity, User, WebAppInfo,
};

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
#[derive(Eq)]
pub struct InlineQueryResultsButton {
    pub text: String,
    pub web_app: Option<WebAppInfo>,
    pub start_parameter: Option<String>,
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

macro_rules! iqr_from {
    (maybecached $type:ident) => {
        paste::paste! {
            impl From<[< InlineQueryResultCached $type >] > for InlineQueryResult {
                fn from(value: [< InlineQueryResultCached $type >]) -> Self {
                    Self::$type(MaybeCached::Cached(value))
                }
            }
            impl From<[< InlineQueryResult $type >] > for InlineQueryResult {
                fn from(value: [< InlineQueryResult $type >]) -> Self {
                    Self::$type(MaybeCached::NotCached(value))
                }
            }
        }
    };
    (cached $type:ident) => {
        paste::paste! {
            impl From<[< InlineQueryResultCached $type >] > for InlineQueryResult {
                fn from(value: [< InlineQueryResultCached $type >]) -> Self {
                    Self::$type(value)
                }
            }
        }
    };
    ($type:ident) => {
        paste::paste! {
            impl From<[< InlineQueryResult $type >] > for InlineQueryResult {
                fn from(value: [< InlineQueryResult $type >]) -> Self {
                    Self::$type(value)
                }
            }
        }
    };
}

iqr_from!(maybecached Audio);
iqr_from!(maybecached Document);
iqr_from!(maybecached Gif);
iqr_from!(maybecached Mpeg4Gif);
iqr_from!(maybecached Photo);
iqr_from!(cached Sticker);
iqr_from!(maybecached Video);
iqr_from!(maybecached Voice);
iqr_from!(Article);
iqr_from!(Contact);
iqr_from!(Game);
iqr_from!(Location);
iqr_from!(Venue);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum MaybeCached<T1, T2> {
    Cached(T1),
    NotCached(T2),
}

#[apply(apistruct!)]
pub struct InlineQueryResultArticle {
    pub id: String,
    pub title: String,
    pub input_message_content: InputMessageContent,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub url: Option<String>,
    #[doc(hidden)]
    #[deprecated(
        since = "0.38.0",
        note = "Please pass an empty string as `url` instead"
    )]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum InputMessageContent {
    Text(InputTextMessageContent),
    Location(InputLocationMessageContent),
    Venue(InputVenueMessageContent),
    Contact(InputContactMessageContent),
    Invoice(InputInvoiceMessageContent),
}

impl From<InputTextMessageContent> for InputMessageContent {
    fn from(value: InputTextMessageContent) -> Self {
        Self::Text(value)
    }
}
impl From<InputLocationMessageContent> for InputMessageContent {
    fn from(value: InputLocationMessageContent) -> Self {
        Self::Location(value)
    }
}
impl From<InputVenueMessageContent> for InputMessageContent {
    fn from(value: InputVenueMessageContent) -> Self {
        Self::Venue(value)
    }
}
impl From<InputContactMessageContent> for InputMessageContent {
    fn from(value: InputContactMessageContent) -> Self {
        Self::Contact(value)
    }
}
impl From<InputInvoiceMessageContent> for InputMessageContent {
    fn from(value: InputInvoiceMessageContent) -> Self {
        Self::Invoice(value)
    }
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
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<String>,
    pub query: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SentWebAppMessage {
    pub inline_message_id: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PreparedInlineMessage {
    pub id: String,
    pub expiration_date: u64,
}
