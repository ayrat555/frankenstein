# CallbackQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for this query | 
**from** | [**crate::models::User**](User.md) |  | 
**message** | Option<[**crate::models::Message**](Message.md)> |  | [optional]
**inline_message_id** | Option<**String**> | *Optional*. Identifier of the message sent via the bot in inline mode, that originated the query. | [optional]
**chat_instance** | **String** | Global identifier, uniquely corresponding to the chat to which the message with the callback button was sent. Useful for high scores in [games](https://core.telegram.org/bots/api/#games). | 
**data** | Option<**String**> | *Optional*. Data associated with the callback button. Be aware that a bad client can send arbitrary data in this field. | [optional]
**game_short_name** | Option<**String**> | *Optional*. Short name of a [Game](https://core.telegram.org/bots/api/#games) to be returned, serves as the unique identifier for the game | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


