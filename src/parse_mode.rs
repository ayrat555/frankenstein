#![allow(deprecated)]

use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParseMode {
    #[serde(rename = "HTML")]
    Html,

    MarkdownV2,

    #[deprecated = "This is a legacy mode, retained for backward compatibility. Use `MarkdownV2` instead."]
    Markdown,
}

impl FromStr for ParseMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTML" | "Html" | "html" => Ok(Self::Html),
            "Markdown" | "markdown" => Ok(Self::Markdown),
            "MarkdownV2" | "markdownv2" => Ok(Self::MarkdownV2),
            _ => Err(()),
        }
    }
}

impl ParseMode {
    #[must_use]
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Html => "HTML",
            Self::MarkdownV2 => "MarkdownV2",
            Self::Markdown => "Markdown",
        }
    }
}

impl Display for ParseMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[test]
fn serde_markdown_works() {
    let json = serde_json::to_string(&ParseMode::MarkdownV2).unwrap();
    assert_eq!(json, r#""MarkdownV2""#);
}

#[test]
fn serde_html_works() {
    let json = serde_json::to_string(&ParseMode::Html).unwrap();
    assert_eq!(json, r#""HTML""#);
}
