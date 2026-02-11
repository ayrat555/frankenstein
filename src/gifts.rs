use serde::{Deserialize, Serialize};

use crate::macros::{apistruct, apply};
use crate::stickers::Sticker;
use crate::types::{Chat, MessageEntity, User};

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GiftBackground {
    pub center_color: u32,
    pub edge_color: u32,
    pub text_color: u32,
}

#[apply(apistruct!)]
pub struct Gift {
    pub id: String,
    pub sticker: Sticker,
    pub star_count: u32,
    pub upgrade_star_count: Option<u32>,
    pub is_premium: Option<bool>,
    pub has_colors: Option<bool>,
    pub total_count: Option<u32>,
    pub remaining_count: Option<u32>,
    pub personal_total_count: Option<u32>,
    pub personal_remaining_count: Option<u32>,
    pub background: Option<GiftBackground>,
    pub unique_gift_variant_count: Option<u32>,
    pub publisher_chat: Option<Box<Chat>>,
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
    pub rarity: Option<String>,
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
    pub gift_id: String,
    pub base_name: String,
    pub name: String,
    pub number: u32,
    pub model: UniqueGiftModel,
    pub symbol: UniqueGiftSymbol,
    pub backdrop: UniqueGiftBackdrop,
    pub is_premium: Option<bool>,
    pub is_burned: Option<bool>,
    pub is_from_blockchain: Option<bool>,
    pub colors: Option<UniqueGiftColors>,
    pub publisher_chat: Option<Box<Chat>>,
}

#[apply(apistruct!)]
pub struct GiftInfo {
    pub gift: Gift,
    pub owned_gift_id: Option<String>,
    pub convert_star_count: Option<u32>,
    pub prepaid_upgrade_star_count: Option<u32>,
    pub is_upgrade_separate: Option<bool>,
    pub can_be_upgraded: Option<bool>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub is_private: Option<bool>,
    pub unique_gift_number: Option<u32>,
}

#[apply(apistruct!)]
pub struct UniqueGiftInfo {
    pub gift: UniqueGift,
    pub origin: GiftOrigin,
    pub last_resale_currency: Option<String>,
    pub last_resale_amount: Option<u64>,
    pub owned_gift_id: Option<String>,
    pub transfer_star_count: Option<u32>,
    pub next_transfer_date: Option<u64>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum GiftOrigin {
    Upgrade,
    Transfer,
    Resale,
    GiftedUpgrade,
    Offer,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct AcceptedGiftTypes {
    pub unlimited_gifts: bool,
    pub limited_gifts: bool,
    pub unique_gifts: bool,
    pub premium_subscription: bool,
    pub gifts_from_channels: bool,
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
    pub is_upgrade_separate: Option<bool>,
    pub unique_gift_number: Option<u32>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct UniqueGiftColors {
    pub model_custom_emoji_id: String,
    pub symbol_custom_emoji_id: String,
    pub light_theme_main_color: u32,
    pub light_theme_other_colors: Vec<u32>,
    pub dark_theme_main_color: u32,
    pub dark_theme_other_colors: Vec<u32>,
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
