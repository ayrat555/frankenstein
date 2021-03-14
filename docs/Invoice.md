# Invoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | **String** | Product name | 
**description** | **String** | Product description | 
**start_parameter** | **String** | Unique bot deep-linking parameter that can be used to generate this invoice | 
**currency** | **String** | Three-letter ISO 4217 [currency](/bots/payments#supported-currencies) code | 
**total_amount** | **i32** | Total price in the *smallest units* of the currency (integer, **not** float/double). For example, for a price of `US$ 1.45` pass `amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


