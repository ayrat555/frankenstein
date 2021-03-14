# VideoNote

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file_id** | **String** | Identifier for this file, which can be used to download or reuse the file | 
**file_unique_id** | **String** | Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file. | 
**length** | **i32** | Video width and height (diameter of the video message) as defined by sender | 
**duration** | **i32** | Duration of the video in seconds as defined by sender | 
**thumb** | Option<[**crate::models::PhotoSize**](PhotoSize.md)> |  | [optional]
**file_size** | Option<**i32**> | *Optional*. File size | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


