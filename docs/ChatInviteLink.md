# ChatInviteLink

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**invite_link** | **String** | The invite link. If the link was created by another chat administrator, then the second part of the link will be replaced with “…”. | 
**creator** | [**crate::models::User**](User.md) |  | 
**is_primary** | **bool** | True, if the link is primary | 
**is_revoked** | **bool** | True, if the link is revoked | 
**expire_date** | Option<**i32**> | *Optional*. Point in time (Unix timestamp) when the link will expire or has been expired | [optional]
**member_limit** | Option<**i32**> | *Optional*. Maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


