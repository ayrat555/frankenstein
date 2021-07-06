<p align="center"><img src="frankenstein_logo.png" alt="frankenstein" height="300px"></p>

[![Crates.io][s1]][ci] [![docs page][docs-badge]][docs] ![test][ga-test] ![style][ga-style]

# Frankenstein

Telegram bot API client for Rust.

It's a complete wrapper for Telegram bot API and it's up to date with version 5.3 of the API.

Frankenstein data structures (rust structs and enums) are mapped one-to-one from Telegram bot API objects and method params.

## Installation

Add this to your Cargo.toml


```toml
[dependencies]
frankenstein = "0.5"
```

## Usage

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

In frankenstein, it's described as:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: i64,

    pub is_bot: bool,

    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_join_groups: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_all_group_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_inline_queries: Option<bool>,
}
```

Optional fields are described as Option enum.

Every struct has the `new` method which used for initialization. It accepts only required fields, optional fields are set to `None`:

```rust
pub fn new(id: i64, is_bot: bool, first_name: String) -> User {
    Self {
        id,
        is_bot,
        first_name,
        last_name: None,
        username: None,
        language_code: None,
        can_join_groups: None,
        can_read_all_group_messages: None,
        supports_inline_queries: None,
    }
}
```

All fields have setter and getter methods :

```rust
...

 pub fn set_supports_inline_queries(&mut self, supports_inline_queries: Option<bool>) {
     self.supports_inline_queries = supports_inline_queries;
 }
 pub fn id(&self) -> i64 {
     self.id
 }

...
```


For method parameters, the same approach is used. The only difference for parameters is the name of the struct in frankenstein ends with `Params` postfix.

For example, parameters for `leaveChat` method:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
let mut update_params = GetUpdatesParams::new();
update_params.set_allowed_updates(Some(vec!["message".to_string()]));

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


### Documentation

Frankenstein implements all telegram bot api methods. To see which parameters you should pass, check [docs.rs](https://docs.rs/frankenstein/0.5.1/frankenstein/api/trait.TelegramApi.html#provided-methods)

## Replacing the default http client

The library uses `ureq` http client by default, but it can be easily replaced with any http client of your choice:

1. `ureq` comes with a default feature (`impl`). So the feature should be disabled:

```toml
frankenstein = { version = "0.5", default-features = false }
```

2. Implement `TelegramApi` trait which requires two functions:

- `request_with_form_data` is used to upload files
- `request` is used for requests without file uploads

You can check [the default `TelegramApi` trait implementation](https://github.com/ayrat555/frankenstein/blob/aac88c01d06aa945393db7255ef2485a7c764d47/src/api_impl.rs) for `ureq`.

Also, you can take a look at the [implementation for `isahc` http client](https://github.com/ayrat555/frankenstein/blob/aac88c01d06aa945393db7255ef2485a7c764d47/examples/api_trait_implementation.rs) in the examples directory.

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
