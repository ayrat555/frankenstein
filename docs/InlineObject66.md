# InlineObject66

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipping_query_id** | **String** | Unique identifier for the query to be answered | 
**ok** | **bool** | Specify True if delivery to the specified address is possible and False if there are any problems (for example, if delivery to the specified address is not possible) | 
**shipping_options** | Option<[**Vec<crate::models::ShippingOption>**](ShippingOption.md)> | Required if *ok* is True. A JSON-serialized array of available shipping options. | [optional]
**error_message** | Option<**String**> | Required if *ok* is False. Error message in human readable form that explains why it is impossible to complete the order (e.g. \"Sorry, delivery to your desired address is unavailable'). Telegram will display this message to the user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


