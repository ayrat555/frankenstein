# SuccessfulPayment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**currency** | **String** | Three-letter ISO 4217 [currency](/bots/payments#supported-currencies) code | 
**total_amount** | **i32** | Total price in the *smallest units* of the currency (integer, **not** float/double). For example, for a price of `US$ 1.45` pass `amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). | 
**invoice_payload** | **String** | Bot specified invoice payload | 
**shipping_option_id** | Option<**String**> | *Optional*. Identifier of the shipping option chosen by the user | [optional]
**order_info** | Option<[**crate::models::OrderInfo**](OrderInfo.md)> |  | [optional]
**telegram_payment_charge_id** | **String** | Telegram payment identifier | 
**provider_payment_charge_id** | **String** | Provider payment identifier | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


