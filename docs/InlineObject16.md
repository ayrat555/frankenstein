# InlineObject16

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chat_id** | Option<[**crate::models::AnyOfintegerstring**](anyOf<integer,string>.md)> | Required if *inline\\_message\\_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`) | [optional]
**message_id** | Option<**i32**> | Required if *inline\\_message\\_id* is not specified. Identifier of the message with live location to stop | [optional]
**inline_message_id** | Option<**String**> | Required if *chat\\_id* and *message\\_id* are not specified. Identifier of the inline message | [optional]
**reply_markup** | Option<[**crate::models::InlineKeyboardMarkup**](InlineKeyboardMarkup.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


