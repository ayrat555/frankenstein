//! Structs for handling and uploading files

use std::path::Path;

use serde::{Deserialize, Serialize};

/// Represents a new file to be uploaded via `multipart/form-data`.
///
/// See <https://core.telegram.org/bots/api#inputfile>.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct InputFile {
    pub bytes: Vec<u8>,
    pub file_name: String,
}

impl InputFile {
    pub fn read_std<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path = path.as_ref();
        let bytes = std::fs::read(path)?;
        let file_name = Self::file_name_from_path(path)?;
        Ok(Self { bytes, file_name })
    }

    #[cfg(feature = "inputfile-read-tokio")]
    pub async fn read_tokio_fs<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path = path.as_ref();
        let bytes = tokio::fs::read(path).await?;
        let file_name = Self::file_name_from_path(path)?;
        Ok(Self { bytes, file_name })
    }

    /// This method is intended to be used after `fs` operations
    fn file_name_from_path(path: &Path) -> std::io::Result<String> {
        let file_name = path
            .file_name()
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "A file that could be read should also have a name",
                )
            })?
            .to_string_lossy()
            .to_string();
        Ok(file_name)
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
