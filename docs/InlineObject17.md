# InlineObject17

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chat_id** | [**crate::models::AnyOfintegerstring**](anyOf<integer,string>.md) | Unique identifier for the target chat or username of the target channel (in the format `@channelusername`) | 
**latitude** | **f32** | Latitude of the venue | 
**longitude** | **f32** | Longitude of the venue | 
**title** | **String** | Name of the venue | 
**address** | **String** | Address of the venue | 
**foursquare_id** | Option<**String**> | Foursquare identifier of the venue | [optional]
**foursquare_type** | Option<**String**> | Foursquare type of the venue, if known. (For example, “arts\\_entertainment/default”, “arts\\_entertainment/aquarium” or “food/icecream”.) | [optional]
**google_place_id** | Option<**String**> | Google Places identifier of the venue | [optional]
**google_place_type** | Option<**String**> | Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).) | [optional]
**disable_notification** | Option<**bool**> | Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound. | [optional]
**reply_to_message_id** | Option<**i32**> | If the message is a reply, ID of the original message | [optional]
**allow_sending_without_reply** | Option<**bool**> | Pass *True*, if the message should be sent even if the specified replied-to message is not found | [optional]
**reply_markup** | Option<[**crate::models::AnyOfInlineKeyboardMarkupReplyKeyboardMarkupReplyKeyboardRemoveForceReply**](anyOf<InlineKeyboardMarkup,ReplyKeyboardMarkup,ReplyKeyboardRemove,ForceReply>.md)> | Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating), [custom reply keyboard](https://core.telegram.org/bots#keyboards), instructions to remove reply keyboard or to force a reply from the user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


