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
