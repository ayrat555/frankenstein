# InlineObject19

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chat_id** | [**crate::models::AnyOfintegerstring**](anyOf<integer,string>.md) | Unique identifier for the target chat or username of the target channel (in the format `@channelusername`) | 
**question** | **String** | Poll question, 1-300 characters | 
**options** | **Vec<String>** | A JSON-serialized list of answer options, 2-10 strings 1-100 characters each | 
**is_anonymous** | Option<**bool**> | True, if the poll needs to be anonymous, defaults to *True* | [optional]
**_type** | Option<**String**> | Poll type, “quiz” or “regular”, defaults to “regular” | [optional]
**allows_multiple_answers** | Option<**bool**> | True, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to *False* | [optional]
**correct_option_id** | Option<**i32**> | 0-based identifier of the correct answer option, required for polls in quiz mode | [optional]
**explanation** | Option<**String**> | Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing | [optional]
**explanation_parse_mode** | Option<**String**> | Mode for parsing entities in the explanation. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. | [optional]
**explanation_entities** | Option<[**Vec<crate::models::MessageEntity>**](MessageEntity.md)> | List of special entities that appear in the poll explanation, which can be specified instead of *parse\\_mode* | [optional]
**open_period** | Option<**i32**> | Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with *close\\_date*. | [optional]
**close_date** | Option<**i32**> | Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with *open\\_period*. | [optional]
**is_closed** | Option<**bool**> | Pass *True*, if the poll needs to be immediately closed. This can be useful for poll preview. | [optional]
**disable_notification** | Option<**bool**> | Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound. | [optional]
**reply_to_message_id** | Option<**i32**> | If the message is a reply, ID of the original message | [optional]
**allow_sending_without_reply** | Option<**bool**> | Pass *True*, if the message should be sent even if the specified replied-to message is not found | [optional]
**reply_markup** | Option<[**crate::models::AnyOfInlineKeyboardMarkupReplyKeyboardMarkupReplyKeyboardRemoveForceReply**](anyOf<InlineKeyboardMarkup,ReplyKeyboardMarkup,ReplyKeyboardRemove,ForceReply>.md)> | Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating), [custom reply keyboard](https://core.telegram.org/bots#keyboards), instructions to remove reply keyboard or to force a reply from the user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


