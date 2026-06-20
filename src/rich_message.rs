//! API Objects related to [Rich Messages](https://core.telegram.org/bots/api#rich-messages).

use serde::{Deserialize, Serialize};

use crate::macros::{apistruct, apply};
use crate::types::{Animation, Audio, Location, PhotoSize, User, Video, Voice};

#[apply(apistruct!)]
pub struct RichMessage {
    pub blocks: Vec<RichBlock>,
    pub is_rtl: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputRichMessage {
    pub html: Option<String>,
    pub markdown: Option<String>,
    pub is_rtl: Option<bool>,
    pub skip_entity_detection: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RichText {
    Text(String),
    List(Vec<Self>),
    Object(RichTextObject),
}

impl From<String> for RichText {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for RichText {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

impl From<Vec<Self>> for RichText {
    fn from(value: Vec<Self>) -> Self {
        Self::List(value)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichTextObject {
    Bold(RichTextBold),
    Italic(RichTextItalic),
    Underline(RichTextUnderline),
    Strikethrough(RichTextStrikethrough),
    Spoiler(RichTextSpoiler),
    DateTime(RichTextDateTime),
    TextMention(RichTextTextMention),
    Subscript(RichTextSubscript),
    Superscript(RichTextSuperscript),
    Marked(RichTextMarked),
    Code(RichTextCode),
    CustomEmoji(RichTextCustomEmoji),
    MathematicalExpression(RichTextMathematicalExpression),
    Url(RichTextUrl),
    EmailAddress(RichTextEmailAddress),
    PhoneNumber(RichTextPhoneNumber),
    BankCardNumber(RichTextBankCardNumber),
    Mention(RichTextMention),
    Hashtag(RichTextHashtag),
    Cashtag(RichTextCashtag),
    BotCommand(RichTextBotCommand),
    Anchor(RichTextAnchor),
    AnchorLink(RichTextAnchorLink),
    Reference(RichTextReference),
    ReferenceLink(RichTextReferenceLink),
}

macro_rules! rich_text_from {
    ($type:ident, $variant:ident) => {
        impl From<$type> for RichText {
            fn from(value: $type) -> Self {
                Self::Object(RichTextObject::$variant(value))
            }
        }
    };
}

rich_text_from!(RichTextBold, Bold);
rich_text_from!(RichTextItalic, Italic);
rich_text_from!(RichTextUnderline, Underline);
rich_text_from!(RichTextStrikethrough, Strikethrough);
rich_text_from!(RichTextSpoiler, Spoiler);
rich_text_from!(RichTextDateTime, DateTime);
rich_text_from!(RichTextTextMention, TextMention);
rich_text_from!(RichTextSubscript, Subscript);
rich_text_from!(RichTextSuperscript, Superscript);
rich_text_from!(RichTextMarked, Marked);
rich_text_from!(RichTextCode, Code);
rich_text_from!(RichTextCustomEmoji, CustomEmoji);
rich_text_from!(RichTextMathematicalExpression, MathematicalExpression);
rich_text_from!(RichTextUrl, Url);
rich_text_from!(RichTextEmailAddress, EmailAddress);
rich_text_from!(RichTextPhoneNumber, PhoneNumber);
rich_text_from!(RichTextBankCardNumber, BankCardNumber);
rich_text_from!(RichTextMention, Mention);
rich_text_from!(RichTextHashtag, Hashtag);
rich_text_from!(RichTextCashtag, Cashtag);
rich_text_from!(RichTextBotCommand, BotCommand);
rich_text_from!(RichTextAnchor, Anchor);
rich_text_from!(RichTextAnchorLink, AnchorLink);
rich_text_from!(RichTextReference, Reference);
rich_text_from!(RichTextReferenceLink, ReferenceLink);

macro_rules! rich_text_format_struct {
    ($type:ident) => {
        #[apply(apistruct!)]
        pub struct $type {
            pub text: Box<RichText>,
        }
    };
}

rich_text_format_struct!(RichTextBold);
rich_text_format_struct!(RichTextItalic);
rich_text_format_struct!(RichTextUnderline);
rich_text_format_struct!(RichTextStrikethrough);
rich_text_format_struct!(RichTextSpoiler);
rich_text_format_struct!(RichTextSubscript);
rich_text_format_struct!(RichTextSuperscript);
rich_text_format_struct!(RichTextMarked);
rich_text_format_struct!(RichTextCode);

#[apply(apistruct!)]
pub struct RichTextDateTime {
    pub text: Box<RichText>,
    pub unix_time: u64,
    pub date_time_format: String,
}

#[apply(apistruct!)]
pub struct RichTextTextMention {
    pub text: Box<RichText>,
    pub user: User,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct RichTextCustomEmoji {
    pub custom_emoji_id: String,
    pub alternative_text: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct RichTextMathematicalExpression {
    pub expression: String,
}

#[apply(apistruct!)]
pub struct RichTextUrl {
    pub text: Box<RichText>,
    pub url: String,
}

#[apply(apistruct!)]
pub struct RichTextEmailAddress {
    pub text: Box<RichText>,
    pub email_address: String,
}

#[apply(apistruct!)]
pub struct RichTextPhoneNumber {
    pub text: Box<RichText>,
    pub phone_number: String,
}

#[apply(apistruct!)]
pub struct RichTextBankCardNumber {
    pub text: Box<RichText>,
    pub bank_card_number: String,
}

#[apply(apistruct!)]
pub struct RichTextMention {
    pub text: Box<RichText>,
    pub username: String,
}

#[apply(apistruct!)]
pub struct RichTextHashtag {
    pub text: Box<RichText>,
    pub hashtag: String,
}

#[apply(apistruct!)]
pub struct RichTextCashtag {
    pub text: Box<RichText>,
    pub cashtag: String,
}

#[apply(apistruct!)]
pub struct RichTextBotCommand {
    pub text: Box<RichText>,
    pub bot_command: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct RichTextAnchor {
    pub name: String,
}

#[apply(apistruct!)]
pub struct RichTextAnchorLink {
    pub text: Box<RichText>,
    pub anchor_name: String,
}

#[apply(apistruct!)]
pub struct RichTextReference {
    pub text: Box<RichText>,
    pub name: String,
}

#[apply(apistruct!)]
pub struct RichTextReferenceLink {
    pub text: Box<RichText>,
    pub reference_name: String,
}

#[apply(apistruct!)]
pub struct RichBlockCaption {
    pub text: RichText,
    pub credit: Option<RichText>,
}

#[apply(apistruct!)]
pub struct RichBlockTableCell {
    pub text: Option<RichText>,
    pub is_header: Option<bool>,
    pub colspan: Option<u32>,
    pub rowspan: Option<u32>,
    pub align: String,
    pub valign: String,
}

#[apply(apistruct!)]
pub struct RichBlockListItem {
    pub label: String,
    pub blocks: Vec<RichBlock>,
    pub has_checkbox: Option<bool>,
    pub is_checked: Option<bool>,
    pub value: Option<i32>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichBlock {
    Paragraph(RichBlockParagraph),
    Heading(RichBlockSectionHeading),
    Pre(RichBlockPreformatted),
    Footer(RichBlockFooter),
    Divider(RichBlockDivider),
    MathematicalExpression(RichBlockMathematicalExpression),
    Anchor(RichBlockAnchor),
    List(RichBlockList),
    Blockquote(RichBlockBlockQuotation),
    Pullquote(RichBlockPullQuotation),
    Collage(RichBlockCollage),
    Slideshow(RichBlockSlideshow),
    Table(RichBlockTable),
    Details(RichBlockDetails),
    Map(RichBlockMap),
    Animation(RichBlockAnimation),
    Audio(RichBlockAudio),
    Photo(RichBlockPhoto),
    Video(RichBlockVideo),
    VoiceNote(RichBlockVoiceNote),
    Thinking(RichBlockThinking),
}

#[apply(apistruct!)]
pub struct RichBlockParagraph {
    pub text: RichText,
}

#[apply(apistruct!)]
pub struct RichBlockSectionHeading {
    pub text: RichText,
    pub size: u8,
}

#[apply(apistruct!)]
pub struct RichBlockPreformatted {
    pub text: RichText,
    pub language: Option<String>,
}

#[apply(apistruct!)]
pub struct RichBlockFooter {
    pub text: RichText,
}

#[apply(apistruct!)]
#[derive(Copy, Eq)]
pub struct RichBlockDivider {}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct RichBlockMathematicalExpression {
    pub expression: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct RichBlockAnchor {
    pub name: String,
}

#[apply(apistruct!)]
pub struct RichBlockList {
    pub items: Vec<RichBlockListItem>,
}

#[apply(apistruct!)]
pub struct RichBlockBlockQuotation {
    pub blocks: Vec<RichBlock>,
    pub credit: Option<RichText>,
}

#[apply(apistruct!)]
pub struct RichBlockPullQuotation {
    pub text: RichText,
    pub credit: Option<RichText>,
}

#[apply(apistruct!)]
pub struct RichBlockCollage {
    pub blocks: Vec<RichBlock>,
    pub caption: Option<RichBlockCaption>,
}

#[apply(apistruct!)]
pub struct RichBlockSlideshow {
    pub blocks: Vec<RichBlock>,
    pub caption: Option<RichBlockCaption>,
}

#[apply(apistruct!)]
pub struct RichBlockTable {
    pub cells: Vec<Vec<RichBlockTableCell>>,
    pub is_bordered: Option<bool>,
    pub is_striped: Option<bool>,
    pub caption: Option<RichText>,
}

#[apply(apistruct!)]
pub struct RichBlockDetails {
    pub summary: RichText,
    pub blocks: Vec<RichBlock>,
    pub is_open: Option<bool>,
}

#[apply(apistruct!)]
pub struct RichBlockMap {
    pub location: Location,
    pub zoom: u8,
    pub width: u32,
    pub height: u32,
    pub caption: Option<RichBlockCaption>,
}

#[apply(apistruct!)]
pub struct RichBlockAnimation {
    pub animation: Animation,
    pub has_spoiler: Option<bool>,
    pub caption: Option<RichBlockCaption>,
}

#[apply(apistruct!)]
pub struct RichBlockAudio {
    pub audio: Audio,
    pub caption: Option<RichBlockCaption>,
}

#[apply(apistruct!)]
pub struct RichBlockPhoto {
    pub photo: Vec<PhotoSize>,
    pub has_spoiler: Option<bool>,
    pub caption: Option<RichBlockCaption>,
}

#[apply(apistruct!)]
pub struct RichBlockVideo {
    pub video: Video,
    pub has_spoiler: Option<bool>,
    pub caption: Option<RichBlockCaption>,
}

#[apply(apistruct!)]
pub struct RichBlockVoiceNote {
    pub voice_note: Voice,
    pub caption: Option<RichBlockCaption>,
}

#[apply(apistruct!)]
pub struct RichBlockThinking {
    pub text: RichText,
}
