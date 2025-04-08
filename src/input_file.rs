//! Structs for handling and uploading files

use std::borrow::Cow;

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

type Filelist = Vec<(Cow<'static, str>, InputFile)>;

#[cfg(any(feature = "trait-sync", feature = "trait-async"))]
pub(crate) trait HasInputFile {
    fn move_named_to_filelist(&mut self, name: &'static str, files: &mut Filelist);
    fn move_to_filelist(&mut self, files: &mut Filelist);
}

#[cfg(any(feature = "trait-sync", feature = "trait-async"))]
impl HasInputFile for FileUpload {
    fn move_named_to_filelist(&mut self, name: &'static str, files: &mut Filelist) {
        if let Self::InputFile(_) = self {
            let attach = Self::String(format!("attach://{name}"));
            let Self::InputFile(file) = std::mem::replace(self, attach) else {
                unreachable!("the match already ensures it being an input file");
            };
            files.push((Cow::Borrowed(name), file));
        }
    }

    fn move_to_filelist(&mut self, files: &mut Filelist) {
        if let Self::InputFile(_) = self {
            let name = format!("file{}", files.len());
            let attach = Self::String(format!("attach://{name}"));
            let Self::InputFile(file) = std::mem::replace(self, attach) else {
                unreachable!("the match already ensures it being an input file");
            };
            files.push((Cow::Owned(name), file));
        }
    }
}

#[cfg(any(feature = "trait-sync", feature = "trait-async"))]
impl HasInputFile for Option<FileUpload> {
    fn move_named_to_filelist(&mut self, name: &'static str, files: &mut Filelist) {
        if let Some(FileUpload::InputFile(_)) = self {
            let attach = Some(FileUpload::String(format!("attach://{name}")));
            let Some(FileUpload::InputFile(file)) = std::mem::replace(self, attach) else {
                unreachable!("the match already ensures it being an input file");
            };
            files.push((Cow::Borrowed(name), file));
        }
    }

    fn move_to_filelist(&mut self, files: &mut Filelist) {
        if let Some(FileUpload::InputFile(_)) = self {
            let name = format!("file{}", files.len());
            let attach = Some(FileUpload::String(format!("attach://{name}")));
            let Some(FileUpload::InputFile(file)) = std::mem::replace(self, attach) else {
                unreachable!("the match already ensures it being an input file");
            };
            files.push((Cow::Owned(name), file));
        }
    }
}
