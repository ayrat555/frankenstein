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
    pub photo: String,
}

#[apply(apistruct!)]
pub struct InputProfilePhotoAnimated {
    pub animation: String,
    pub main_frame_timestamp: Option<f64>,
}
