# Changelog

## 0.42.0 (2025-07-04)

* fix!: make year in Birthdate optional - [#286](https://github.com/ayrat555/frankenstein/pull/286)
* fix: fix add_sticker_to_set, add sticker examples - [#284](https://github.com/ayrat555/frankenstein/pull/284)
* fix: make nanostar_amount optional in StarTransaction struct - [#288](https://github.com/ayrat555/frankenstein/pull/288)

## 0.41.0 (2025-05-26)

* refactor!: box large enum variants - [#278](https://github.com/ayrat555/frankenstein/pull/278)
* feat!: Bot API 9.0 - [#277](https://github.com/ayrat555/frankenstein/pull/277)

## 0.40.0 (2025-03-19)

* docs(examples): read token and chat from env - [#262](https://github.com/ayrat555/frankenstein/pull/262)
* refactor: simplify file handling loop - [#263](https://github.com/ayrat555/frankenstein/pull/263)
* refactor: simplify code when attaching many files - [#264](https://github.com/ayrat555/frankenstein/pull/264)
* refactor!: simplify code with internal HasInputFile trait - [#265](https://github.com/ayrat555/frankenstein/pull/265)
* refactor!: move objects into multiple modules - [#266](https://github.com/ayrat555/frankenstein/pull/266)
* refactor!: move ResponseParams to response module - [#267](https://github.com/ayrat555/frankenstein/pull/267)
* feat: add LinkPreviewOptions::DISABLED - [#268](https://github.com/ayrat555/frankenstein/pull/268)
* ci(rust): dont check features on beta - [#271](https://github.com/ayrat555/frankenstein/pull/271)
* build(lints): lint things that break API - [#272](https://github.com/ayrat555/frankenstein/pull/272)
* fix(gift): fix typo in property name - [#274](https://github.com/ayrat555/frankenstein/pull/274)
* refactor!: remove top level reexport and rename to methods / types - [#275](https://github.com/ayrat555/frankenstein/pull/275)

## 0.39.2 (2025-02-19)

* fix: attach video cover correctly as a file - [#258](https://github.com/ayrat555/frankenstein/pull/258)

## 0.39.1 (2025-02-18)

* feat: Bot API 8.3 - [#256](https://github.com/ayrat555/frankenstein/pull/256)
* build(cargo): improve Cargo.toml - [#255](https://github.com/ayrat555/frankenstein/pull/255)

## 0.39.0 (2025-02-03)

* build!: rename the features and remove the default feature - [#248](https://github.com/ayrat555/frankenstein/pull/248)
* feat(error)!: more specific errors including their source - [#252](https://github.com/ayrat555/frankenstein/pull/252)
* feat!: name clients Bot to differentiate from TelegramApi - [#253](https://github.com/ayrat555/frankenstein/pull/253)
* refactor(ureq): update to ureq v3 - [#247](https://github.com/ayrat555/frankenstein/pull/247)
* build: specify rust-version and keep lock file - [#250](https://github.com/ayrat555/frankenstein/pull/250)
* refactor(lint): fix clippy::unnecessary_semicolon - [#251](https://github.com/ayrat555/frankenstein/pull/251)

## 0.38.0 (2025-01-05)

* New update types - [241](https://github.com/ayrat555/frankenstein/pull/241)
* Bot API 8.2 - [243](https://github.com/ayrat555/frankenstein/pull/243)

## 0.37.0 (2024-12-07)

* Bot API 8.1 - [#239](https://github.com/ayrat555/frankenstein/pull/239)

## 0.36.0 (2024-12-03)

* Bot API 8.0 - [#234](https://github.com/ayrat555/frankenstein/pull/234)

## 0.35.0 (2024-11-27)

* feat: try fit wasm partially - [#225](https://github.com/ayrat555/frankenstein/pull/225)
* fix!: fix serialization of InlineQueryResult - [#230](https://github.com/ayrat555/frankenstein/pull/230)
* build(cargo): update bon requirement from 2.2.0 to 3.0.0 - [#232](https://github.com/ayrat555/frankenstein/pull/232)
* build(cargo): update thiserror requirement from 1 to 2 - [#228](https://github.com/ayrat555/frankenstein/pull/228)

## 0.34.1 (2024-11-02)

- [Bot API 7.11](https://core.telegram.org/bots/api#october-31-2024) - [#226](https://github.com/ayrat555/frankenstein/pull/226)

## 0.34.0 (2024-09-19)

- docs: show the required feature on docs.rs - [#209](https://github.com/ayrat555/frankenstein/pull/209)
- refactor!: flatten the module structure - [#208](https://github.com/ayrat555/frankenstein/pull/208)
- perf!: always take params reference - [#211](https://github.com/ayrat555/frankenstein/pull/211)
- refactor!: use MessageOrBool over EditMessageResponse - [#212](https://github.com/ayrat555/frankenstein/pull/212)
- build!: remove implicit features - [#214](https://github.com/ayrat555/frankenstein/pull/214)
- Use consistent builder derive configs across API types - [#213](https://github.com/ayrat555/frankenstein/pull/213)
- test: improve testing for api error response - [#216](https://github.com/ayrat555/frankenstein/pull/216)
- refactor!: simplify traits with macros - [#210](https://github.com/ayrat555/frankenstein/pull/210)
- style: group imports - [#217](https://github.com/ayrat555/frankenstein/pull/217)
- refactor: generalize serde logic into macro - [#218](https://github.com/ayrat555/frankenstein/pull/218)
- test(error): expect api error - [#222](https://github.com/ayrat555/frankenstein/pull/222)
- docs: deduplicate spaces - [#220](https://github.com/ayrat555/frankenstein/pull/220)
- test(json): assert_str - [#221](https://github.com/ayrat555/frankenstein/pull/221)

## 0.33.0 (2024-09-11)

- refactor!: merge Error and HttpError by @EdJoPaTo - [#204](https://github.com/ayrat555/frankenstein/pull/204)
- refactor: improve generic argument naming by @EdJoPaTo - [#205](https://github.com/ayrat555/frankenstein/pull/205)
- refactor(json)!: generalize encoding/decoding by @EdJoPaTo - [#206](https://github.com/ayrat555/frankenstein/pull/206)
- Migrate from `typed-builder` to `bon` by @Veetaha - [#196](https://github.com/ayrat555/frankenstein/pull/196)

## 0.32.5 (2024-09-07)

- [Bot API 7.10](https://core.telegram.org/bots/api#september-6-2024) - [#200](https://github.com/ayrat555/frankenstein/pull/200)
- docs(examples): simplify update offset logic - [#198](https://github.com/ayrat555/frankenstein/pull/198)
- refactor(api): reuse new_url - [#199](https://github.com/ayrat555/frankenstein/pull/199)
- fix: clippy::too_long_first_doc_paragraph - [#197](https://github.com/ayrat555/frankenstein/pull/197)
- add into attributes for chat_id in SendInvoice and SendPaidMedia - [#194](https://github.com/ayrat555/frankenstein/pull/194)
- refactor: clippy::must_use_candidate - [#191](https://github.com/ayrat555/frankenstein/pull/191)

## 0.32.4 (2024-08-18)

- [Bot API 7.9](https://core.telegram.org/bots/api#august-14-2024) - [#192](https://github.com/ayrat555/frankenstein/pull/192)
- feat: add Eq when possible on PartialEq - [#188](https://github.com/ayrat555/frankenstein/pull/188)
- refactor: simplify From<ureq::Error> - [#189](https://github.com/ayrat555/frankenstein/pull/189)
- build(cargo): enable nursery lints - [#190](https://github.com/ayrat555/frankenstein/pull/190)

## 0.32.3 (2024-08-08)

- [Bot API 7.8](https://core.telegram.org/bots/api#july-31-2024) - [#186](https://github.com/ayrat555/frankenstein/pull/186)

## 0.32.2 (2024-08-03)

- fix parsing of PaidMediaInfo - [#184](https://github.com/ayrat555/frankenstein/pull/184)

## 0.32.1 (2024-07-16)

- Update typed-builder requirement from 0.18 to 0.19 - [#180](https://github.com/ayrat555/frankenstein/pull/180)

## 0.32.0 (2024-07-14)

- [Bot API 7.4](https://core.telegram.org/bots/api#may-28-2024) - [#171](https://github.com/ayrat555/frankenstein/pull/171)
- [Bot API 7.5](https://core.telegram.org/bots/api#june-18-2024) - [#174](https://github.com/ayrat555/frankenstein/pull/174)
- [Bot API 7.6](https://core.telegram.org/bots/api#july-1-2024) - [#175](https://github.com/ayrat555/frankenstein/pull/175)
- [Bot API 7.7](https://core.telegram.org/bots/api#july-7-2024) - [#177](https://github.com/ayrat555/frankenstein/pull/177)
- fix: ChatBoostUpdated in UpdateContent - [#176](https://github.com/ayrat555/frankenstein/pull/176)

## 0.31.0 (2024-05-20)

- [Bot API 7.3](https://core.telegram.org/bots/api#may-6-2024) - [#169](https://github.com/ayrat555/frankenstein/pull/169)

## 0.30.8 (2024-04-26)

- fix: remove "status" from ChatMemberRestricted - [#165](https://github.com/ayrat555/frankenstein/pull/165)
- fix: message_id is optional in ExternalReplyInfo - [#166](https://github.com/ayrat555/frankenstein/pull/166)

## 0.30.7 (2024-04-20)

- Use i64 for ChatShared::chat_id - [#162](https://github.com/ayrat555/frankenstein/pull/162)

## 0.30.6 (2024-04-19)

- [Bot API 7.2](https://core.telegram.org/bots/api#march-31-2024) - [#155](https://github.com/ayrat555/frankenstein/pull/155)

## 0.30.5 (2024-03-26)

- Fix the winners_count field in the Giveaway struct - [#150](https://github.com/ayrat555/frankenstein/pull/150)
- Improve ci - [#152](https://github.com/ayrat555/frankenstein/pull/152)
- Fix lints - [#151](https://github.com/ayrat555/frankenstein/pull/151)

## 0.30.4 (2024-02-17)

- [Bot API 7.1](https://core.telegram.org/bots/api#february-16-2024) - [#148](https://github.com/ayrat555/frankenstein/pull/148)

## 0.30.3 (2024-02-12)

- Support unknown message entity types - [#146](https://github.com/ayrat555/frankenstein/pull/146)

## 0.30.2 (2024-01-17)

- Fix custom emoji decoding - [#143](https://github.com/ayrat555/frankenstein/pull/143)

## 0.30.1 (2024-01-07)

- Fix parsing of callback queries - [#137](https://github.com/ayrat555/frankenstein/pull/137)

## 0.30.0 (2024-01-06)

- [Bot API 7.0](https://core.telegram.org/bots/api#december-29-2023) - [#128](https://github.com/ayrat555/frankenstein/pull/128)

## 0.29.2 (2023-12-29)

- Add blockquote to `MessageEntityType` - [#126](https://github.com/ayrat555/frankenstein/pull/126)

## 0.29.1 (2023-10-14)

- Add unpin_all_chat_messages method - [#122](https://github.com/ayrat555/frankenstein/pull/122)

## 0.29.0 (2023-10-04)

- Fix parsing 'kicked' chat member status - [#119](https://github.com/ayrat555/frankenstein/pull/119)

## 0.28.0 (2023-09-24)

- [Bot API 6.9](https://core.telegram.org/bots/api#september-22-2023) - [#117](https://github.com/ayrat555/frankenstein/pull/117)

## 0.27.0 (2023-08-21)

- [Bot API 6.8](https://core.telegram.org/bots/api#august-18-2023) - [#112](https://github.com/ayrat555/frankenstein/pull/112)

## 0.26.0 (2023-06-16)

- Rename `File` enum into `FileUpload` - [#109](https://github.com/ayrat555/frankenstein/pull/109)

## 0.25.0 (2023-04-23)

- [Bot API 6.7](https://core.telegram.org/bots/api#april-21-2023) - [#106](https://github.com/ayrat555/frankenstein/pull/106)

## 0.24.1 (2023-03-18)

- Update `typed-builder` to 0.14
- Update `mockito` dependency to 1.0

## 0.24.0 (2023-03-12)

- [Bot API 6.6](https://core.telegram.org/bots/api#march-9-2023) - [#102](https://github.com/ayrat555/frankenstein/pull/102)

## 0.23.0 (2023-02-04)

### [Bot API 6.5](https://core.telegram.org/bots/api#february-3-2023) - [#97](https://github.com/ayrat555/frankenstein/pull/97)

- Added the struct `KeyboardButtonRequestUser` and the field `request_user` to the struct `KeyboardButton`.
- Added the struct `KeyboardButtonRequestChat` and the field `request_chat` to the struct `KeyboardButton`.
- Added the structs `UserShared`, `ChatShared` and the fields `user_shared`, and `chat_shared` to the struct `Message`.
- Replaced the fields `can_send_media_messages` in the structs `ChatMemberRestricted` and `ChatPermissions` with separate fields `can_send_audios`, `can_send_documents`, `can_send_photos`, `can_send_videos`, `can_send_video_notes`, and `can_send_voice_notes` for different media types.
- Added the parameter `use_independent_chat_permissions` to the structs `RestrictChatMemberParams` and `SetChatPermissionsParams`.
- Added the field `user_chat_id` to the struct `ChatJoinRequest`.

## 0.22.1 (2023-02-01)

- Improve code style - [#96](https://github.com/ayrat555/frankenstein/pull/96)
- Update `typed-builder` requirement from 0.11 to 0.12 - [#95](https://github.com/ayrat555/frankenstein/pull/95)

## 0.22.0 (2022-12-31)

### [Bot API 6.4](https://core.telegram.org/bots/api#december-30-2022) - [#94](https://github.com/ayrat555/frankenstein/pull/94)

- Added the field `is_persistent` to the struct `ReplyKeyboardMarkup`, allowing to control when the keyboard is shown.
- Added the parameter `has_spoiler` to the methods `send_photo`, `send_video`, and `send_animation`.
- Added the field `has_spoiler` to the structs `InputMediaPhoto`, `InputMediaVideo`, and `InputMediaAnimation`.
- Added the field `has_media_spoiler` to the struct `Message`.
- The parameters `name` and `icon_custom_emoji_id` of the method `edit_forum_topic` are now optional. If they are omitted, the existing values are kept.
- Added the structs `ForumTopicEdited`, `GeneralForumTopicHidden`, `GeneralForumTopicUnhidden`, and `WriteAccessAllowed` and the fields `forum_topic_edited`, `general_forum_topic_hidden`, `general_forum_topic_unhidden`, and `write_access_allowed` to the struct `Message`.
- Added the methods `edit_general_forum_topic`, `close_general_forum_topic`, `reopen_general_forum_topic`, `hide_general_forum_topic`, `unhide_general_forum_topic` for managing the General topic in forums.
- Added the parameter `message_thread_id` to the method `send_chat_action` for sending chat actions to a specific message thread or a forum topic.
- Added the field `has_hidden_members` to the struct `Chat`. Note that the method `get_chat_member` is only guaranteed to work if the bot is an administrator in the chat.
- Added the field `has_aggressive_anti_spam_enabled` to the struct `Chat`.

## 0.21.0 (2022-10-06)

### [Bot API 6.3](https://core.telegram.org/bots/api#november-5-2022) - [#91](https://github.com/ayrat555/frankenstein/pull/91)

- Added the fields `is_topic_message` and `message_thread_id` to the struct `Message` to allow detection of messages belonging to a forum topic and their message thread identifier.
- Added the structs `ForumTopicCreated`, `ForumTopicClosed`, and `ForumTopicReopened` and the fields `forum_topic_created`, `forum_topic_closed`, and `forum_topic_reopened` to the struct `Message`
- Added the field `can_manage_topics` to the structs `ChatAdministratorRights`, `ChatPermissions`, `ChatMemberAdministrator`, and `ChatMemberRestricted`.
- Added the parameter `can_manage_topics` to the struct `PromoteChatMemberParams`.
- Added the methods `create_forum_topic`, `edit_forum_topic`, `close_forum_topic`, `reopen_forum_topic`, `delete_forum_topic`, `unpin_all_forum_topic_messages`, and `get_forum_topic_icon_stickers` for forum topic management.
- Added the field `message_thread_id` to the methods `SendMessageParams`, `SendPhotoParams`, `SendVideoParams`, `SendAnimationParams`, `SendAudioParams`, `SendDocumentParams`, `SendStickerParams`, `SendVideoNoteParams`, `SendVoiceParams`, `SendLocationParams`, `SendVenueParams`, `SendContactParams`, `SendPollParams`, `SendDiceParams`, `SendInvoiceParams`, `SendGameParams`, `SendMediaGroupParams`, `CopyMessageParams`, `ForwardMessageParams` to support sending of messages to a forum topic.
- Added support for Multiple Usernames via the field `active_usernames` in the struct `Chat`.
- Added the field `emoji_status_custom_emoji_id` to the struct `Chat`.

## 0.20.0 (2022-08-13)

- Change the `offset` field's type for `getUpdates` - [#86](https://github.com/ayrat555/frankenstein/pull/86)
- Change `connect_timeout` for async client - [#83](https://github.com/ayrat555/frankenstein/pull/83)

### [Bot API 6.2](https://core.telegram.org/bots/api#august-12-2022) - [#85](https://github.com/ayrat555/frankenstein/pull/85)

- Added the `MessageEntity` type `custom_emoji`.
- Added the field `custom_emoji_id` to the class `MessageEntity` for `custom_emoji` entities.
- Added the method `getCustomEmojiStickers`.
- Added the fields type and `custom_emoji_id` to the class `Sticker`.
- Added the field `sticker_type` to the class `StickerSet`, describing the type of stickers in the set.
- The field `contains_masks` has been removed from the documentation of the class `StickerSet`. The field is still returned on the object for backward compatibility, but new bots should use the field `sticker_type` instead.
- Added the parameter `sticker_type` to the method `createNewStickerSet`.
- The parameter `contains_masks` has been removed from the documentation of the method `createNewStickerSet`. The parameter will still work for backward compatibility, but new bots should use the parameter `sticker_type` instead.

## 0.19.2 (2022-07-20)

- Fix: Use client timeout of 500 seconds by @EdJoPaTo in [#82](https://github.com/ayrat555/frankenstein/pull/82)
- Chore: build with all features on docs.rs by @OpenByteDev in [#81](https://github.com/ayrat555/frankenstein/pull/81)

## 0.19.1 (2022-07-09)

- Fix status field of banned chat members [#78](https://github.com/ayrat555/frankenstein/pull/78)

## 0.19.0 (2022-07-06)

- Add builders for API clients [#77](https://github.com/ayrat555/frankenstein/pull/77)

## 0.18.0 (2022-06-21)

### [Bot API 6.1](https://core.telegram.org/bots/api-changelog#june-20-2022) - [#73](https://github.com/ayrat555/frankenstein/pull/73)

- Added the fields `join_to_send_messages` and `join_by_request` to the struct `Chat`.
- Added the method `create_invoice_link` to generate an HTTP link for an invoice
- Added the field `is_premium` to the struct `User`.
- Added the field `premium_animation` to the struct `Sticker`.
- Added the field `added_to_attachment_menu` to the struct `User`.
- Added the parameter `secret_token` to the method `set_webhook`.

## 0.17.0 (2022-06-16)

- Change type of `file_size` from `u32` to `u64` by @ayrat555 in [#70](https://github.com/ayrat555/frankenstein/pull/70)
- Refactor: specify `Eq` when possible on `PartialEq` by @EdJoPaTo in [#71](https://github.com/ayrat555/frankenstein/pull/71)
- Refactor: adopt breaking lints by @EdJoPaTo in [#72](https://github.com/ayrat555/frankenstein/pull/72)

## 0.16.0 (2022-06-07)

- Reduce `Message` struct memory size [#69](https://github.com/ayrat555/frankenstein/pull/69)

## 0.15.1 (2022-06-02)

- Add `new_with_client` to `AsyncApi` [#66](https://github.com/ayrat555/frankenstein/pull/66)

## 0.15.0 (2022-05-11)

- Convert allowed_update type into enum - [#65](https://github.com/ayrat555/frankenstein/pull/65)

## 0.14.0 (2022-05-09)

- Optimize Update struct by migrating its content to enum - [#62](https://github.com/ayrat555/frankenstein/pull/62)

## 0.13.0 (2022-04-18)

### [Bot API 6.0](https://core.telegram.org/bots/api#april-16-2022) - [#58](https://github.com/ayrat555/frankenstein/pull/58), [#59](https://github.com/ayrat555/frankenstein/pull/59)

- Added support for Web Apps, see the [detailed manual here](https://core.telegram.org/bots/webapps). ([blog announcement](https://telegram.org/blog/notifications-bots))
- Added the class [`WebAppInfo`](https://core.telegram.org/bots/api#webappinfo) and the fields web_app to the classes [`KeyboardButton`](https://core.telegram.org/bots/api#keyboardbutton) and [`InlineKeyboardButton`](https://core.telegram.org/bots/api#inlinekeyboardbutton).
- Added the class [`SentWebAppMessage`](https://core.telegram.org/bots/api#sentwebappmessage) and the method [`answerWebAppQuery`](https://core.telegram.org/bots/api#answerwebappquery) for sending an answer to a Web App query, which originated from an inline button of the 'web_app' type.
- Added the class [`WebAppData`](https://core.telegram.org/bots/api#webappdata) and the field web_app_data to the class [Message](https://core.telegram.org/bots/api#message).
- Added the class [`MenuButton`](https://core.telegram.org/bots/api#menubutton) and the methods [`setChatMenuButton`](https://core.telegram.org/bots/api#setchatmenubutton) and [`getChatMenuButton`](https://core.telegram.org/bots/api#getchatmenubutton) for managing the behavior of the bot's menu button in private chats.
- Added the class [`ChatAdministratorRights`](https://core.telegram.org/bots/api#chatadministratorrights) and the methods [`setMyDefaultAdministratorRights`](https://core.telegram.org/bots/api#setmydefaultadministratorrights) and [`getMyDefaultAdministratorRights`](https://core.telegram.org/bots/api#getmydefaultadministratorrights) for managing the bot's default administrator rights.
- Added support for t.me links that can be used to add the bot to groups and channels as an administrator.
- Added the field `last_synchronization_error_date` to the class [`WebhookInfo`](https://core.telegram.org/bots/api#webhookinfo).
- Renamed the field `can_manage_voice_chats` to `can_manage_video_chats` in the class [`ChatMemberAdministrator`](https://core.telegram.org/bots/api#chatmemberadministrator). The old field will remain temporarily available.
- Renamed the parameter `can_manage_voice_chats` to `can_manage_video_chats` in the method [`promoteChatMember`](https://core.telegram.org/bots/api#promotechatmember). The old parameter will remain temporarily available.
- Renamed the fields `voice_chat_scheduled`, `voice_chat_started`, `voice_chat_ended`, and `voice_chat_participants_invited` to `video_chat_scheduled`, `video_chat_started`, `video_chat_ended`, and `video_chat_participants_invited` in the class [`Message`](https://core.telegram.org/bots/api#message). The old fields will remain temporarily available.

## 0.12.0 (2022-03-20)

- Switch from `derive_builder` to `typed-builder` - [#53](https://github.com/ayrat555/frankenstein/pull/53)

## 0.11.0 (2022-03-19)

- Add `ParseMode` enum - [#49](https://github.com/ayrat555/frankenstein/pull/49)
- Derive Copy when possible - [#50](https://github.com/ayrat555/frankenstein/pull/50)
- Improve error handling - [#51](https://github.com/ayrat555/frankenstein/pull/51)

## 0.10.1 (2022-02-08)

- Remove `serde_json` from trait features - [#43](https://github.com/ayrat555/frankenstein/pull/43)

## 0.10.0 (2022-02-08)

- Add optional async API - [#38](https://github.com/ayrat555/frankenstein/pull/38)

## 0.9.5 (2022-02-02)

- Disable unused features in the multipart crate - [#41](https://github.com/ayrat555/frankenstein/pull/41)

## 0.9.4 (2022-02-01)

- Bot API 5.7 Changes - [#40](https://github.com/ayrat555/frankenstein/pull/40)

## 0.9.3 (2022-01-15)

- Configure request timeout - [#39](https://github.com/ayrat555/frankenstein/pull/39)
- Use `ureq`'s agent to make requests - [#39](https://github.com/ayrat555/frankenstein/pull/39)

## 0.9.2 (2022-01-01)

- [Telegram Bot API 5.6 Changes](https://core.telegram.org/bots/api#december-30-2021) - [#36](https://github.com/ayrat555/frankenstein/pull/36)

## 0.9.1 (2021-12-24)

- Add From trait impl for File enum - [#35](https://github.com/ayrat555/frankenstein/pull/35)

## 0.9.0 (2021-12-13)

- Use builders for all structs - [#34](https://github.com/ayrat555/frankenstein/pull/34)

## 0.8.1 (2021-12-08)

- [Telegram Bot API 5.5 Changes](https://core.telegram.org/bots/api#december-7-2021) - [#33](https://github.com/ayrat555/frankenstein/pull/33)

## 0.8.0 (2021-12-05)

### [#32](https://github.com/ayrat555/frankenstein/pull/32)

- fix `InputMessageContent` enum
- improve error handling

## 0.7.1 (2021-11-08)

- Rename impl feature to `http-client` - [#30](https://github.com/ayrat555/frankenstein/pull/30)
- Improve CI - [#31](https://github.com/ayrat555/frankenstein/pull/31)

## 0.7.0 (2021-11-06)

- [Telegram API 5.4 Changes](https://core.telegram.org/bots/api#november-5-2021) - [#27](https://github.com/ayrat555/frankenstein/pull/27)
- Update `ureq` to 2.3 - [#27](https://github.com/ayrat555/frankenstein/pull/27)

## 0.6.0 (2021-08-26)

- Use enums instead of strings for type fields - [#25](https://github.com/ayrat555/frankenstein/pull/25)

## 0.5.3 (2021-08-14)

- Make `Api` struct cloneable - [#23](https://github.com/ayrat555/frankenstein/pull/23)

## 0.5.2 (2021-07-10)

- Make `SendChatActionParams.action` an enum - [#22](https://github.com/ayrat555/frankenstein/pull/22)

## 0.5.1 (2021-07-06)

- Fix `ReplyKeyboardMarkup` serialization error - [#20](https://github.com/ayrat555/frankenstein/pull/20)

## 0.5.0 (2021-06-26)

### [Telegram API 5.3 Changes](https://core.telegram.org/bots/api#june-25-2021)

- add `BotCommandScope` - [#b937a5442d](https://github.com/ayrat555/frankenstein/commit/b937a5442dfd7f756593e02c897130ed681fc4f6)
- add `input_field_placeholder` - [#371fe3463](https://github.com/ayrat555/frankenstein/commit/371fe3463c44e40c4999e8ab7224014b903821d5)
- split `ChatMember` struct into 6 structs - [#8b104155a8](https://github.com/ayrat555/frankenstein/commit/8b104155a84220611d0e7666de5f63d630902d5c)
- rename `kickChatMember` and `getChatMembersCount` - [#35819f8843a88c](https://github.com/ayrat555/frankenstein/commit/35819f8843a88ccf2cd56f098082565274afb131)

### Other Changes

- Move type field into enums - [#19](https://github.com/ayrat555/frankenstein/pull/19)

## 0.4.0 (2021-06-13)

- Feature: Get rid of `isize`/`usize` types ([#11](https://github.com/ayrat555/frankenstein/pull/11), [#16](https://github.com/ayrat555/frankenstein/pull/16), [ab72b4469b3ed](https://github.com/ayrat555/frankenstein/commit/ab72b4469b3edf89d84f07ce17770605aa9068b2))
- Chore: Fix `clippy::pedantic` warnings ([#12](https://github.com/ayrat555/frankenstein/pull/12), [#14](https://github.com/ayrat555/frankenstein/pull/14))
- Chore: remove `Enum` postfix from enums ([#13](https://github.com/ayrat555/frankenstein/pull/13))
- Chore: bump `multipart` from 0.17 to 0.18 ([#15](https://github.com/ayrat555/frankenstein/pull/15))

## 0.3.1 (2021-05-15)

- Make `serde_json` optional dependency ([#6](https://github.com/ayrat555/frankenstein/pull/6))

## 0.3.0 (2021-05-09)

- Create `TelegramApi` trait ([#5](https://github.com/ayrat555/frankenstein/pull/5))
- Fix `InlineKeyboardMarkup` type ([7392c1dac9a42fbb66e05dc905b79ff849be289b](https://github.com/ayrat555/frankenstein/commit/7392c1dac9a42fbb66e05dc905b79ff849be289b))

## 0.2.1 (2021-05-02)

- Make all struct fields public ([0798a34dddcbce1030a2f20b478e455b8f289623](https://github.com/ayrat555/frankenstein/commit/0798a34dddcbce1030a2f20b478e455b8f289623))

## 0.2.0 (2021-05-01)

- The first release on crates.io
