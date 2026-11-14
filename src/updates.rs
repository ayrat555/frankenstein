//! API Objects used to [get updates](https://core.telegram.org/bots/api#getting-updates).

use serde::{Deserialize, Serialize};

use crate::inline_mode::{ChosenInlineResult, InlineQuery};
use crate::macros::{apistruct, apply};
use crate::payments::{PaidMediaPurchased, PreCheckoutQuery, ShippingQuery};
use crate::types::{
    AllowedUpdate, BotSubscriptionUpdated, BusinessConnection, BusinessMessagesDeleted,
    CallbackQuery, ChatBoostRemoved, ChatBoostUpdated, ChatJoinRequest, ChatMemberUpdated,
    ManagedBotUpdated, Message, MessageGenerationStopped, MessageReactionCountUpdated,
    MessageReactionUpdated, Poll, PollAnswer, SharedWidgetQuery, WidgetQuery,
};

/// Represents an incoming update from telegram.
/// [Official documentation.](https://core.telegram.org/bots/api#update)
#[apply(apistruct!)]
pub struct Update {
    pub update_id: u32,

    /// Maps to exactly one of the many optional fields
    /// from [the official documentation](https://core.telegram.org/bots/api#update).
    #[serde(flatten)]
    pub content: UpdateContent,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateContent {
    Message(Box<Message>),
    EditedMessage(Box<Message>),
    ChannelPost(Box<Message>),
    EditedChannelPost(Box<Message>),
    BusinessConnection(BusinessConnection),
    BusinessMessage(Box<Message>),
    EditedBusinessMessage(Box<Message>),
    DeletedBusinessMessages(BusinessMessagesDeleted),
    GuestMessage(Box<Message>),
    MessageReaction(MessageReactionUpdated),
    MessageReactionCount(MessageReactionCountUpdated),
    InlineQuery(InlineQuery),
    ChosenInlineResult(ChosenInlineResult),
    CallbackQuery(Box<CallbackQuery>),
    WidgetQuery(Box<WidgetQuery>),
    SharedWidgetQuery(Box<SharedWidgetQuery>),
    ShippingQuery(ShippingQuery),
    PreCheckoutQuery(PreCheckoutQuery),
    Poll(Box<Poll>),
    PollAnswer(PollAnswer),
    MyChatMember(ChatMemberUpdated),
    ChatMember(ChatMemberUpdated),
    ChatJoinRequest(ChatJoinRequest),
    ChatBoost(ChatBoostUpdated),
    RemovedChatBoost(ChatBoostRemoved),
    PurchasedPaidMedia(PaidMediaPurchased),
    ManagedBot(ManagedBotUpdated),
    Subscription(BotSubscriptionUpdated),
    StoppedMessageGeneration(MessageGenerationStopped),
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct WebhookInfo {
    pub url: String,
    pub has_custom_certificate: bool,
    pub pending_update_count: u32,
    pub ip_address: Option<String>,
    pub last_error_date: Option<u64>,
    pub last_error_message: Option<String>,
    pub last_synchronization_error_date: Option<u64>,
    pub max_connections: Option<u16>,
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
}

#[cfg(test)]
mod serde_tests {
    use super::*;
    use crate::types::{Chat, ChatType, User};

    #[test]
    pub fn widget_query_update_is_deserialized() {
        let update_content = r#"{
            "update_id": 2342,
            "widget_query": {
                "id": "q1",
                "from": {"id": 123456789, "is_bot": false, "first_name": "Name"},
                "chat": {"id": 123456789, "type": "private", "first_name": "Name"},
                "message_id": 10,
                "chat_instance": "ci",
                "type": {
                    "type": "sign_data",
                    "wallet": {"address": "EQabc", "amount": "1000000000", "public_key": "ab", "state_init": "te6c"},
                    "sign_data": "{}"
                },
                "data": "vote:1"
            }
        }"#;

        let update: Update = serde_json::from_str(update_content).unwrap();
        let UpdateContent::WidgetQuery(query) = update.content else {
            panic!("expected a widget query, got {:?}", update.content);
        };
        assert_eq!(query.data, "vote:1");
        let crate::types::WidgetQueryType::SignData(sign_data) = query.type_field else {
            panic!("expected sign_data");
        };
        assert_eq!(sign_data.wallet.address, "EQabc");
    }

    #[test]
    pub fn shared_widget_query_update_is_deserialized() {
        let update_content = r#"{
            "update_id": 2343,
            "shared_widget_query": {
                "id": "q2",
                "from": {"id": 123456789, "is_bot": false, "first_name": "Name"},
                "type": {"type": "data"},
                "data": "refresh",
                "widget_id": "w1"
            }
        }"#;

        let update: Update = serde_json::from_str(update_content).unwrap();
        let UpdateContent::SharedWidgetQuery(query) = update.content else {
            panic!("expected a shared widget query, got {:?}", update.content);
        };
        assert_eq!(query.widget_id, "w1");
        assert_eq!(
            query.type_field,
            crate::types::WidgetQueryType::Data(
                crate::types::WidgetQueryTypeData::builder().build()
            )
        );
    }

    #[test]
    pub fn update_content_is_flattened() {
        let update_content = r#"{
            "update_id": 2341,
            "message": {
                "message_id": 2746,
                "from": {
                    "id": 1276618370,
                    "is_bot": true,
                    "first_name": "test_el_bot",
                    "username": "el_mon_test_bot"
                },
                "date": 1618207352,
                "chat": {
                    "id": 123456789,
                    "type": "private",
                    "username": "Name555",
                    "first_name": "Name",
                    "last_name": "Surname"
                },
                "text": "Hello!"
            }
        }"#;

        let update: Update = serde_json::from_str(update_content).unwrap();

        let message = Message::builder()
            .message_id(2746)
            .from(
                User::builder()
                    .id(1276618370)
                    .is_bot(true)
                    .first_name("test_el_bot")
                    .username("el_mon_test_bot")
                    .build(),
            )
            .date(1618207352)
            .chat(
                Chat::builder()
                    .id(123456789)
                    .type_field(ChatType::Private)
                    .username("Name555")
                    .first_name("Name")
                    .last_name("Surname")
                    .build(),
            )
            .text("Hello!")
            .build();

        let expected = Update {
            update_id: 2341,
            content: UpdateContent::Message(Box::new(message)),
        };

        assert_eq!(update, expected);
    }
}
