# InlineObject64

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inline_query_id** | **String** | Unique identifier for the answered query | 
**results** | [**Vec<crate::models::InlineQueryResult>**](InlineQueryResult.md) | A JSON-serialized array of results for the inline query | 
**cache_time** | Option<**i32**> | The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300. | [optional][default to 300]
**is_personal** | Option<**bool**> | Pass *True*, if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query | [optional]
**next_offset** | Option<**String**> | Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don't support pagination. Offset length can't exceed 64 bytes. | [optional]
**switch_pm_text** | Option<**String**> | If passed, clients will display a button with specified text that switches the user to a private chat with the bot and sends the bot a start message with the parameter *switch\\_pm\\_parameter* | [optional]
**switch_pm_parameter** | Option<**String**> | [Deep-linking](/bots#deep-linking) parameter for the /start message sent to the bot when user presses the switch button. 1-64 characters, only `A-Z`, `a-z`, `0-9`, `_` and `-` are allowed.    *Example:* An inline bot that sends YouTube videos can ask the user to connect the bot to their YouTube account to adapt search results accordingly. To do this, it displays a 'Connect your YouTube account' button above the results, or even before showing any. The user presses the button, switches to a private chat with the bot and, in doing so, passes a start parameter that instructs the bot to return an oauth link. Once done, the bot can offer a [*switch\\_inline*](https://core.telegram.org/bots/api/#inlinekeyboardmarkup) button so that the user can easily return to the chat where they wanted to use the bot's inline capabilities. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


