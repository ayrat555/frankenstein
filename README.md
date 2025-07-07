<p align="center"><img src="frankenstein_logo.png" alt="frankenstein" height="300px"></p>

# Frankenstein

Telegram bot API client for Rust.

It's a complete wrapper for Telegram bot API, and it's up-to-date with version 9.0 of the API.

Frankenstein's data structures (Rust structs and enums) are mapped one-to-one from Telegram bot API types and method parameters.

## Installation

Run `cargo add frankenstein` or add the following to your `Cargo.toml`.

```toml
[dependencies]
frankenstein = { version = "0.43", features = [] }
```

You likely want to use either a blocking or an async client. Enable it via the [Features](#features).

## Features

Without enabling any additional features this crate will only ship with Telegram types.

- blocking (synchronous)
  - `client-ureq` - a blocking HTTP API client based on `ureq`
  - `trait-sync` - a blocking API trait, it's included in the `client-ureq` feature. It may be useful for people who want to create a custom blocking client (for example, replacing an HTTP client)
- async
  - `client-reqwest` - an async HTTP API client based on `reqwest`. This client partially supports wasm32, but file uploads are currently not supported there.
  - `trait-async` - an async API trait, it's used in the `client-reqwest`. It may be useful for people who want to create a custom async client

For example for the async client add the following line to your `Cargo.toml` file:

```toml
frankenstein = { version = "0.43", features = ["client-reqwest"] }
```

## Usage

Examples in this section use the blocking client (`frankenstein::Api`), but async examples would look the same (just replace `frankenstein::Api` with `frankenstein::AsyncApi`)

### Data structures

All types described in the API docs have direct counterparts in the Frankenstein. For example, in the docs there is [the user type](https://core.telegram.org/bots/api#user):

```plaintext
id	Integer	Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
is_bot	Boolean	True, if this user is a bot
first_name	String	User's or bot's first name
last_name	String	Optional. User's or bot's last name
username	String	Optional. User's or bot's username
language_code	String	Optional. IETF language tag of the user's language
can_join_groups	Boolean	Optional. True, if the bot can be invited to groups. Returned only in getMe.
can_read_all_group_messages	Boolean	Optional. True, if privacy mode is disabled for the bot. Returned only in getMe.
supports_inline_queries	Boolean	Optional. True, if the bot supports inline queries. Returned only in getMe.
```

In Frankenstein, it's described like this:

```rust
pub struct User {
    pub id: u64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub can_join_groups: Option<bool>,
    pub can_read_all_group_messages: Option<bool>,
    pub supports_inline_queries: Option<bool>,
}
```

Optional fields are described as `Option`.

Every struct can be created with the associated builder. Only required fields are required to set, optional fields are set to `None` when not provided:

```rust
use frankenstein::methods::SendMessageParams;
let send_message_params = SendMessageParams::builder()
    .chat_id(1337)
    .text("hello")
    .build();
```

### Making requests

```rust,no_run
#![cfg(feature = "client-ureq")]
use frankenstein::TelegramApi;
use frankenstein::client_ureq::Bot;
use frankenstein::methods::{GetUpdatesParams, SendMessageParams};
use frankenstein::types::AllowedUpdate;

let token = "123:ABC";
let bot = Bot::new(token);

// Send a message
let send_message_params = SendMessageParams::builder()
    .chat_id(1337)
    .text("hello")
    .build();
let result = bot.send_message(&send_message_params);

// or get the updates (= interactions with the bot)
let update_params = GetUpdatesParams::builder()
    .allowed_updates(vec![AllowedUpdate::Message])
    .build();
let result = bot.get_updates(&update_params);
```

Every function returns a `Result` with a successful response or failed response.

See more examples in the [`examples`](https://github.com/ayrat555/frankenstein/tree/0.40.2/examples) directory.

### Uploading files

Some methods in the API allow uploading files. In the Frankenstein for this `FileUpload` enum is used:

```rust
pub enum FileUpload {
    InputFile(InputFile),
    String(String),
}

pub struct InputFile {
    path: std::path::PathBuf
}
```

It has two variants:

- `FileUpload::String` is used to pass the ID of the already uploaded file
- `FileUpload::InputFile` is used to upload a new file using multipart upload.

### Documentation

Frankenstein implements all Telegram bot API methods. To see which parameters you should pass, check the [official Telegram Bot API documentation](https://core.telegram.org/bots/api#available-methods) or [docs.rs/frankenstein](https://docs.rs/frankenstein/0.43.0/frankenstein/trait.TelegramApi.html#provided-methods)

You can check out real-world bots created using this library:

- [El Monitorro](https://github.com/ayrat555/el_monitorro) - RSS/Atom/JSON feed reader.
- [subvt-telegram-bot](https://github.com/helikon-labs/subvt-backend/tree/main/subvt-telegram-bot) - A Telegram bot for the validators of the [Polkadot](https://polkadot.network/) and [Kusama](https://kusama.network/).
- [wdr-maus-downloader](https://github.com/EdJoPaTo/wdr-maus-downloader) - checks for a new episode of the WDR Maus and downloads it.
- [weather_bot_rust](https://github.com/pxp9/weather_bot_rust) - A Telegram bot that provides weather info around the world.

## Contributing

1. [Fork it!](https://github.com/ayrat555/frankenstein/fork)
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create new Pull Request

## Author

Ayrat Badykov (@ayrat555)
