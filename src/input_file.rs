//! Structs for handling and uploading files

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Represents a new file to be uploaded via `multipart/form-data`.
///
/// See <https://core.telegram.org/bots/api#inputfile>.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct InputFile {
    pub path: PathBuf,
}

impl From<PathBuf> for InputFile {
    fn from(path: PathBuf) -> Self {
        Self { path }
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

impl From<PathBuf> for FileUpload {
    fn from(path: PathBuf) -> Self {
        Self::InputFile(InputFile { path })
    }
}

impl From<InputFile> for FileUpload {
    fn from(file: InputFile) -> Self {
        Self::InputFile(file)
    }
}

#[cfg(any(feature = "trait-sync", feature = "trait-async"))]
pub(crate) trait HasInputFile {
    fn clone_path(&self) -> Option<PathBuf>;
    fn replace_attach(&mut self, name: &str) -> Option<PathBuf>;
    fn replace_attach_dyn(&mut self, index: impl FnOnce() -> usize) -> Option<(String, PathBuf)>;
}

#[cfg(any(feature = "trait-sync", feature = "trait-async"))]
impl HasInputFile for FileUpload {
    fn clone_path(&self) -> Option<PathBuf> {
        match self {
            Self::InputFile(input_file) => Some(input_file.path.clone()),
            Self::String(_) => None,
        }
    }

    fn replace_attach(&mut self, name: &str) -> Option<PathBuf> {
        match self {
            Self::InputFile(_) => {
                let attach = Self::String(format!("attach://{name}"));
                let Self::InputFile(file) = std::mem::replace(self, attach) else {
                    unreachable!("the match already ensures it being an input file");
                };
                Some(file.path)
            }
            Self::String(_) => None,
        }
    }

    fn replace_attach_dyn(&mut self, index: impl FnOnce() -> usize) -> Option<(String, PathBuf)> {
        match self {
            Self::InputFile(_) => {
                let name = format!("file{}", index());
                let attach = Self::String(format!("attach://{name}"));
                let Self::InputFile(file) = std::mem::replace(self, attach) else {
                    unreachable!("the match already ensures it being an input file");
                };
                Some((name, file.path))
            }
            Self::String(_) => None,
        }
    }
}

#[cfg(any(feature = "trait-sync", feature = "trait-async"))]
impl HasInputFile for Option<FileUpload> {
    fn clone_path(&self) -> Option<PathBuf> {
        match self {
            Some(FileUpload::InputFile(input_file)) => Some(input_file.path.clone()),
            _ => None,
        }
    }

    fn replace_attach(&mut self, name: &str) -> Option<PathBuf> {
        match self {
            Some(FileUpload::InputFile(_)) => {
                let attach = Some(FileUpload::String(format!("attach://{name}")));
                let Some(FileUpload::InputFile(file)) = std::mem::replace(self, attach) else {
                    unreachable!("the match already ensures it being an input file");
                };
                Some(file.path)
            }
            _ => None,
        }
    }

    fn replace_attach_dyn(&mut self, index: impl FnOnce() -> usize) -> Option<(String, PathBuf)> {
        match self {
            Some(FileUpload::InputFile(_)) => {
                let name = format!("file{}", index());
                let attach = Some(FileUpload::String(format!("attach://{name}")));
                let Some(FileUpload::InputFile(file)) = std::mem::replace(self, attach) else {
                    unreachable!("the match already ensures it being an input file");
                };
                Some((name, file.path))
            }
            _ => None,
        }
    }
}
