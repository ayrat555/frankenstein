# Message

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message_id** | **i32** | Unique message identifier inside this chat | 
**from** | Option<[**crate::models::User**](User.md)> |  | [optional]
**sender_chat** | Option<[**crate::models::Chat**](Chat.md)> |  | [optional]
**date** | **i32** | Date the message was sent in Unix time | 
**chat** | [**crate::models::Chat**](Chat.md) |  | 
**forward_from** | Option<[**crate::models::User**](User.md)> |  | [optional]
**forward_from_chat** | Option<[**crate::models::Chat**](Chat.md)> |  | [optional]
**forward_from_message_id** | Option<**i32**> | *Optional*. For messages forwarded from channels, identifier of the original message in the channel | [optional]
**forward_signature** | Option<**String**> | *Optional*. For messages forwarded from channels, signature of the post author if present | [optional]
**forward_sender_name** | Option<**String**> | *Optional*. Sender's name for messages forwarded from users who disallow adding a link to their account in forwarded messages | [optional]
**forward_date** | Option<**i32**> | *Optional*. For forwarded messages, date the original message was sent in Unix time | [optional]
**reply_to_message** | Option<[**crate::models::Message**](Message.md)> |  | [optional]
**via_bot** | Option<[**crate::models::User**](User.md)> |  | [optional]
**edit_date** | Option<**i32**> | *Optional*. Date the message was last edited in Unix time | [optional]
**media_group_id** | Option<**String**> | *Optional*. The unique identifier of a media message group this message belongs to | [optional]
**author_signature** | Option<**String**> | *Optional*. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator | [optional]
**text** | Option<**String**> | *Optional*. For text messages, the actual UTF-8 text of the message, 0-4096 characters | [optional]
**entities** | Option<[**Vec<crate::models::MessageEntity>**](MessageEntity.md)> | *Optional*. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text | [optional]
**animation** | Option<[**crate::models::Animation**](Animation.md)> |  | [optional]
**audio** | Option<[**crate::models::Audio**](Audio.md)> |  | [optional]
**document** | Option<[**crate::models::Document**](Document.md)> |  | [optional]
**photo** | Option<[**Vec<crate::models::PhotoSize>**](PhotoSize.md)> | *Optional*. Message is a photo, available sizes of the photo | [optional]
**sticker** | Option<[**crate::models::Sticker**](Sticker.md)> |  | [optional]
**video** | Option<[**crate::models::Video**](Video.md)> |  | [optional]
**video_note** | Option<[**crate::models::VideoNote**](VideoNote.md)> |  | [optional]
**voice** | Option<[**crate::models::Voice**](Voice.md)> |  | [optional]
**caption** | Option<**String**> | *Optional*. Caption for the animation, audio, document, photo, video or voice, 0-1024 characters | [optional]
**caption_entities** | Option<[**Vec<crate::models::MessageEntity>**](MessageEntity.md)> | *Optional*. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption | [optional]
**contact** | Option<[**crate::models::Contact**](Contact.md)> |  | [optional]
**dice** | Option<[**crate::models::Dice**](Dice.md)> |  | [optional]
**game** | Option<[**crate::models::Game**](Game.md)> |  | [optional]
**poll** | Option<[**crate::models::Poll**](Poll.md)> |  | [optional]
**venue** | Option<[**crate::models::Venue**](Venue.md)> |  | [optional]
**location** | Option<[**crate::models::Location**](Location.md)> |  | [optional]
**new_chat_members** | Option<[**Vec<crate::models::User>**](User.md)> | *Optional*. New members that were added to the group or supergroup and information about them (the bot itself may be one of these members) | [optional]
**left_chat_member** | Option<[**crate::models::User**](User.md)> |  | [optional]
**new_chat_title** | Option<**String**> | *Optional*. A chat title was changed to this value | [optional]
**new_chat_photo** | Option<[**Vec<crate::models::PhotoSize>**](PhotoSize.md)> | *Optional*. A chat photo was change to this value | [optional]
**delete_chat_photo** | Option<**bool**> | *Optional*. Service message: the chat photo was deleted | [optional]
**group_chat_created** | Option<**bool**> | *Optional*. Service message: the group has been created | [optional]
**supergroup_chat_created** | Option<**bool**> | *Optional*. Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in reply\\_to\\_message if someone replies to a very first message in a directly created supergroup. | [optional]
**channel_chat_created** | Option<**bool**> | *Optional*. Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in reply\\_to\\_message if someone replies to a very first message in a channel. | [optional]
**message_auto_delete_timer_changed** | Option<[**crate::models::MessageAutoDeleteTimerChanged**](MessageAutoDeleteTimerChanged.md)> |  | [optional]
**migrate_to_chat_id** | Option<**i32**> | *Optional*. The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier. | [optional]
**migrate_from_chat_id** | Option<**i32**> | *Optional*. The supergroup has been migrated from a group with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier. | [optional]
**pinned_message** | Option<[**crate::models::Message**](Message.md)> |  | [optional]
**invoice** | Option<[**crate::models::Invoice**](Invoice.md)> |  | [optional]
**successful_payment** | Option<[**crate::models::SuccessfulPayment**](SuccessfulPayment.md)> |  | [optional]
**connected_website** | Option<**String**> | *Optional*. The domain name of the website on which the user has logged in. [More about Telegram Login »](/widgets/login) | [optional]
**passport_data** | Option<[**crate::models::PassportData**](PassportData.md)> |  | [optional]
**proximity_alert_triggered** | Option<[**crate::models::ProximityAlertTriggered**](ProximityAlertTriggered.md)> |  | [optional]
**voice_chat_started** | Option<[**serde_json::Value**](.md)> | This object represents a service message about a voice chat started in the chat. Currently holds no information. | [optional]
**voice_chat_ended** | Option<[**crate::models::VoiceChatEnded**](VoiceChatEnded.md)> |  | [optional]
**voice_chat_participants_invited** | Option<[**crate::models::VoiceChatParticipantsInvited**](VoiceChatParticipantsInvited.md)> |  | [optional]
**reply_markup** | Option<[**crate::models::InlineKeyboardMarkup**](InlineKeyboardMarkup.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


