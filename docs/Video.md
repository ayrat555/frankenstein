# Video

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file_id** | **String** | Identifier for this file, which can be used to download or reuse the file | 
**file_unique_id** | **String** | Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file. | 
**width** | **i32** | Video width as defined by sender | 
**height** | **i32** | Video height as defined by sender | 
**duration** | **i32** | Duration of the video in seconds as defined by sender | 
**thumb** | Option<[**crate::models::PhotoSize**](PhotoSize.md)> |  | [optional]
**file_name** | Option<**String**> | *Optional*. Original filename as defined by sender | [optional]
**mime_type** | Option<**String**> | *Optional*. Mime type of a file as defined by sender | [optional]
**file_size** | Option<**i32**> | *Optional*. File size | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


