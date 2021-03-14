# InlineObject27

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chat_id** | [**crate::models::AnyOfintegerstring**](anyOf<integer,string>.md) | Unique identifier for the target chat or username of the target channel (in the format `@channelusername`) | 
**user_id** | **i32** | Unique identifier of the target user | 
**is_anonymous** | Option<**bool**> | Pass *True*, if the administrator's presence in the chat is hidden | [optional]
**can_manage_chat** | Option<**bool**> | Pass True, if the administrator can access the chat event log, chat statistics, message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege | [optional]
**can_post_messages** | Option<**bool**> | Pass True, if the administrator can create channel posts, channels only | [optional]
**can_edit_messages** | Option<**bool**> | Pass True, if the administrator can edit messages of other users and can pin messages, channels only | [optional]
**can_delete_messages** | Option<**bool**> | Pass True, if the administrator can delete messages of other users | [optional]
**can_manage_voice_chats** | Option<**bool**> | Pass True, if the administrator can manage voice chats, supergroups only | [optional]
**can_restrict_members** | Option<**bool**> | Pass True, if the administrator can restrict, ban or unban chat members | [optional]
**can_promote_members** | Option<**bool**> | Pass True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by him) | [optional]
**can_change_info** | Option<**bool**> | Pass True, if the administrator can change chat title, photo and other settings | [optional]
**can_invite_users** | Option<**bool**> | Pass True, if the administrator can invite new users to the chat | [optional]
**can_pin_messages** | Option<**bool**> | Pass True, if the administrator can pin messages, supergroups only | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


