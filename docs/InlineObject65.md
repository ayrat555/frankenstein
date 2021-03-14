# InlineObject65

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chat_id** | **i32** | Unique identifier for the target private chat | 
**title** | **String** | Product name, 1-32 characters | 
**description** | **String** | Product description, 1-255 characters | 
**payload** | **String** | Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes. | 
**provider_token** | **String** | Payments provider token, obtained via [Botfather](https://t.me/botfather) | 
**start_parameter** | **String** | Unique deep-linking parameter that can be used to generate this invoice when used as a start parameter | 
**currency** | **String** | Three-letter ISO 4217 currency code, see [more on currencies](/bots/payments#supported-currencies) | 
**prices** | [**Vec<crate::models::LabeledPrice>**](LabeledPrice.md) | Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.) | 
**provider_data** | Option<**String**> | A JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider. | [optional]
**photo_url** | Option<**String**> | URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for. | [optional]
**photo_size** | Option<**i32**> | Photo size | [optional]
**photo_width** | Option<**i32**> | Photo width | [optional]
**photo_height** | Option<**i32**> | Photo height | [optional]
**need_name** | Option<**bool**> | Pass *True*, if you require the user's full name to complete the order | [optional]
**need_phone_number** | Option<**bool**> | Pass *True*, if you require the user's phone number to complete the order | [optional]
**need_email** | Option<**bool**> | Pass *True*, if you require the user's email address to complete the order | [optional]
**need_shipping_address** | Option<**bool**> | Pass *True*, if you require the user's shipping address to complete the order | [optional]
**send_phone_number_to_provider** | Option<**bool**> | Pass *True*, if user's phone number should be sent to provider | [optional]
**send_email_to_provider** | Option<**bool**> | Pass *True*, if user's email address should be sent to provider | [optional]
**is_flexible** | Option<**bool**> | Pass *True*, if the final price depends on the shipping method | [optional]
**disable_notification** | Option<**bool**> | Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound. | [optional]
**reply_to_message_id** | Option<**i32**> | If the message is a reply, ID of the original message | [optional]
**allow_sending_without_reply** | Option<**bool**> | Pass *True*, if the message should be sent even if the specified replied-to message is not found | [optional]
**reply_markup** | Option<[**crate::models::InlineKeyboardMarkup**](InlineKeyboardMarkup.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


