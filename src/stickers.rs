//! API Objects to be used with [Stickers](https://core.telegram.org/bots/api#stickers).

use serde::{Deserialize, Serialize};

use crate::input_file::FileUpload;
use crate::macros::{apistruct, apply};
use crate::types::{File, PhotoSize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StickerType {
    Regular,
    Mask,
    CustomEmoji,
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
pub struct StickerSet {
    pub name: String,
    pub title: String,
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StickerFormat {
    Static,
    Animated,
    Video,
}

#[apply(apistruct!)]
pub struct InputSticker {
    pub sticker: FileUpload,
    pub format: StickerFormat,
    pub emoji_list: Vec<String>,
    pub mask_position: Option<MaskPosition>,
    pub keywords: Option<Vec<String>>,
}
