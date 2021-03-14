# InlineKeyboardButton

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text** | **String** | Label text on the button | 
**url** | Option<**String**> | *Optional*. HTTP or tg:// url to be opened when button is pressed | [optional]
**login_url** | Option<[**crate::models::LoginUrl**](LoginUrl.md)> |  | [optional]
**callback_data** | Option<**String**> | *Optional*. Data to be sent in a [callback query](https://core.telegram.org/bots/api/#callbackquery) to the bot when button is pressed, 1-64 bytes | [optional]
**switch_inline_query** | Option<**String**> | *Optional*. If set, pressing the button will prompt the user to select one of their chats, open that chat and insert the bot's username and the specified inline query in the input field. Can be empty, in which case just the bot's username will be inserted.    **Note:** This offers an easy way for users to start using your bot in [inline mode](/bots/inline) when they are currently in a private chat with it. Especially useful when combined with [*switch\\_pm…*](https://core.telegram.org/bots/api/#answerinlinequery) actions – in this case the user will be automatically returned to the chat they switched from, skipping the chat selection screen. | [optional]
**switch_inline_query_current_chat** | Option<**String**> | *Optional*. If set, pressing the button will insert the bot's username and the specified inline query in the current chat's input field. Can be empty, in which case only the bot's username will be inserted.    This offers a quick way for the user to open your bot in inline mode in the same chat – good for selecting something from multiple options. | [optional]
**callback_game** | Option<[**serde_json::Value**](.md)> | A placeholder, currently holds no information. Use [BotFather](https://t.me/botfather) to set up your game. | [optional]
**pay** | Option<**bool**> | *Optional*. Specify True, to send a [Pay button](https://core.telegram.org/bots/api/#payments).    **NOTE:** This type of button **must** always be the first button in the first row. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


