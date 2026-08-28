//! API Objects related to [Rich Messages](https://core.telegram.org/bots/api#rich-messages).

use serde::{Deserialize, Serialize};

use crate::input_media::{
    InputMediaAnimation, InputMediaAudio, InputMediaDocument, InputMediaPhoto, InputMediaVideo,
    InputMediaVoiceNote,
};
use crate::macros::{apistruct, apply};
use crate::types::{
    Animation, Audio, CopyTextButton, DisabledButton, Document, Location, LoginUrl, PhotoSize,
    SwitchInlineQueryChosenChat, User, Video, Voice, WebAppInfo,
};

#[apply(apistruct!)]
pub struct RichMessage {
    pub blocks: Vec<RichBlock>,
    pub is_rtl: Option<bool>,
}

#[apply(apistruct!)]
pub struct InputRichMessage {
    pub blocks: Option<Vec<InputRichBlock>>,
    pub html: Option<String>,
    pub markdown: Option<String>,
    pub media: Option<Vec<InputRichMessageMedia>>,
    pub is_rtl: Option<bool>,
    pub skip_entity_detection: Option<bool>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputRichMessageMedia {
    pub id: String,
    pub media: InputRichMessageMediaKind,
}

/// The media of an [`InputRichMessageMedia`].
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputRichMessageMediaKind {
    Animation(InputMediaAnimation),
    Audio(InputMediaAudio),
    Document(InputMediaDocument),
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
    VoiceNote(InputMediaVoiceNote),
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
    Button(Box<RichTextButton>),
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

impl From<RichTextButton> for RichText {
    fn from(value: RichTextButton) -> Self {
        Self::Object(RichTextObject::Button(Box::new(value)))
    }
}

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
pub struct RichMessageButton {
    pub text: Box<RichText>,
    pub style: Option<String>,
    pub url: Option<String>,
    pub callback_data: Option<String>,
    pub web_app: Option<WebAppInfo>,
    pub login_url: Option<LoginUrl>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    pub copy_text: Option<CopyTextButton>,
    pub disabled: Option<DisabledButton>,
}

#[apply(apistruct!)]
pub struct RichTextButton {
    pub button: RichMessageButton,
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
    ExpandableBlockquote(RichBlockExpandableBlockQuotation),
    Pullquote(RichBlockPullQuotation),
    Collage(RichBlockCollage),
    Slideshow(RichBlockSlideshow),
    Table(RichBlockTable),
    Details(RichBlockDetails),
    Map(RichBlockMap),
    Buttons(RichBlockButtons),
    Animation(RichBlockAnimation),
    Audio(RichBlockAudio),
    Document(RichBlockDocument),
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
pub struct RichBlockExpandableBlockQuotation {
    pub text: RichText,
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
    pub is_compact: Option<bool>,
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
pub struct RichBlockButtons {
    pub buttons: Vec<RichMessageButton>,
    pub align: Option<String>,
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
pub struct RichBlockDocument {
    pub document: Document,
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputRichBlock {
    Paragraph(InputRichBlockParagraph),
    Heading(InputRichBlockSectionHeading),
    Pre(InputRichBlockPreformatted),
    Footer(InputRichBlockFooter),
    Divider(InputRichBlockDivider),
    MathematicalExpression(InputRichBlockMathematicalExpression),
    Anchor(InputRichBlockAnchor),
    List(InputRichBlockList),
    Blockquote(InputRichBlockBlockQuotation),
    ExpandableBlockquote(InputRichBlockExpandableBlockQuotation),
    Pullquote(InputRichBlockPullQuotation),
    Collage(InputRichBlockCollage),
    Slideshow(InputRichBlockSlideshow),
    Table(InputRichBlockTable),
    Details(InputRichBlockDetails),
    Map(InputRichBlockMap),
    Buttons(InputRichBlockButtons),
    Animation(InputRichBlockAnimation),
    Audio(InputRichBlockAudio),
    Document(InputRichBlockDocument),
    Photo(InputRichBlockPhoto),
    Video(InputRichBlockVideo),
    VoiceNote(InputRichBlockVoiceNote),
    Thinking(InputRichBlockThinking),
}

#[apply(apistruct!)]
pub struct InputRichBlockParagraph {
    pub text: RichText,
}

#[apply(apistruct!)]
pub struct InputRichBlockSectionHeading {
    pub text: RichText,
    pub size: u8,
}

#[apply(apistruct!)]
pub struct InputRichBlockPreformatted {
    pub text: RichText,
    pub language: Option<String>,
}

#[apply(apistruct!)]
pub struct InputRichBlockFooter {
    pub text: RichText,
}

#[apply(apistruct!)]
#[derive(Copy, Eq)]
pub struct InputRichBlockDivider {}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputRichBlockMathematicalExpression {
    pub expression: String,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct InputRichBlockAnchor {
    pub name: String,
}

#[apply(apistruct!)]
pub struct InputRichBlockList {
    pub items: Vec<InputRichBlockListItem>,
}

#[apply(apistruct!)]
pub struct InputRichBlockListItem {
    pub blocks: Vec<InputRichBlock>,
    pub has_checkbox: Option<bool>,
    pub is_checked: Option<bool>,
    pub value: Option<i32>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[apply(apistruct!)]
pub struct InputRichBlockBlockQuotation {
    pub blocks: Vec<InputRichBlock>,
    pub credit: Option<RichText>,
}

#[apply(apistruct!)]
pub struct InputRichBlockExpandableBlockQuotation {
    pub text: RichText,
    pub credit: Option<RichText>,
}

#[apply(apistruct!)]
pub struct InputRichBlockPullQuotation {
    pub text: RichText,
    pub credit: Option<RichText>,
}

#[apply(apistruct!)]
pub struct InputRichBlockCollage {
    pub blocks: Vec<InputRichBlock>,
    pub caption: Option<RichBlockCaption>,
}

#[apply(apistruct!)]
pub struct InputRichBlockSlideshow {
    pub blocks: Vec<InputRichBlock>,
    pub caption: Option<RichBlockCaption>,
}

#[apply(apistruct!)]
pub struct InputRichBlockTable {
    pub cells: Vec<Vec<RichBlockTableCell>>,
    pub is_bordered: Option<bool>,
    pub is_striped: Option<bool>,
    pub is_compact: Option<bool>,
    pub caption: Option<RichText>,
}

#[apply(apistruct!)]
pub struct InputRichBlockDetails {
    pub summary: RichText,
    pub blocks: Vec<InputRichBlock>,
    pub is_open: Option<bool>,
}

#[apply(apistruct!)]
pub struct InputRichBlockMap {
    pub location: Location,
    pub zoom: u8,
    pub width: u32,
    pub height: u32,
    pub caption: Option<RichBlockCaption>,
}

#[apply(apistruct!)]
pub struct InputRichBlockButtons {
    pub buttons: Vec<RichMessageButton>,
    pub align: Option<String>,
}

#[apply(apistruct!)]
pub struct InputRichBlockAnimation {
    pub animation: InputMediaAnimation,
    pub caption: Option<RichBlockCaption>,
}

#[apply(apistruct!)]
pub struct InputRichBlockAudio {
    pub audio: InputMediaAudio,
    pub caption: Option<RichBlockCaption>,
}

#[apply(apistruct!)]
pub struct InputRichBlockDocument {
    pub document: InputMediaDocument,
    pub caption: Option<RichBlockCaption>,
}

#[apply(apistruct!)]
pub struct InputRichBlockPhoto {
    pub photo: InputMediaPhoto,
    pub caption: Option<RichBlockCaption>,
}

#[apply(apistruct!)]
pub struct InputRichBlockVideo {
    pub video: InputMediaVideo,
    pub caption: Option<RichBlockCaption>,
}

#[apply(apistruct!)]
pub struct InputRichBlockVoiceNote {
    pub voice_note: InputMediaVoiceNote,
    pub caption: Option<RichBlockCaption>,
}

#[apply(apistruct!)]
pub struct InputRichBlockThinking {
    pub text: RichText,
}

#[cfg(any(feature = "trait-sync", feature = "trait-async"))]
mod input_file_replacement {
    use std::path::PathBuf;

    use super::{InputRichBlock, InputRichMessage, InputRichMessageMediaKind};
    use crate::input_file::HasInputFile;
    use crate::input_media::{
        InputMediaAnimation, InputMediaAudio, InputMediaDocument, InputMediaPhoto, InputMediaVideo,
        InputMediaVoiceNote,
    };

    type Files = Vec<(String, PathBuf)>;

    macro_rules! replace_attach {
        ($property:expr, $files:ident) => {
            if let Some(file) = $property.replace_attach_dyn(|| $files.len()) {
                $files.push(file);
            }
        };
    }

    fn replace_animation(animation: &mut InputMediaAnimation, files: &mut Files) {
        replace_attach!(animation.media, files);
        replace_attach!(animation.thumbnail, files);
    }

    fn replace_audio(audio: &mut InputMediaAudio, files: &mut Files) {
        replace_attach!(audio.media, files);
        replace_attach!(audio.thumbnail, files);
    }

    fn replace_document(document: &mut InputMediaDocument, files: &mut Files) {
        replace_attach!(document.media, files);
        replace_attach!(document.thumbnail, files);
    }

    fn replace_photo(photo: &mut InputMediaPhoto, files: &mut Files) {
        replace_attach!(photo.media, files);
    }

    fn replace_video(video: &mut InputMediaVideo, files: &mut Files) {
        replace_attach!(video.media, files);
        replace_attach!(video.cover, files);
        replace_attach!(video.thumbnail, files);
    }

    fn replace_voice_note(voice_note: &mut InputMediaVoiceNote, files: &mut Files) {
        replace_attach!(voice_note.media, files);
    }

    fn replace_blocks(blocks: &mut [InputRichBlock], files: &mut Files) {
        for block in blocks {
            match block {
                InputRichBlock::List(list) => {
                    for item in &mut list.items {
                        replace_blocks(&mut item.blocks, files);
                    }
                }
                InputRichBlock::Blockquote(blockquote) => {
                    replace_blocks(&mut blockquote.blocks, files);
                }
                InputRichBlock::Collage(collage) => replace_blocks(&mut collage.blocks, files),
                InputRichBlock::Slideshow(slideshow) => {
                    replace_blocks(&mut slideshow.blocks, files);
                }
                InputRichBlock::Details(details) => replace_blocks(&mut details.blocks, files),
                InputRichBlock::Animation(b) => replace_animation(&mut b.animation, files),
                InputRichBlock::Audio(b) => replace_audio(&mut b.audio, files),
                InputRichBlock::Document(b) => replace_document(&mut b.document, files),
                InputRichBlock::Photo(b) => replace_photo(&mut b.photo, files),
                InputRichBlock::Video(b) => replace_video(&mut b.video, files),
                InputRichBlock::VoiceNote(b) => replace_voice_note(&mut b.voice_note, files),
                InputRichBlock::Paragraph(_)
                | InputRichBlock::Heading(_)
                | InputRichBlock::Pre(_)
                | InputRichBlock::Footer(_)
                | InputRichBlock::Divider(_)
                | InputRichBlock::MathematicalExpression(_)
                | InputRichBlock::Anchor(_)
                | InputRichBlock::ExpandableBlockquote(_)
                | InputRichBlock::Pullquote(_)
                | InputRichBlock::Table(_)
                | InputRichBlock::Map(_)
                | InputRichBlock::Buttons(_)
                | InputRichBlock::Thinking(_) => {}
            }
        }
    }

    impl InputRichMessage {
        pub(crate) fn replace_input_files(&mut self) -> Files {
            let mut files = Files::new();
            if let Some(media) = &mut self.media {
                for media in media {
                    match &mut media.media {
                        InputRichMessageMediaKind::Animation(animation) => {
                            replace_animation(animation, &mut files);
                        }
                        InputRichMessageMediaKind::Audio(audio) => {
                            replace_audio(audio, &mut files);
                        }
                        InputRichMessageMediaKind::Document(document) => {
                            replace_document(document, &mut files);
                        }
                        InputRichMessageMediaKind::Photo(photo) => {
                            replace_photo(photo, &mut files);
                        }
                        InputRichMessageMediaKind::Video(video) => {
                            replace_video(video, &mut files);
                        }
                        InputRichMessageMediaKind::VoiceNote(voice_note) => {
                            replace_voice_note(voice_note, &mut files);
                        }
                    }
                }
            }
            if let Some(blocks) = &mut self.blocks {
                replace_blocks(blocks, &mut files);
            }
            files
        }
    }

    #[cfg(test)]
    mod tests {
        use std::path::PathBuf;

        use super::super::{
            InputRichBlock, InputRichBlockDetails, InputRichBlockDocument, InputRichBlockPhoto,
            InputRichMessage, InputRichMessageMedia, InputRichMessageMediaKind, RichText,
        };
        use crate::input_media::{InputMediaDocument, InputMediaPhoto, InputMediaVideo};

        #[test]
        fn input_files_are_replaced_with_attach_references() {
            let mut message = InputRichMessage::builder()
                .media(vec![
                    InputRichMessageMedia::builder()
                        .id("video1")
                        .media(InputRichMessageMediaKind::Video(
                            InputMediaVideo::builder()
                                .media(PathBuf::from("video.mp4"))
                                .thumbnail(PathBuf::from("thumbnail.jpg"))
                                .build(),
                        ))
                        .build(),
                    InputRichMessageMedia::builder()
                        .id("document1")
                        .media(InputRichMessageMediaKind::Document(
                            InputMediaDocument::builder()
                                .media(PathBuf::from("document.pdf"))
                                .thumbnail(PathBuf::from("document-thumbnail.jpg"))
                                .build(),
                        ))
                        .build(),
                ])
                .blocks(vec![InputRichBlock::Details(
                    InputRichBlockDetails::builder()
                        .summary(RichText::from("photos"))
                        .blocks(vec![
                            InputRichBlock::Photo(
                                InputRichBlockPhoto::builder()
                                    .photo(
                                        InputMediaPhoto::builder()
                                            .media(PathBuf::from("photo.jpg"))
                                            .build(),
                                    )
                                    .build(),
                            ),
                            InputRichBlock::Document(
                                InputRichBlockDocument::builder()
                                    .document(
                                        InputMediaDocument::builder()
                                            .media(PathBuf::from("block-document.pdf"))
                                            .thumbnail(PathBuf::from(
                                                "block-document-thumbnail.jpg",
                                            ))
                                            .build(),
                                    )
                                    .build(),
                            ),
                        ])
                        .build(),
                )])
                .build();

            let files = message.replace_input_files();

            assert_eq!(
                files,
                vec![
                    ("file0".to_owned(), PathBuf::from("video.mp4")),
                    ("file1".to_owned(), PathBuf::from("thumbnail.jpg")),
                    ("file2".to_owned(), PathBuf::from("document.pdf")),
                    ("file3".to_owned(), PathBuf::from("document-thumbnail.jpg")),
                    ("file4".to_owned(), PathBuf::from("photo.jpg")),
                    ("file5".to_owned(), PathBuf::from("block-document.pdf")),
                    (
                        "file6".to_owned(),
                        PathBuf::from("block-document-thumbnail.jpg")
                    ),
                ]
            );

            let value = serde_json::to_value(&message).unwrap();
            assert_eq!(value["media"][0]["media"]["media"], "attach://file0");
            assert_eq!(value["media"][0]["media"]["thumbnail"], "attach://file1");
            assert_eq!(value["media"][1]["media"]["media"], "attach://file2");
            assert_eq!(value["media"][1]["media"]["thumbnail"], "attach://file3");
            assert_eq!(
                value["blocks"][0]["blocks"][0]["photo"]["media"],
                "attach://file4"
            );
            assert_eq!(
                value["blocks"][0]["blocks"][1]["document"]["media"],
                "attach://file5"
            );
            assert_eq!(
                value["blocks"][0]["blocks"][1]["document"]["thumbnail"],
                "attach://file6"
            );
        }
    }
}
