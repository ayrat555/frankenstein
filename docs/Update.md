# Update

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**update_id** | **i32** | The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you're using [Webhooks](https://core.telegram.org/bots/api/#setwebhook), since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially. | 
**message** | Option<[**crate::models::Message**](Message.md)> |  | [optional]
**edited_message** | Option<[**crate::models::Message**](Message.md)> |  | [optional]
**channel_post** | Option<[**crate::models::Message**](Message.md)> |  | [optional]
**edited_channel_post** | Option<[**crate::models::Message**](Message.md)> |  | [optional]
**inline_query** | Option<[**crate::models::InlineQuery**](InlineQuery.md)> |  | [optional]
**chosen_inline_result** | Option<[**crate::models::ChosenInlineResult**](ChosenInlineResult.md)> |  | [optional]
**callback_query** | Option<[**crate::models::CallbackQuery**](CallbackQuery.md)> |  | [optional]
**shipping_query** | Option<[**crate::models::ShippingQuery**](ShippingQuery.md)> |  | [optional]
**pre_checkout_query** | Option<[**crate::models::PreCheckoutQuery**](PreCheckoutQuery.md)> |  | [optional]
**poll** | Option<[**crate::models::Poll**](Poll.md)> |  | [optional]
**poll_answer** | Option<[**crate::models::PollAnswer**](PollAnswer.md)> |  | [optional]
**my_chat_member** | Option<[**crate::models::ChatMemberUpdated**](ChatMemberUpdated.md)> |  | [optional]
**chat_member** | Option<[**crate::models::ChatMemberUpdated**](ChatMemberUpdated.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


