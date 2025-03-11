//! Structs for handling and uploading files

use serde::{Deserialize, Serialize};

/// Represents a new file to be uploaded via `multipart/form-data`.
///
/// See <https://core.telegram.org/bots/api#inputfile>.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum InputFile {
    Bytes {
        bytes: Vec<u8>,
        file_name: String,
    },
    #[cfg(not(target_arch = "wasm32"))]
    Path(std::path::PathBuf),
}

#[cfg(not(target_arch = "wasm32"))]
impl From<std::path::PathBuf> for InputFile {
    fn from(value: std::path::PathBuf) -> Self {
        Self::Path(value)
    }
}

/// Represents different approaches of sending files.
///
/// See <https://core.telegram.org/bots/api#sending-files>.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum FileUpload {
    /// `file_id` to send a file that exists on the Telegram servers (recommended) or pass an HTTP URL for Telegram to get a file from the Internet
    String(String),
    /// upload a new file using `multipart/form-data`
    InputFile(InputFile),
}

impl From<String> for FileUpload {
    fn from(file: String) -> Self {
        Self::String(file)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<std::path::PathBuf> for FileUpload {
    fn from(path: std::path::PathBuf) -> Self {
        Self::InputFile(InputFile::Path(path))
    }
}

impl From<InputFile> for FileUpload {
    fn from(file: InputFile) -> Self {
        Self::InputFile(file)
    }
}

#[cfg(any(feature = "trait-sync", feature = "trait-async"))]
pub(crate) trait HasInputFile {
    // TODO: replace with `replace_attach` when possible
    fn get_input_file_ref(&self) -> Option<&InputFile>;
    fn replace_attach(&mut self, name: &str) -> Option<InputFile>;
    fn replace_attach_dyn(&mut self, index: impl FnOnce() -> usize) -> Option<(String, InputFile)>;
}

#[cfg(any(feature = "trait-sync", feature = "trait-async"))]
impl HasInputFile for FileUpload {
    fn get_input_file_ref(&self) -> Option<&InputFile> {
        match self {
            Self::InputFile(input_file) => Some(input_file),
            Self::String(_) => None,
        }
    }

    fn replace_attach(&mut self, name: &str) -> Option<InputFile> {
        match self {
            Self::InputFile(_) => {
                let attach = Self::String(format!("attach://{name}"));
                let Self::InputFile(file) = std::mem::replace(self, attach) else {
                    unreachable!("the match already ensures it being an input file");
                };
                Some(file)
            }
            Self::String(_) => None,
        }
    }

    fn replace_attach_dyn(&mut self, index: impl FnOnce() -> usize) -> Option<(String, InputFile)> {
        match self {
            Self::InputFile(_) => {
                let name = format!("file{}", index());
                let attach = Self::String(format!("attach://{name}"));
                let Self::InputFile(file) = std::mem::replace(self, attach) else {
                    unreachable!("the match already ensures it being an input file");
                };
                Some((name, file))
            }
            Self::String(_) => None,
        }
    }
}

#[cfg(any(feature = "trait-sync", feature = "trait-async"))]
impl HasInputFile for Option<FileUpload> {
    fn get_input_file_ref(&self) -> Option<&InputFile> {
        match self {
            Some(FileUpload::InputFile(input_file)) => Some(input_file),
            _ => None,
        }
    }

    fn replace_attach(&mut self, name: &str) -> Option<InputFile> {
        match self {
            Some(FileUpload::InputFile(_)) => {
                let attach = Some(FileUpload::String(format!("attach://{name}")));
                let Some(FileUpload::InputFile(file)) = std::mem::replace(self, attach) else {
                    unreachable!("the match already ensures it being an input file");
                };
                Some(file)
            }
            _ => None,
        }
    }

    fn replace_attach_dyn(&mut self, index: impl FnOnce() -> usize) -> Option<(String, InputFile)> {
        match self {
            Some(FileUpload::InputFile(_)) => {
                let name = format!("file{}", index());
                let attach = Some(FileUpload::String(format!("attach://{name}")));
                let Some(FileUpload::InputFile(file)) = std::mem::replace(self, attach) else {
                    unreachable!("the match already ensures it being an input file");
                };
                Some((name, file))
            }
            _ => None,
        }
    }
}
