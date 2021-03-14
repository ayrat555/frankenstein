# EncryptedPassportElement

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | **String** | Element type. One of “personal\\_details”, “passport”, “driver\\_license”, “identity\\_card”, “internal\\_passport”, “address”, “utility\\_bill”, “bank\\_statement”, “rental\\_agreement”, “passport\\_registration”, “temporary\\_registration”, “phone\\_number”, “email”. | 
**data** | Option<**String**> | *Optional*. Base64-encoded encrypted Telegram Passport element data provided by the user, available for “personal\\_details”, “passport”, “driver\\_license”, “identity\\_card”, “internal\\_passport” and “address” types. Can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials). | [optional]
**phone_number** | Option<**String**> | *Optional*. User's verified phone number, available only for “phone\\_number” type | [optional]
**email** | Option<**String**> | *Optional*. User's verified email address, available only for “email” type | [optional]
**files** | Option<[**Vec<crate::models::PassportFile>**](PassportFile.md)> | *Optional*. Array of encrypted files with documents provided by the user, available for “utility\\_bill”, “bank\\_statement”, “rental\\_agreement”, “passport\\_registration” and “temporary\\_registration” types. Files can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials). | [optional]
**front_side** | Option<[**crate::models::PassportFile**](PassportFile.md)> |  | [optional]
**reverse_side** | Option<[**crate::models::PassportFile**](PassportFile.md)> |  | [optional]
**selfie** | Option<[**crate::models::PassportFile**](PassportFile.md)> |  | [optional]
**translation** | Option<[**Vec<crate::models::PassportFile>**](PassportFile.md)> | *Optional*. Array of encrypted files with translated versions of documents provided by the user. Available if requested for “passport”, “driver\\_license”, “identity\\_card”, “internal\\_passport”, “utility\\_bill”, “bank\\_statement”, “rental\\_agreement”, “passport\\_registration” and “temporary\\_registration” types. Files can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials). | [optional]
**hash** | **String** | Base64-encoded element hash for using in [PassportElementErrorUnspecified](https://core.telegram.org/bots/api/#passportelementerrorunspecified) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


