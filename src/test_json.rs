#[track_caller]
pub fn assert_json_str<T>(value: &T, expected: &str)
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
    assert_json_str(&value, expected);
}

#[test]
#[should_panic = "value should equal the expected JSON"]
fn assert_str_fails() {
    let value = serde_json::json!({"code": 42});
    let expected = r#"{"foo":"bar"}"#;
    assert_json_str(&value, expected);
}
