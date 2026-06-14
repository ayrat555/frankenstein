//! API Objects related to [InputMedia](https://core.telegram.org/bots/api#inputmedia)

use serde::{Deserialize, Serialize};

use crate::input_file::FileUpload;
use crate::macros::{apistruct, apply};
use crate::parse_mode::ParseMode;
use crate::types::MessageEntity;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputMedia {
    Animation(InputMediaAnimation),
    Document(InputMediaDocument),
    Audio(InputMediaAudio),
    LivePhoto(InputMediaLivePhoto),
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
}

impl From<InputMediaAnimation> for InputMedia {
    fn from(value: InputMediaAnimation) -> Self {
        Self::Animation(value)
    }
}
impl From<InputMediaDocument> for InputMedia {
    fn from(value: InputMediaDocument) -> Self {
        Self::Document(value)
    }
}
impl From<InputMediaAudio> for InputMedia {
    fn from(value: InputMediaAudio) -> Self {
        Self::Audio(value)
    }
}
impl From<InputMediaLivePhoto> for InputMedia {
    fn from(value: InputMediaLivePhoto) -> Self {
        Self::LivePhoto(value)
    }
}
impl From<InputMediaPhoto> for InputMedia {
    fn from(value: InputMediaPhoto) -> Self {
        Self::Photo(value)
    }
}
impl From<InputMediaVideo> for InputMedia {
    fn from(value: InputMediaVideo) -> Self {
        Self::Video(value)
    }
}

/// Media relevant for `sendMediaGroup`.
///
/// See <https://core.telegram.org/bots/api#sendmediagroup>
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MediaGroupInputMedia {
    Audio(InputMediaAudio),
    Document(InputMediaDocument),
    LivePhoto(InputMediaLivePhoto),
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
}

impl From<InputMediaAudio> for MediaGroupInputMedia {
    fn from(value: InputMediaAudio) -> Self {
        Self::Audio(value)
    }
}
impl From<InputMediaDocument> for MediaGroupInputMedia {
    fn from(value: InputMediaDocument) -> Self {
        Self::Document(value)
    }
}
impl From<InputMediaLivePhoto> for MediaGroupInputMedia {
    fn from(value: InputMediaLivePhoto) -> Self {
        Self::LivePhoto(value)
    }
}
impl From<InputMediaPhoto> for MediaGroupInputMedia {
    fn from(value: InputMediaPhoto) -> Self {
        Self::Photo(value)
    }
}
impl From<InputMediaVideo> for MediaGroupInputMedia {
    fn from(value: InputMediaVideo) -> Self {
        Self::Video(value)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputPollMedia {
    Animation(InputMediaAnimation),
    Audio(InputMediaAudio),
    Document(InputMediaDocument),
    LivePhoto(InputMediaLivePhoto),
    Location(InputMediaLocation),
    Photo(InputMediaPhoto),
    Venue(InputMediaVenue),
    Video(InputMediaVideo),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputPollOptionMedia {
    Animation(InputMediaAnimation),
    Link(InputMediaLink),
    LivePhoto(InputMediaLivePhoto),
    Location(InputMediaLocation),
    Photo(InputMediaPhoto),
    Sticker(InputMediaSticker),
    Venue(InputMediaVenue),
    Video(InputMediaVideo),
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
    pub cover: Option<FileUpload>,
    pub start_timestamp: Option<u64>,
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
pub struct InputMediaLink {
    pub url: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputMediaLivePhoto {
    pub media: FileUpload,
    pub photo: FileUpload,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub has_spoiler: Option<bool>,
}

#[apply(apistruct!)]
pub struct InputMediaLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub horizontal_accuracy: Option<f64>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputMediaSticker {
    pub media: FileUpload,
    pub emoji: Option<String>,
}

#[apply(apistruct!)]
pub struct InputMediaVenue {
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputPaidMedia {
    LivePhoto(InputPaidMediaLivePhoto),
    Photo(InputPaidMediaPhoto),
    Video(InputPaidMediaVideo),
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputPaidMediaLivePhoto {
    pub media: String,
    pub photo: String,
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
    pub cover: Option<String>,
    pub start_timestamp: Option<u64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<u32>,
    pub supports_streaming: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputProfilePhoto {
    Static(InputProfilePhotoStatic),
    Animated(InputProfilePhotoAnimated),
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputProfilePhotoStatic {
    pub photo: FileUpload,
}

#[apply(apistruct!)]
pub struct InputProfilePhotoAnimated {
    pub animation: FileUpload,
    pub main_frame_timestamp: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputStoryContent {
    Photo(InputStoryContentPhoto),
    Video(InputStoryContentVideo),
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputStoryContentPhoto {
    pub photo: FileUpload,
}

#[apply(apistruct!)]
pub struct InputStoryContentVideo {
    pub video: FileUpload,
    pub duration: Option<f64>,
    pub cover_frame_timestamp: Option<f64>,
    pub is_animation: Option<bool>,
}
