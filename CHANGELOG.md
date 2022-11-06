## 0.21.0 (2022-10-06)

### [Bot API 6.3](https://core.telegram.org/bots/api#november-5-2022) - [#91](https://github.com/ayrat555/frankenstein/pull/91)

  * Added the fields `is_topic_message` and `message_thread_id` to the struct `Message` to allow detection of messages belonging to a forum topic and their message thread identifier.
  * Added the structs `ForumTopicCreated`, `ForumTopicClosed`, and `ForumTopicReopened` and the fields `forum_topic_created`, `forum_topic_closed`, and `forum_topic_reopened` to the struct `Message`
  * Added the field `can_manage_topics` to the structs `ChatAdministratorRights`, `ChatPermissions`, `ChatMemberAdministrator`, and `ChatMemberRestricted`.
  * Added the parameter `can_manage_topics` to the struct `PromoteChatMemberParams`.
  * Added the methods `create_forum_topic`, `edit_forum_topic`, `close_forum_topic`, `reopen_forum_topic`, `delete_forum_topic`, `unpin_all_forum_topic_messages`, and `get_forum_topic_icon_stickers` for forum topic management.
  * Added the field `message_thread_id` to the methods `SendMessageParams`, `SendPhotoParams`, `SendVideoParams`, `SendAnimationParams`, `SendAudioParams`, `SendDocumentParams`, `SendStickerParams`, `SendVideoNoteParams`, `SendVoiceParams`, `SendLocationParams`, `SendVenueParams`, `SendContactParams`, `SendPollParams`, `SendDiceParams`, `SendInvoiceParams`, `SendGameParams`, `SendMediaGroupParams`, `CopyMessageParams`, `ForwardMessageParams` to support sending of messages to a forum topic.
  * Added support for Multiple Usernames via the field `active_usernames` in the struct Chat.
  * Added the field `emoji_status_custom_emoji_id` to the struct `Chat`.

## 0.20.0 (2022-08-13)

  * Change the offset field's type for getUpdates - [#86](https://github.com/ayrat555/frankenstein/pull/86)
  * Change connect_timeout for async client - [#83](https://github.com/ayrat555/frankenstein/pull/83)

### [Bot API 6.2](https://core.telegram.org/bots/api#august-12-2022) - [#85](https://github.com/ayrat555/frankenstein/pull/85)

  * Added the MessageEntity type “custom_emoji”.
  * Added the field custom_emoji_id to the class MessageEntity for “custom_emoji” entities.
  * Added the method getCustomEmojiStickers.
  * Added the fields type and custom_emoji_id to the class Sticker.
  * Added the field sticker_type to the class StickerSet, describing the type of stickers in the set.
  * The field contains_masks has been removed from the documentation of the class StickerSet. The field is still returned in the object for backward compatibility, but new bots should use the field sticker_type instead.
  * Added the parameter sticker_type to the method createNewStickerSet.
  * The parameter contains_masks has been removed from the documentation of the method createNewStickerSet. The parameter will still work for backward compatibility, but new bots should use the parameter sticker_type instead.

## 0.19.2 (2022-07-20)

  * Fix: Use client timeout of 500 seconds by @EdJoPaTo in [#82](https://github.com/ayrat555/frankenstein/pull/82)
  * Chore: build with all features on docs.rs by @OpenByteDev in [#81](https://github.com/ayrat555/frankenstein/pull/81)

## 0.19.1 (2022-07-09)

  * Fix status field of banned chat members [#78](https://github.com/ayrat555/frankenstein/pull/78)

## 0.19.0 (2022-07-06)

  * Add builders for api clients [#77](https://github.com/ayrat555/frankenstein/pull/77)

## 0.18.0 (2022-06-21)
### [Bot API 6.1](https://core.telegram.org/bots/api-changelog#june-20-2022) - [#73](https://github.com/ayrat555/frankenstein/pull/73)

  * Added the fields `join_to_send_messages` and `join_by_request` to the struct `Chat`.
  * Added the method `create_invoice_link` to generate an HTTP link for an invoice
  * Added the field `is_premium` to the struct `User`.
  * Added the field `premium_animation` to the struct `Sticker`.
  * Added the field `added_to_attachment_menu` to the struct `User`.
  * Added the parameter `secret_token` to the method `set_webhook`.

## 0.17.0 (2022-06-16)

  * Change type of file_size from u32 to u64 by @ayrat555 in [#70](https://github.com/ayrat555/frankenstein/pull/70)
  * Refactor: specify Eq when possible on PartialEq by @EdJoPaTo in [#71](https://github.com/ayrat555/frankenstein/pull/71)
  * Refactor: adopt breaking lints by @EdJoPaTo in [#72](https://github.com/ayrat555/frankenstein/pull/72)

## 0.16.0 (2022-06-07)

  * Reduce `Message` struct memory size [#69](https://github.com/ayrat555/frankenstein/pull/69)

## 0.15.1 (2022-06-02)

  * Add `new_with_client` to AsyncApi [#66](https://github.com/ayrat555/frankenstein/pull/66)

## 0.15.0 (2022-05-11)

  * Convert allowed_update type into enum - [#65](https://github.com/ayrat555/frankenstein/pull/65)

## 0.14.0 (2022-05-09)

  * Optimize Update struct by migrating its content to enum - [#62](https://github.com/ayrat555/frankenstein/pull/62)

## 0.13.0 (2022-04-18)

### [Bot API 6.0](https://core.telegram.org/bots/api#april-16-2022) - [#58](https://github.com/ayrat555/frankenstein/pull/58), [#59](https://github.com/ayrat555/frankenstein/pull/59)

  * Added support for Web Apps, see the [detailed manual here](https://core.telegram.org/bots/webapps). ([blog announcement](https://telegram.org/blog/notifications-bots))
  * Added the class [WebAppInfo](https://core.telegram.org/bots/api#webappinfo) and the fields web_app to the classes [KeyboardButton](https://core.telegram.org/bots/api#keyboardbutton) and [InlineKeyboardButton](https://core.telegram.org/bots/api#inlinekeyboardbutton).
  * Added the class [SentWebAppMessage](https://core.telegram.org/bots/api#sentwebappmessage) and the method [answerWebAppQuery](https://core.telegram.org/bots/api#answerwebappquery) for sending an answer to a Web App query, which originated from an inline button of the 'web_app' type.
  * Added the class [WebAppData](https://core.telegram.org/bots/api#webappdata) and the field web_app_data to the class [Message](https://core.telegram.org/bots/api#message).
  * Added the class [MenuButton](https://core.telegram.org/bots/api#menubutton) and the methods [setChatMenuButton](https://core.telegram.org/bots/api#setchatmenubutton) and [getChatMenuButton](https://core.telegram.org/bots/api#getchatmenubutton) for managing the behavior of the bot's menu button in private chats.
  * Added the class [ChatAdministratorRights](https://core.telegram.org/bots/api#chatadministratorrights) and the methods [setMyDefaultAdministratorRights](https://core.telegram.org/bots/api#setmydefaultadministratorrights) and [getMyDefaultAdministratorRights](https://core.telegram.org/bots/api#getmydefaultadministratorrights) for managing the bot's default administrator rights.
  * Added support for t.me links that can be used to add the bot to groups and channels as an administrator.
  * Added the field last_synchronization_error_date to the class [WebhookInfo](https://core.telegram.org/bots/api#webhookinfo).
  * Renamed the field can_manage_voice_chats to can_manage_video_chats in the class [ChatMemberAdministrator](https://core.telegram.org/bots/api#chatmemberadministrator). The old field will remain temporarily available.
  * Renamed the parameter can_manage_voice_chats to can_manage_video_chats in the method [promoteChatMember](https://core.telegram.org/bots/api#promotechatmember). The old parameter will remain temporarily available.
  * Renamed the fields voice_chat_scheduled, voice_chat_started, voice_chat_ended, and voice_chat_participants_invited to video_chat_scheduled, video_chat_started, video_chat_ended, and video_chat_participants_invited in the class [Message](https://core.telegram.org/bots/api#message). The old fields will remain temporarily available.

## 0.12.0 (2022-03-20)

  * Switch from `derive_builder` to `typed-builder` - [#53](https://github.com/ayrat555/frankenstein/pull/53)

## 0.11.0 (2022-03-19)

  * Add ParseMode enum - [#49](https://github.com/ayrat555/frankenstein/pull/49)
  * Derive Copy when possible - [#50](https://github.com/ayrat555/frankenstein/pull/50)
  * Improve error handling - [#51](https://github.com/ayrat555/frankenstein/pull/51)

## 0.10.1 (2022-02-08)

  * Remove serde_json from trait features - [#43](https://github.com/ayrat555/frankenstein/pull/43)

## 0.10.0 (2022-02-08)

  * Add optional async api - [#38](https://github.com/ayrat555/frankenstein/pull/38)

## 0.9.5 (2022-02-02)

  * Disable unused features in the multipart crate - [#41](https://github.com/ayrat555/frankenstein/pull/41)

## 0.9.4 (2022-02-01)

  * Bot API 5.7 Changes - [#40](https://github.com/ayrat555/frankenstein/pull/40)

## 0.9.3 (2022-01-15)

  * Configure request timeout - [#39](https://github.com/ayrat555/frankenstein/pull/39)
  * Use ureq's agent to make requests  - [#39](https://github.com/ayrat555/frankenstein/pull/39)

## 0.9.2 (2022-01-01)

  * [Telegram Bot API 5.6 Changes](https://core.telegram.org/bots/api#december-30-2021) - [#36](https://github.com/ayrat555/frankenstein/pull/36)

## 0.9.1 (2021-12-24)

  * Add From trait impl for File enum - [#35](https://github.com/ayrat555/frankenstein/pull/35)

## 0.9.0 (2021-12-13)

  * Use builders for all structs - [#34](https://github.com/ayrat555/frankenstein/pull/34)

## 0.8.1 (2021-12-08)

  * [Telegram Bot API 5.5 Changes](https://core.telegram.org/bots/api#december-7-2021) - [#33](https://github.com/ayrat555/frankenstein/pull/33)

## 0.8.0 (2021-12-05)

### [#32](https://github.com/ayrat555/frankenstein/pull/32):

  * fix InputMessageContent enum
  * improve error handling

## 0.7.1 (2021-11-08)

  * Rename impl feature to http-client - [#30](https://github.com/ayrat555/frankenstein/pull/30)
  * Improve CI - [#31](https://github.com/ayrat555/frankenstein/pull/31)

## 0.7.0 (2021-11-06)

  * [Telegram API 5.4 Changes](https://core.telegram.org/bots/api#november-5-2021) - [#27](https://github.com/ayrat555/frankenstein/pull/27)
  * Update ureq to 2.3 - [#27](https://github.com/ayrat555/frankenstein/pull/27)

## 0.6.0 (2021-08-26)

  * Use enums instead of strings for type fields - [#25](https://github.com/ayrat555/frankenstein/pull/25)

## 0.5.3 (2021-08-14)

  * Make Api struct cloneable - [#23](https://github.com/ayrat555/frankenstein/pull/23)

## 0.5.2 (2021-07-10)

  * Make SendChatActionParams.action an enum - [#22](https://github.com/ayrat555/frankenstein/pull/22)

## 0.5.1 (2021-07-06)

  * Fix ReplyKeyboardMarkup serialization error - [#20](https://github.com/ayrat555/frankenstein/pull/20)

## 0.5.0 (2021-06-26)

### [Telegram API 5.3 Changes](https://core.telegram.org/bots/api#june-25-2021):

  * add BotCommandScope - [#b937a5442d](https://github.com/ayrat555/frankenstein/commit/b937a5442dfd7f756593e02c897130ed681fc4f6)
  * add input_field_placeholder - [#371fe3463](https://github.com/ayrat555/frankenstein/commit/371fe3463c44e40c4999e8ab7224014b903821d5)
  * split ChatMember struct into 6 structs - [#8b104155a8](https://github.com/ayrat555/frankenstein/commit/8b104155a84220611d0e7666de5f63d630902d5c)
  * rename kickChatMember and getChatMembersCount - [#35819f8843a88c](https://github.com/ayrat555/frankenstein/commit/35819f8843a88ccf2cd56f098082565274afb131)

### Other Changes

  * Move type field into enums - [#19](https://github.com/ayrat555/frankenstein/pull/19)

## 0.4.0 (2021-06-13)

- Feature: Get rid of isize/usize types ([#11](https://github.com/ayrat555/frankenstein/pull/11), [#16](https://github.com/ayrat555/frankenstein/pull/16), [ab72b4469b3ed](https://github.com/ayrat555/frankenstein/commit/ab72b4469b3edf89d84f07ce17770605aa9068b2))
- Chore: Fix clippy::pedantic warnings ([#12](https://github.com/ayrat555/frankenstein/pull/12), [#14](https://github.com/ayrat555/frankenstein/pull/14))
- Chore: remove `Enum` postfix from enums ([#13](https://github.com/ayrat555/frankenstein/pull/13))
- Chore: bump multipart from 0.17 to 0.18 ([#15](https://github.com/ayrat555/frankenstein/pull/15))

## 0.3.1 (2021-05-15)

- Make serde_json optional dependency ([#6](https://github.com/ayrat555/frankenstein/pull/6))

## 0.3.0 (2021-05-09)

- Create `TelegramApi` trait ([#5](https://github.com/ayrat555/frankenstein/pull/5))
- Fix `InlineKeyboardMarkup` type ([7392c1dac9a42fbb66e05dc905b79ff849be289b](https://github.com/ayrat555/frankenstein/commit/7392c1dac9a42fbb66e05dc905b79ff849be289b))

## 0.2.1 (2021-05-02)

- Make all struct fields public ([0798a34dddcbce1030a2f20b478e455b8f289623](https://github.com/ayrat555/frankenstein/commit/0798a34dddcbce1030a2f20b478e455b8f289623))

## 0.2.0 (2021-05-01)

- The first release on crates.io
