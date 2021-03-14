# Game

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | **String** | Title of the game | 
**description** | **String** | Description of the game | 
**photo** | [**Vec<crate::models::PhotoSize>**](PhotoSize.md) | Photo that will be displayed in the game message in chats. | 
**text** | Option<**String**> | *Optional*. Brief description of the game or high scores included in the game message. Can be automatically edited to include current high scores for the game when the bot calls [setGameScore](https://core.telegram.org/bots/api/#setgamescore), or manually edited using [editMessageText](https://core.telegram.org/bots/api/#editmessagetext). 0-4096 characters. | [optional]
**text_entities** | Option<[**Vec<crate::models::MessageEntity>**](MessageEntity.md)> | *Optional*. Special entities that appear in *text*, such as usernames, URLs, bot commands, etc. | [optional]
**animation** | Option<[**crate::models::Animation**](Animation.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


