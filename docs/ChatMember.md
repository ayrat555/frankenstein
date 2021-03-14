# ChatMember

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user** | [**crate::models::User**](User.md) |  | 
**status** | **String** | The member's status in the chat. Can be “creator”, “administrator”, “member”, “restricted”, “left” or “kicked” | 
**custom_title** | Option<**String**> | *Optional*. Owner and administrators only. Custom title for this user | [optional]
**is_anonymous** | Option<**bool**> | *Optional*. Owner and administrators only. True, if the user's presence in the chat is hidden | [optional]
**can_be_edited** | Option<**bool**> | *Optional*. Administrators only. True, if the bot is allowed to edit administrator privileges of that user | [optional]
**can_manage_chat** | Option<**bool**> | *Optional*. Administrators only. True, if the administrator can access the chat event log, chat statistics, message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege | [optional]
**can_post_messages** | Option<**bool**> | *Optional*. Administrators only. True, if the administrator can post in the channel; channels only | [optional]
**can_edit_messages** | Option<**bool**> | *Optional*. Administrators only. True, if the administrator can edit messages of other users and can pin messages; channels only | [optional]
**can_delete_messages** | Option<**bool**> | *Optional*. Administrators only. True, if the administrator can delete messages of other users | [optional]
**can_manage_voice_chats** | Option<**bool**> | *Optional*. Administrators only. True, if the administrator can manage voice chats | [optional]
**can_restrict_members** | Option<**bool**> | *Optional*. Administrators only. True, if the administrator can restrict, ban or unban chat members | [optional]
**can_promote_members** | Option<**bool**> | *Optional*. Administrators only. True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by the user) | [optional]
**can_change_info** | Option<**bool**> | *Optional*. Administrators and restricted only. True, if the user is allowed to change the chat title, photo and other settings | [optional]
**can_invite_users** | Option<**bool**> | *Optional*. Administrators and restricted only. True, if the user is allowed to invite new users to the chat | [optional]
**can_pin_messages** | Option<**bool**> | *Optional*. Administrators and restricted only. True, if the user is allowed to pin messages; groups and supergroups only | [optional]
**is_member** | Option<**bool**> | *Optional*. Restricted only. True, if the user is a member of the chat at the moment of the request | [optional]
**can_send_messages** | Option<**bool**> | *Optional*. Restricted only. True, if the user is allowed to send text messages, contacts, locations and venues | [optional]
**can_send_media_messages** | Option<**bool**> | *Optional*. Restricted only. True, if the user is allowed to send audios, documents, photos, videos, video notes and voice notes | [optional]
**can_send_polls** | Option<**bool**> | *Optional*. Restricted only. True, if the user is allowed to send polls | [optional]
**can_send_other_messages** | Option<**bool**> | *Optional*. Restricted only. True, if the user is allowed to send animations, games, stickers and use inline bots | [optional]
**can_add_web_page_previews** | Option<**bool**> | *Optional*. Restricted only. True, if the user is allowed to add web page previews to their messages | [optional]
**until_date** | Option<**i32**> | *Optional*. Restricted and kicked only. Date when restrictions will be lifted for this user; unix time | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


