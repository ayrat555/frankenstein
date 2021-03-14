# ChosenInlineResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**result_id** | **String** | The unique identifier for the result that was chosen | 
**from** | [**crate::models::User**](User.md) |  | 
**location** | Option<[**crate::models::Location**](Location.md)> |  | [optional]
**inline_message_id** | Option<**String**> | *Optional*. Identifier of the sent inline message. Available only if there is an [inline keyboard](https://core.telegram.org/bots/api/#inlinekeyboardmarkup) attached to the message. Will be also received in [callback queries](https://core.telegram.org/bots/api/#callbackquery) and can be used to [edit](https://core.telegram.org/bots/api/#updating-messages) the message. | [optional]
**query** | **String** | The query that was used to obtain the result | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


