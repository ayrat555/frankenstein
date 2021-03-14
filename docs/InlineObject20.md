# InlineObject20

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chat_id** | [**crate::models::AnyOfintegerstring**](anyOf<integer,string>.md) | Unique identifier for the target chat or username of the target channel (in the format `@channelusername`) | 
**emoji** | Option<**String**> | Emoji on which the dice throw animation is based. Currently, must be one of “<img alt=\"🎲\" src=\"//telegram.org/img/emoji/40/F09F8EB2.png\" height=\"20\" width=\"20\" />”, “<img alt=\"🎯\" src=\"//telegram.org/img/emoji/40/F09F8EAF.png\" height=\"20\" width=\"20\" />”, “<img alt=\"🏀\" src=\"//telegram.org/img/emoji/40/F09F8F80.png\" height=\"20\" width=\"20\" />”, “<img alt=\"⚽\" src=\"//telegram.org/img/emoji/40/E29ABD.png\" height=\"20\" width=\"20\" />”, “<img alt=\"🎳\" src=\"//telegram.org/img/emoji/40/F09F8EB3.png\" height=\"20\" width=\"20\" />”, or “<img alt=\"🎰\" src=\"//telegram.org/img/emoji/40/F09F8EB0.png\" height=\"20\" width=\"20\" />”. Dice can have values 1-6 for “<img alt=\"🎲\" src=\"//telegram.org/img/emoji/40/F09F8EB2.png\" height=\"20\" width=\"20\" />”, “<img alt=\"🎯\" src=\"//telegram.org/img/emoji/40/F09F8EAF.png\" height=\"20\" width=\"20\" />” and “<img alt=\"🎳\" src=\"//telegram.org/img/emoji/40/F09F8EB3.png\" height=\"20\" width=\"20\" />”, values 1-5 for “<img alt=\"🏀\" src=\"//telegram.org/img/emoji/40/F09F8F80.png\" height=\"20\" width=\"20\" />” and “<img alt=\"⚽\" src=\"//telegram.org/img/emoji/40/E29ABD.png\" height=\"20\" width=\"20\" />”, and values 1-64 for “<img alt=\"🎰\" src=\"//telegram.org/img/emoji/40/F09F8EB0.png\" height=\"20\" width=\"20\" />”. Defaults to “<img alt=\"🎲\" src=\"//telegram.org/img/emoji/40/F09F8EB2.png\" height=\"20\" width=\"20\" />” | [optional][default to Emoji_]
**disable_notification** | Option<**bool**> | Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound. | [optional]
**reply_to_message_id** | Option<**i32**> | If the message is a reply, ID of the original message | [optional]
**allow_sending_without_reply** | Option<**bool**> | Pass *True*, if the message should be sent even if the specified replied-to message is not found | [optional]
**reply_markup** | Option<[**crate::models::AnyOfInlineKeyboardMarkupReplyKeyboardMarkupReplyKeyboardRemoveForceReply**](anyOf<InlineKeyboardMarkup,ReplyKeyboardMarkup,ReplyKeyboardRemove,ForceReply>.md)> | Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating), [custom reply keyboard](https://core.telegram.org/bots#keyboards), instructions to remove reply keyboard or to force a reply from the user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


