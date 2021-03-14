# InlineObject48

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**callback_query_id** | **String** | Unique identifier for the query to be answered | 
**text** | Option<**String**> | Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters | [optional]
**show_alert** | Option<**bool**> | If *true*, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to *false*. | [optional][default to false]
**url** | Option<**String**> | URL that will be opened by the user's client. If you have created a [Game](https://core.telegram.org/bots/api/#game) and accepted the conditions via [@Botfather](https://t.me/botfather), specify the URL that opens your game — note that this will only work if the query comes from a [*callback\\_game*](https://core.telegram.org/bots/api/#inlinekeyboardbutton) button.    Otherwise, you may use links like `t.me/your_bot?start=XXXX` that open your bot with a parameter. | [optional]
**cache_time** | Option<**i32**> | The maximum amount of time in seconds that the result of the callback query may be cached client-side. Telegram apps will support caching starting in version 3.14. Defaults to 0. | [optional][default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


