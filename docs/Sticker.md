# Sticker

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file_id** | **String** | Identifier for this file, which can be used to download or reuse the file | 
**file_unique_id** | **String** | Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file. | 
**width** | **i32** | Sticker width | 
**height** | **i32** | Sticker height | 
**is_animated** | **bool** | *True*, if the sticker is [animated](https://telegram.org/blog/animated-stickers) | 
**thumb** | Option<[**crate::models::PhotoSize**](PhotoSize.md)> |  | [optional]
**emoji** | Option<**String**> | *Optional*. Emoji associated with the sticker | [optional]
**set_name** | Option<**String**> | *Optional*. Name of the sticker set to which the sticker belongs | [optional]
**mask_position** | Option<[**crate::models::MaskPosition**](MaskPosition.md)> |  | [optional]
**file_size** | Option<**i32**> | *Optional*. File size | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


