# WebhookInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | Webhook URL, may be empty if webhook is not set up | 
**has_custom_certificate** | **bool** | True, if a custom certificate was provided for webhook certificate checks | 
**pending_update_count** | **i32** | Number of updates awaiting delivery | 
**ip_address** | Option<**String**> | *Optional*. Currently used webhook IP address | [optional]
**last_error_date** | Option<**i32**> | *Optional*. Unix time for the most recent error that happened when trying to deliver an update via webhook | [optional]
**last_error_message** | Option<**String**> | *Optional*. Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook | [optional]
**max_connections** | Option<**i32**> | *Optional*. Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery | [optional]
**allowed_updates** | Option<**Vec<String>**> | *Optional*. A list of update types the bot is subscribed to. Defaults to all update types except *chat\\_member* | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


