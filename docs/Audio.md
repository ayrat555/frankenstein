# Audio

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file_id** | **String** | Identifier for this file, which can be used to download or reuse the file | 
**file_unique_id** | **String** | Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file. | 
**duration** | **i32** | Duration of the audio in seconds as defined by sender | 
**performer** | Option<**String**> | *Optional*. Performer of the audio as defined by sender or by audio tags | [optional]
**title** | Option<**String**> | *Optional*. Title of the audio as defined by sender or by audio tags | [optional]
**file_name** | Option<**String**> | *Optional*. Original filename as defined by sender | [optional]
**mime_type** | Option<**String**> | *Optional*. MIME type of the file as defined by sender | [optional]
**file_size** | Option<**i32**> | *Optional*. File size | [optional]
**thumb** | Option<[**crate::models::PhotoSize**](PhotoSize.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


