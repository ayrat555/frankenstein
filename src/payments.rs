//! API Objects to be used with [Payments](https://core.telegram.org/bots/api#payments).

use serde::{Deserialize, Serialize};

use crate::gifts::Gift;
use crate::macros::{apistruct, apply};
use crate::types::{Chat, PaidMedia, User};

#[apply(apistruct!)]
#[derive(Eq)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: u32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: u32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct OrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ShippingOption {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: u32,
    pub invoice_payload: String,
    pub subscription_expiration_date: Option<u64>,
    pub is_recurring: Option<bool>,
    pub is_first_recurring: Option<bool>,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct RefundedPayment {
    pub currency: String,
    pub total_amount: u32,
    pub invoice_payload: String,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: Option<String>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: u32,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct PaidMediaPurchased {
    pub from: User,
    pub paid_media_payload: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RevenueWithdrawalState {
    Pending(RevenueWithdrawalStatePending),
    Succeeded(RevenueWithdrawalStateSucceeded),
    Failed(RevenueWithdrawalStateFailed),
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct RevenueWithdrawalStatePending {}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct RevenueWithdrawalStateSucceeded {
    pub date: u64,
    pub url: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct RevenueWithdrawalStateFailed {}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct AffiliateInfo {
    pub affiliate_user: Option<User>,
    pub affiliate_chat: Option<Chat>,
    pub commission_per_mille: u32,
    pub amount: u32,
    pub nanostar_amount: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum TransactionPartner {
    User(Box<TransactionPartnerUser>),
    Chat(Box<TransactionPartnerChat>),
    AffiliateProgram(TransactionPartnerAffiliateProgram),
    Fragment(TransactionPartnerFragment),
    TelegramAds(TransactionPartnerTelegramAds),
    TelegramApi(TransactionPartnerTelegramApi),
    Other(TransactionPartnerOther),
}

#[apply(apistruct!)]
pub struct TransactionPartnerUser {
    pub user: User,
    pub affiliate: Option<AffiliateInfo>,
    pub invoice_payload: Option<String>,
    pub subscription_period: Option<u32>,
    pub paid_media: Option<Vec<PaidMedia>>,
    pub paid_media_payload: Option<String>,
    pub gift: Option<Gift>,
}

#[apply(apistruct!)]
pub struct TransactionPartnerChat {
    pub chat: Chat,
    pub gift: Option<Gift>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct TransactionPartnerAffiliateProgram {
    pub sponsor_user: User,
    pub commission_per_mille: u32,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct TransactionPartnerFragment {
    pub withdrawal_state: Option<RevenueWithdrawalState>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct TransactionPartnerTelegramAds {}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct TransactionPartnerTelegramApi {
    pub request_count: u64,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct TransactionPartnerOther {}

#[apply(apistruct!)]
pub struct StarTransaction {
    pub id: String,
    pub amount: u32,
    pub nanostar_amount: u32,
    pub date: u64,
    pub source: Option<TransactionPartner>,
    pub receiver: Option<TransactionPartner>,
}

#[apply(apistruct!)]
pub struct StarTransactions {
    pub transactions: Vec<StarTransaction>,
}

#[apply(apistruct!)]
pub struct StarAmount {
    pub amount: i32,
    pub nanostar_amount: Option<i32>,
}
