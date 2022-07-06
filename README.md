<p align="center"><img src="frankenstein_logo.png" alt="frankenstein" height="300px"></p>

[![Crates.io][s1]][ci] [![docs page][docs-badge]][docs] ![test][ga-test] ![style][ga-style]

# Frankenstein

Telegram bot API client for Rust.

It's a complete wrapper for Telegram bot API and it's up to date with version 6.1 of the API.

Frankenstein data structures (rust structs and enums) are mapped one-to-one from Telegram bot API objects and method params.

## Installation

Add this to your Cargo.toml


```toml
[dependencies]
frankenstein = "0.19"
```

## Features

### Default features

- `http-client` - a blocking HTTP client (uses `ureq`), it's the only default feature
- `telegram-trait` - a blocking API trait, it's included in the `http-client` feature. it may be useful for people who want to create a custom blocking client (for example, replacing an HTTP client)

### Optional features

- `async-http-client` - an async HTTP client, it uses `reqwest` and it's disabled by default
- `async-telegram-trait` - an async API trait, it's used in the `async-http-client`. it may be useful for people who want to create a custom async client

To use the async client add the following line to your `Cargo.toml` file:

```toml
frankenstein = { version = "0.19", default-features = false, features = ["async-http-client"] }
```

You can also disable all features:

```toml
frankenstein = { version = "0.19", default-features = false }
```

In this case the crate will ship only with telegram types


## Usage

Examples in this section use the blocking client (`frankenstein::Api`), but async examples would look the same (just replace `frankenstein::Api` with `frankenstein::AsyncApi`)

### Data structures

All objects described in the API docs have direct counterparts in the frankenstein. For example, in the docs there is [the user type](https://core.telegram.org/bots/api#user):
```
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

In frankenstein, it's described like this:

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
let send_message_params = SendMessageParams::builder()
    .chat_id(message.chat.id)
    .text("hello")
    .reply_to_message_id(message.message_id)
    .build();
```

For api parameters, the same approach is used. The only difference for parameters is the name of the struct in frankenstein ends with `Params` postfix.

For example, parameters for `leaveChat` method:

```rust
pub struct LeaveChatParams {
    chat_id: ChatId,
}
```


### Making requests

To make a request to the telegram bot api:

1. Initialize the `Api` struct:

```rust
use frankenstein::Api;
use frankenstein::TelegramApi;

...

let token = "My_token";
let api = Api::new(token);
```

2. Use this api object to make requests to the Bot API:

```rust
let update_params = GetUpdatesParams::builder()
    .allowed_updates(vec![AllowedUpdate::Message])
    .build();

let result = api.get_updates(&update_params);
```

Every function returns a `Result` enum with a successful response or failed response.

See a complete example in the `examples` directory.

### Uploading files

Some methods in the API allow uploading files. In the frankenstein for this `File` struct is used:

```rust
pub enum File {
    InputFile(InputFile),
    String(String),
}

pub struct InputFile {
    path: std::path::PathBuf
}
```

It has two variants:

- `File::String` is used to pass id of the already uploaded file
- `File::InputFile` is used to upload a new file using multipart upload.


### Customizing http clients

Both the async (`reqwest`) and the blocking (`ureq`) HTTP clients can be customized with their builders.

Customizing the blocking client:

```rust
use frankenstein::ureq;
use frankenstein::Api;
use std::time::Duration;

let request_agent = ureq::builder().timeout(Duration::from_secs(100)).build();
let api_url = format!("{}{}", BASE_API_URL, TOKEN);

Api::builder()
     .api_url(api_url)
     .request_agent(request_agent)
     .build()
```

Customizing the async client:

```rust
use frankenstein::reqwest;
use frankenstein::AsyncApi;
use std::time::Duration;

let client = reqwest::ClientBuilder::new()
    .connect_timeout(Duration::from_secs(100))
    .timeout(Duration::from_secs(100))
    .build()
    .unwrap();
let api_url = format!("{}{}", BASE_API_URL, TOKEN);

AsyncApi::builder().api_url(api_url).client(client).build()
```

### Documentation

Frankenstein implements all telegram bot api methods. To see which parameters you should pass, check [docs.rs](https://docs.rs/frankenstein/0.19.0/frankenstein/api_traits/telegram_api/trait.TelegramApi.html#provided-methods)

You can check out a real world bot created using this library - [El Monitorro](https://github.com/ayrat555/el_monitorro). El Monitorro is a feed reader bot.


## Replacing the default http client

The library uses `ureq` http client by default, but it can be easily replaced with any http client of your choice:

1. `ureq` comes with a default feature (`impl`). So the feature should be disabled:

```toml
frankenstein = { version = "0.19", default-features = false, features = ["telegram-trait"] }
```

2. Implement `TelegramApi` trait which requires two functions:

- `request_with_form_data` is used to upload files
- `request` is used for requests without file uploads

You can check [the default `TelegramApi` trait implementation](https://github.com/ayrat555/frankenstein/blob/aac88c01d06aa945393db7255ef2485a7c764d47/src/api_impl.rs) for `ureq`.

Also, you can take a look at the [implementation for `isahc` http client](https://github.com/ayrat555/frankenstein/blob/master/examples/api_trait_implementation.rs) in the examples directory.

Without the default ureq implementation, `frankenstein` has only one dependency - `serde`.

## Contributing

1. [Fork it!](https://github.com/ayrat555/frankenstein/fork)
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create new Pull Request

## Author

Ayrat Badykov (@ayrat555)


[s1]: https://img.shields.io/crates/v/frankenstein.svg
[docs-badge]: https://img.shields.io/badge/docs-website-blue.svg
[ci]: https://crates.io/crates/frankenstein
[docs]: https://docs.rs/frankenstein/
[ga-test]: https://github.com/ayrat555/frankenstein/actions/workflows/rust.yml/badge.svg
[ga-style]: https://github.com/ayrat555/frankenstein/actions/workflows/style.yml/badge.svg
