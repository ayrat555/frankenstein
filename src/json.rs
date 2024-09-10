use crate::Error;

pub fn decode<T>(string: &str) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_str(string).map_err(|error| Error::Decode(format!("{error:?} : {string:?}")))
}

pub fn encode<T>(value: &T) -> Result<String, Error>
where
    T: serde::ser::Serialize + std::fmt::Debug,
{
    serde_json::to_string(value).map_err(|error| Error::Encode(format!("{error:?} : {value:?}")))
}
