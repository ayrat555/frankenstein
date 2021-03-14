# InlineObject3

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chat_id** | [**crate::models::AnyOfintegerstring**](anyOf<integer,string>.md) | Unique identifier for the target chat or username of the target channel (in the format `@channelusername`) | 
**text** | **String** | Text of the message to be sent, 1-4096 characters after entities parsing | 
**parse_mode** | Option<**String**> | Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. | [optional]
**entities** | Option<[**Vec<crate::models::MessageEntity>**](MessageEntity.md)> | List of special entities that appear in message text, which can be specified instead of *parse\\_mode* | [optional]
**disable_web_page_preview** | Option<**bool**> | Disables link previews for links in this message | [optional]
**disable_notification** | Option<**bool**> | Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound. | [optional]
**reply_to_message_id** | Option<**i32**> | If the message is a reply, ID of the original message | [optional]
**allow_sending_without_reply** | Option<**bool**> | Pass *True*, if the message should be sent even if the specified replied-to message is not found | [optional]
**reply_markup** | Option<[**crate::models::AnyOfInlineKeyboardMarkupReplyKeyboardMarkupReplyKeyboardRemoveForceReply**](anyOf<InlineKeyboardMarkup,ReplyKeyboardMarkup,ReplyKeyboardRemove,ForceReply>.md)> | Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating), [custom reply keyboard](https://core.telegram.org/bots#keyboards), instructions to remove reply keyboard or to force a reply from the user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


