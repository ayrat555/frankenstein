# InlineObject70

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **i32** | User identifier | 
**score** | **i32** | New score, must be non-negative | 
**force** | Option<**bool**> | Pass True, if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters | [optional]
**disable_edit_message** | Option<**bool**> | Pass True, if the game message should not be automatically edited to include the current scoreboard | [optional]
**chat_id** | Option<**i32**> | Required if *inline\\_message\\_id* is not specified. Unique identifier for the target chat | [optional]
**message_id** | Option<**i32**> | Required if *inline\\_message\\_id* is not specified. Identifier of the sent message | [optional]
**inline_message_id** | Option<**String**> | Required if *chat\\_id* and *message\\_id* are not specified. Identifier of the inline message | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


