# Location

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**longitude** | **f32** | Longitude as defined by sender | 
**latitude** | **f32** | Latitude as defined by sender | 
**horizontal_accuracy** | Option<**f32**> | *Optional*. The radius of uncertainty for the location, measured in meters; 0-1500 | [optional]
**live_period** | Option<**i32**> | *Optional*. Time relative to the message sending date, during which the location can be updated, in seconds. For active live locations only. | [optional]
**heading** | Option<**i32**> | *Optional*. The direction in which user is moving, in degrees; 1-360. For active live locations only. | [optional]
**proximity_alert_radius** | Option<**i32**> | *Optional*. Maximum distance for proximity alerts about approaching another chat member, in meters. For sent live locations only. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


