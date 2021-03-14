# Poll

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique poll identifier | 
**question** | **String** | Poll question, 1-300 characters | 
**options** | [**Vec<crate::models::PollOption>**](PollOption.md) | List of poll options | 
**total_voter_count** | **i32** | Total number of users that voted in the poll | 
**is_closed** | **bool** | True, if the poll is closed | 
**is_anonymous** | **bool** | True, if the poll is anonymous | 
**_type** | **String** | Poll type, currently can be “regular” or “quiz” | 
**allows_multiple_answers** | **bool** | True, if the poll allows multiple answers | 
**correct_option_id** | Option<**i32**> | *Optional*. 0-based identifier of the correct answer option. Available only for polls in the quiz mode, which are closed, or was sent (not forwarded) by the bot or to the private chat with the bot. | [optional]
**explanation** | Option<**String**> | *Optional*. Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters | [optional]
**explanation_entities** | Option<[**Vec<crate::models::MessageEntity>**](MessageEntity.md)> | *Optional*. Special entities like usernames, URLs, bot commands, etc. that appear in the *explanation* | [optional]
**open_period** | Option<**i32**> | *Optional*. Amount of time in seconds the poll will be active after creation | [optional]
**close_date** | Option<**i32**> | *Optional*. Point in time (Unix timestamp) when the poll will be automatically closed | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


