use serde::{Deserialize, Serialize};

use crate::macros::{apistruct, apply};
use crate::stickers::Sticker;
use crate::types::{MessageEntity, User};

#[apply(apistruct!)]
pub struct Gift {
    pub id: String,
    pub sticker: Sticker,
    pub star_count: u32,
    pub upgrade_star_count: Option<u32>,
    pub total_count: Option<u32>,
    pub remaining_count: Option<u32>,
}

#[apply(apistruct!)]
pub struct Gifts {
    pub gifts: Vec<Gift>,
}

#[apply(apistruct!)]
pub struct UniqueGiftModel {
    pub name: String,
    pub sticker: Sticker,
    pub rarity_per_mille: u32,
}

#[apply(apistruct!)]
pub struct UniqueGiftSymbol {
    pub name: String,
    pub sticker: Sticker,
    pub rarity_per_mille: u32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct UniqueGiftBackdropColors {
    pub center_color: u32,
    pub edge_color: u32,
    pub symbol_color: u32,
    pub text_color: u32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct UniqueGiftBackdrop {
    pub name: String,
    pub colors: UniqueGiftBackdropColors,
    pub rarity_per_mille: u32,
}

#[apply(apistruct!)]
pub struct UniqueGift {
    pub base_name: String,
    pub name: String,
    pub number: u32,
    pub model: UniqueGiftModel,
    pub symbol: UniqueGiftSymbol,
    pub backdrop: UniqueGiftBackdrop,
}

#[apply(apistruct!)]
pub struct GiftInfo {
    pub gift: Gift,
    pub owned_gift_id: Option<String>,
    pub convert_star_count: Option<u32>,
    pub prepaid_upgrade_star_count: Option<u32>,
    pub can_be_upgraded: Option<bool>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub is_private: Option<bool>,
}

#[apply(apistruct!)]
pub struct UniqueGiftInfo {
    pub gift: UniqueGift,
    pub origin: GiftOrigin,
    pub last_resale_star_count: Option<u32>,
    pub owned_gift_id: Option<String>,
    pub transfer_star_count: Option<u32>,
    pub next_transfer_date: Option<u64>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GiftOrigin {
    Upgrade,
    Transfer,
    Resale,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct AcceptedGiftTypes {
    pub unlimited_gifts: bool,
    pub limited_gifts: bool,
    pub unique_gifts: bool,
    pub premium_subscription: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OwnedGift {
    Regular(Box<OwnedGiftRegular>),
    Unique(Box<OwnedGiftUnique>),
}

#[apply(apistruct!)]
pub struct OwnedGiftRegular {
    pub gift: Gift,
    pub owned_gift_id: Option<String>,
    pub sender_user: Option<User>,
    pub send_date: Option<u64>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub is_private: Option<bool>,
    pub is_saved: Option<bool>,
    pub can_be_upgraded: Option<bool>,
    pub was_refunded: Option<bool>,
    pub convert_star_count: Option<u32>,
    pub prepaid_upgrade_star_count: Option<u32>,
}

#[apply(apistruct!)]
pub struct OwnedGiftUnique {
    pub gift: UniqueGift,
    pub owned_gift_id: Option<String>,
    pub sender_user: Option<User>,
    pub send_date: Option<u64>,
    pub is_saved: Option<bool>,
    pub can_be_transferred: Option<bool>,
    pub transfer_star_count: Option<u32>,
    pub next_transfer_date: Option<u64>,
}

#[apply(apistruct!)]
pub struct OwnedGifts {
    pub total_count: u32,
    pub gifts: Vec<OwnedGift>,
    pub next_offset: Option<String>,
}
