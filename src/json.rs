#![cfg_attr(
    not(any(feature = "http-client", feature = "async-http-client")),
    allow(dead_code)
)]

use crate::Error;

/// Shortcut for [`serde_json::from_str`] with [`crate::Error`].
pub fn decode<T>(string: &str) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_str(string).map_err(|error| Error::Decode(format!("{error:?} : {string:?}")))
}

/// Shortcut for [`serde_json::to_string`] with [`crate::Error`].
pub fn encode<T>(value: &T) -> Result<String, Error>
where
    T: serde::ser::Serialize + std::fmt::Debug,
{
    serde_json::to_string(value).map_err(|error| Error::Encode(format!("{error:?} : {value:?}")))
}

#[cfg(test)]
#[track_caller]
pub fn assert_str<T>(value: &T, expected: &str)
where
    T: serde::ser::Serialize,
{
    let actual = serde_json::to_string(value).expect("Should be able to stringify to JSON");
    assert!(
        expected == actual,
        "value should equal the expected JSON\n expected: {expected}\n   actual: {actual}"
    );
}

#[test]
fn assert_str_works() {
    let value = serde_json::json!({"code": 42});
    let expected = r#"{"code":42}"#;
    assert_str(&value, expected);
}

#[test]
#[should_panic = "value should equal the expected JSON"]
fn assert_str_fails() {
    let value = serde_json::json!({"code": 42});
    let expected = r#"{"foo":"bar"}"#;
    assert_str(&value, expected);
}
