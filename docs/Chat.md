# Chat

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier. | 
**_type** | **String** | Type of chat, can be either “private”, “group”, “supergroup” or “channel” | 
**title** | Option<**String**> | *Optional*. Title, for supergroups, channels and group chats | [optional]
**username** | Option<**String**> | *Optional*. Username, for private chats, supergroups and channels if available | [optional]
**first_name** | Option<**String**> | *Optional*. First name of the other party in a private chat | [optional]
**last_name** | Option<**String**> | *Optional*. Last name of the other party in a private chat | [optional]
**photo** | Option<[**crate::models::ChatPhoto**](ChatPhoto.md)> |  | [optional]
**bio** | Option<**String**> | *Optional*. Bio of the other party in a private chat. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat). | [optional]
**description** | Option<**String**> | *Optional*. Description, for groups, supergroups and channel chats. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat). | [optional]
**invite_link** | Option<**String**> | *Optional*. Primary invite link, for groups, supergroups and channel chats. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat). | [optional]
**pinned_message** | Option<[**crate::models::Message**](Message.md)> |  | [optional]
**permissions** | Option<[**crate::models::ChatPermissions**](ChatPermissions.md)> |  | [optional]
**slow_mode_delay** | Option<**i32**> | *Optional*. For supergroups, the minimum allowed delay between consecutive messages sent by each unpriviledged user. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat). | [optional]
**message_auto_delete_time** | Option<**i32**> | *Optional*. The time after which all messages sent to the chat will be automatically deleted; in seconds. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat). | [optional]
**sticker_set_name** | Option<**String**> | *Optional*. For supergroups, name of group sticker set. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat). | [optional]
**can_set_sticker_set** | Option<**bool**> | *Optional*. True, if the bot can change the group sticker set. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat). | [optional]
**linked_chat_id** | Option<**i32**> | *Optional*. Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat). | [optional]
**location** | Option<[**crate::models::ChatLocation**](ChatLocation.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


