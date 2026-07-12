use serde::{Deserialize, Serialize};

use crate::chat::Location;
use crate::file::{
    Animation, Audio, InputMediaAnimation, InputMediaAudio, InputMediaPhoto, InputMediaVideo,
    InputMediaVoiceNote, PhotoSize, Video, Voice,
};
use crate::user::User;

// ─── RichText ─────────────────────────────────────────────────────────────────

/// Rich formatted text — a recursive sum type that mirrors the Telegram
/// `RichText` union from Bot API 10.1.
///
/// A `RichText` value is either:
/// - a plain [`String`] (leaf node),
/// - an [`Array`](RichText::Array) of nested `RichText` values, or
/// - one of the typed inline-formatting variants listed below.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RichText {
    /// Plain text without any formatting.
    Plain(String),
    /// A sequence of rich-text nodes rendered consecutively.
    Array(Vec<RichText>),
    /// Bold formatting.
    Bold(Box<RichTextBold>),
    /// Italic formatting.
    Italic(Box<RichTextItalic>),
    /// Underline formatting.
    Underline(Box<RichTextUnderline>),
    /// Strikethrough formatting.
    Strikethrough(Box<RichTextStrikethrough>),
    /// Spoiler — text hidden until tapped.
    Spoiler(Box<RichTextSpoiler>),
    /// A date/time entity.
    DateTime(Box<RichTextDateTime>),
    /// A mention by user object.
    TextMention(Box<RichTextTextMention>),
    /// Subscript text.
    Subscript(Box<RichTextSubscript>),
    /// Superscript text.
    Superscript(Box<RichTextSuperscript>),
    /// Highlighted/marked text.
    Marked(Box<RichTextMarked>),
    /// Inline monospace code.
    Code(Box<RichTextCode>),
    /// A custom emoji.
    CustomEmoji(Box<RichTextCustomEmoji>),
    /// An inline LaTeX mathematical expression.
    MathematicalExpression(Box<RichTextMathematicalExpression>),
    /// A hyperlink.
    Url(Box<RichTextUrl>),
    /// An e-mail address link.
    EmailAddress(Box<RichTextEmailAddress>),
    /// A telephone number link.
    PhoneNumber(Box<RichTextPhoneNumber>),
    /// A bank card number.
    BankCardNumber(Box<RichTextBankCardNumber>),
    /// A `@username` mention.
    Mention(Box<RichTextMention>),
    /// A `#hashtag`.
    Hashtag(Box<RichTextHashtag>),
    /// A `$cashtag`.
    Cashtag(Box<RichTextCashtag>),
    /// A `/bot_command`.
    BotCommand(Box<RichTextBotCommand>),
    /// An in-document anchor definition.
    Anchor(Box<RichTextAnchor>),
    /// A link targeting an in-document anchor.
    AnchorLink(Box<RichTextAnchorLink>),
    /// A footnote body.
    Footnote(Box<RichTextFootnote>),
    /// A reference to a footnote.
    Reference(Box<RichTextReference>),
}

/// Bold text (`**text**` / `<b>text</b>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextBold {
    /// Always `"bold"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The contained rich text.
    pub text: RichText,
}

/// Italic text (`*text*` / `<i>text</i>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextItalic {
    /// Always `"italic"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The contained rich text.
    pub text: RichText,
}

/// Underlined text (`<u>text</u>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextUnderline {
    /// Always `"underline"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The contained rich text.
    pub text: RichText,
}

/// Strikethrough text (`~~text~~` / `<s>text</s>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextStrikethrough {
    /// Always `"strikethrough"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The contained rich text.
    pub text: RichText,
}

/// Spoiler text (`||text||` / `<tg-spoiler>text</tg-spoiler>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextSpoiler {
    /// Always `"spoiler"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The contained rich text.
    pub text: RichText,
}

/// A date/time entity rendered according to the client's locale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextDateTime {
    /// Always `"date_time"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The display text.
    pub text: RichText,
    /// The Unix timestamp associated with the entity.
    pub unix_time: i64,
    /// Format string controlling how the date/time is rendered.
    pub date_time_format: String,
}

/// A mention of a Telegram user by their `User` object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextTextMention {
    /// Always `"text_mention"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The display text.
    pub text: RichText,
    /// The mentioned user.
    pub user: User,
}

/// Subscript text (`<sub>text</sub>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextSubscript {
    /// Always `"subscript"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The contained rich text.
    pub text: RichText,
}

/// Superscript text (`<sup>text</sup>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextSuperscript {
    /// Always `"superscript"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The contained rich text.
    pub text: RichText,
}

/// Highlighted/marked text (`==text==` / `<mark>text</mark>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextMarked {
    /// Always `"marked"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The contained rich text.
    pub text: RichText,
}

/// Inline monospace/code text (`` `text` `` / `<code>text</code>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextCode {
    /// Always `"code"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The contained rich text.
    pub text: RichText,
}

/// A custom emoji (`![alt](tg://emoji?id=...)`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextCustomEmoji {
    /// Always `"custom_emoji"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier of the custom emoji.
    pub custom_emoji_id: String,
    /// Fallback emoji string for clients that do not support custom emoji.
    pub alternative_text: String,
}

/// An inline LaTeX mathematical expression (`$expr$` / `<tg-math>expr</tg-math>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextMathematicalExpression {
    /// Always `"mathematical_expression"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The LaTeX source of the expression.
    pub expression: String,
}

/// A hyperlink (`[text](url)` / `<a href="url">text</a>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextUrl {
    /// Always `"url"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The display text.
    pub text: RichText,
    /// The target URL.
    pub url: String,
}

/// An e-mail address link (`[text](mailto:addr)` / `<a href="mailto:addr">text</a>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextEmailAddress {
    /// Always `"email_address"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The display text.
    pub text: RichText,
    /// The raw e-mail address.
    pub email_address: String,
}

/// A telephone number link (`[text](tel:+nnn)` / `<a href="tel:+nnn">text</a>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextPhoneNumber {
    /// Always `"phone_number"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The display text.
    pub text: RichText,
    /// The raw phone number.
    pub phone_number: String,
}

/// A bank card number.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextBankCardNumber {
    /// Always `"bank_card_number"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The display text.
    pub text: RichText,
    /// The raw bank card number.
    pub bank_card_number: String,
}

/// A `@username` mention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextMention {
    /// Always `"mention"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The display text.
    pub text: RichText,
    /// The target username (without the leading `@`).
    pub username: String,
}

/// A `#hashtag`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextHashtag {
    /// Always `"hashtag"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The display text.
    pub text: RichText,
    /// The hashtag value (without the leading `#`).
    pub hashtag: String,
}

/// A `$cashtag`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextCashtag {
    /// Always `"cashtag"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The display text.
    pub text: RichText,
    /// The cashtag value (without the leading `$`).
    pub cashtag: String,
}

/// A bot command (e.g. `/start`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextBotCommand {
    /// Always `"bot_command"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The display text.
    pub text: RichText,
    /// The command string including the leading `/`.
    pub bot_command: String,
}

/// An in-document anchor definition (`<a name="id"></a>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextAnchor {
    /// Always `"anchor"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The anchor name.
    pub name: String,
}

/// A link to an in-document anchor (`<a href="#id">text</a>`).
///
/// If `anchor_name` is empty the link scrolls back to the top of the message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextAnchorLink {
    /// Always `"anchor_link"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The display text.
    pub text: RichText,
    /// The target anchor name; empty string scrolls to the top.
    pub anchor_name: String,
}

/// The body of a footnote.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextFootnote {
    /// Always `"footnote"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The footnote content.
    pub text: RichText,
    /// The footnote identifier.
    pub name: String,
}

/// A reference to a previously defined footnote.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextReference {
    /// Always `"reference"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The display text (typically the footnote superscript label).
    pub text: RichText,
    /// The footnote identifier being referenced.
    pub footnote_name: String,
}

// ─── RichBlock helpers ────────────────────────────────────────────────────────

/// Caption (and optional credit) for a media block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockCaption {
    /// The caption text.
    pub text: RichText,
    /// Optional credit line (HTML `<cite>`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<RichText>,
}

/// A single cell inside a [`RichBlockTable`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockTableCell {
    /// The cell content; omit to leave the cell empty/invisible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<RichText>,
    /// `true` if this is a header cell (`<th>`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_header: Option<bool>,
    /// Number of columns the cell spans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colspan: Option<u32>,
    /// Number of rows the cell spans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rowspan: Option<u32>,
    /// Horizontal text alignment: `"left"`, `"center"`, or `"right"`.
    pub align: String,
    /// Vertical text alignment: `"top"`, `"middle"`, or `"bottom"`.
    pub valign: String,
}

/// A single item inside a [`RichBlockList`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockListItem {
    /// The bullet or number label rendered by the client.
    pub label: String,
    /// The nested content of this list item.
    pub blocks: Vec<RichBlock>,
    /// `true` if the item has a checkbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_checkbox: Option<bool>,
    /// `true` if the checkbox is checked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_checked: Option<bool>,
    /// For ordered lists — the explicit numeric value of this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    /// For ordered lists — the label type: `"a"`, `"A"`, `"i"`, `"I"`, or `"1"`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

// ─── RichBlock ────────────────────────────────────────────────────────────────

/// A block-level element in a rich message.
///
/// This is the top-level building block for `RichMessage::blocks`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichBlock {
    /// A text paragraph (`<p>`).
    Paragraph(RichBlockParagraph),
    /// A section heading (`<h1>`…`<h6>`).
    #[serde(rename = "heading")]
    SectionHeading(RichBlockSectionHeading),
    /// A preformatted / code block (`<pre><code>`).
    Pre(RichBlockPreformatted),
    /// A footer (`<footer>`).
    Footer(RichBlockFooter),
    /// A horizontal rule / divider (`<hr/>`).
    Divider(RichBlockDivider),
    /// A block-level LaTeX expression (`<tg-math-block>`).
    MathematicalExpression(RichBlockMathematicalExpression),
    /// An in-document anchor (`<a name="…"></a>`).
    Anchor(RichBlockAnchor),
    /// An ordered or unordered list (`<ul>` / `<ol>`).
    List(RichBlockList),
    /// A block quotation (`<blockquote>`).
    Blockquote(RichBlockBlockQuotation),
    /// A pull quotation (`<aside>`).
    Pullquote(RichBlockPullQuotation),
    /// A multi-media collage (`<tg-collage>`).
    Collage(RichBlockCollage),
    /// A media slideshow (`<tg-slideshow>`).
    Slideshow(RichBlockSlideshow),
    /// A table (`<table>`).
    Table(RichBlockTable),
    /// A collapsible details block (`<details>`).
    Details(RichBlockDetails),
    /// An embedded map (`<tg-map>`).
    Map(RichBlockMap),
    /// A looping animation / GIF (`<video loop>`).
    Animation(RichBlockAnimation),
    /// An audio file (`<audio>`).
    Audio(RichBlockAudio),
    /// A photo (`<photo>`).
    Photo(RichBlockPhoto),
    /// A video (`<video>`).
    Video(RichBlockVideo),
    /// A voice note (`<audio>` in voice-note context).
    VoiceNote(RichBlockVoiceNote),
    /// A `Thinking…` placeholder used during AI streaming drafts.
    ///
    /// Only valid in [`sendRichMessageDraft`](https://core.telegram.org/bots/api#sendrichmessagedraft) calls.
    Thinking(RichBlockThinking),
}

/// A text paragraph (`<p>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockParagraph {
    /// The paragraph text.
    pub text: RichText,
}

/// A section heading, corresponding to `<h1>`…`<h6>`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockSectionHeading {
    /// The heading text.
    pub text: RichText,
    /// Font size level 1–6 (1 = largest, 6 = smallest).
    pub size: u8,
}

/// A preformatted text block (`<pre><code>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockPreformatted {
    /// The preformatted text.
    pub text: RichText,
    /// Optional syntax-highlight language identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// A footer block (`<footer>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockFooter {
    /// The footer text.
    pub text: RichText,
}

/// A horizontal rule / divider (`<hr/>`).
///
/// Has no content fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockDivider {}

/// A block-level mathematical expression in LaTeX format (`<tg-math-block>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockMathematicalExpression {
    /// The raw LaTeX source.
    pub expression: String,
}

/// An in-document anchor (`<a name="…"></a>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockAnchor {
    /// The anchor name.
    pub name: String,
}

/// An ordered or unordered list (`<ul>` / `<ol>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockList {
    /// The list items.
    pub items: Vec<RichBlockListItem>,
}

/// A block quotation (`<blockquote>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockBlockQuotation {
    /// Nested block content.
    pub blocks: Vec<RichBlock>,
    /// Optional attribution credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<RichText>,
}

/// A pull quotation with centred text (`<aside>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockPullQuotation {
    /// The quotation text.
    pub text: RichText,
    /// Optional attribution credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<RichText>,
}

/// A multi-media collage (`<tg-collage>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockCollage {
    /// The media elements of the collage.
    pub blocks: Vec<RichBlock>,
    /// Optional caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

/// A media slideshow (`<tg-slideshow>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockSlideshow {
    /// The media elements of the slideshow.
    pub blocks: Vec<RichBlock>,
    /// Optional caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

/// A table (`<table>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockTable {
    /// A 2-D array of cells (rows × columns).
    pub cells: Vec<Vec<RichBlockTableCell>>,
    /// `true` if the table has visible borders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bordered: Option<bool>,
    /// `true` if alternate rows are shaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_striped: Option<bool>,
    /// Optional table caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichText>,
}

/// A collapsible details / disclosure block (`<details>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockDetails {
    /// The always-visible summary.
    pub summary: RichText,
    /// Nested block content shown when expanded.
    pub blocks: Vec<RichBlock>,
    /// `true` if the block is expanded by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_open: Option<bool>,
}

/// An embedded map (`<tg-map>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockMap {
    /// Latitude of the map centre.
    pub latitude: f64,
    /// Longitude of the map centre.
    pub longitude: f64,
    /// Zoom level (13–20).
    pub zoom: u8,
    /// Expected rendered width in pixels.
    pub width: u32,
    /// Expected rendered height in pixels.
    pub height: u32,
    /// Optional caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

/// A looping animation / GIF block (`<video loop>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockAnimation {
    /// The animation file.
    pub animation: Animation,
    /// `true` if the animation should play automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_autoplay: Option<bool>,
    /// `true` if a spoiler overlay is shown before the first tap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    /// Optional caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

/// An audio file block (`<audio>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockAudio {
    /// The audio file.
    pub audio: Audio,
    /// Optional caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

/// A photo block (`<photo>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockPhoto {
    /// All available sizes of the photo.
    pub photo: Vec<PhotoSize>,
    /// `true` if a spoiler overlay is shown before the first tap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    /// Optional caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

/// A video block (`<video>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockVideo {
    /// The video file.
    pub video: Video,
    /// `true` if the video should play automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_autoplay: Option<bool>,
    /// `true` if the video loops back to the start when it ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_looped: Option<bool>,
    /// `true` if a spoiler overlay is shown before the first tap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    /// Optional caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

/// A voice note block (`<audio>` in voice-note context).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockVoiceNote {
    /// The voice note file.
    pub voice_note: Voice,
    /// Optional caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

/// A `Thinking…` placeholder for use while a bot streams an AI response.
///
/// Only valid inside [`sendRichMessageDraft`](https://core.telegram.org/bots/api#sendrichmessagedraft) calls.
/// See <https://t.me/addemoji/AIActions> for recommended custom emoji.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichBlockThinking {
    /// The placeholder display text (may include custom emoji).
    pub text: RichText,
}

// ─── RichMessage ──────────────────────────────────────────────────────────────

/// A complete rich formatted message as received from the Bot API.
///
/// Carried in `Message::rich_message`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichMessage {
    /// The ordered list of top-level blocks forming the message body.
    pub blocks: Vec<RichBlock>,
    /// `true` if the message must be rendered right-to-left.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rtl: Option<bool>,
}

// ─── InputRichBlock helpers ───────────────────────────────────────────────────

/// An item of a list to be sent, contained in an [`InputRichBlockList`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockListItem {
    /// The content of the item.
    pub blocks: Vec<InputRichBlock>,
    /// Pass `true` if the item has a checkbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_checkbox: Option<bool>,
    /// Pass `true` if the item has a checked checkbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_checked: Option<bool>,
    /// For ordered lists, the numeric value of the item label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    /// For ordered lists, the type of the item label: `"a"`, `"A"`, `"i"`, `"I"`, or `"1"`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

// ─── InputRichBlock ───────────────────────────────────────────────────────────

/// A block in a rich formatted message to be sent.
///
/// This is the input-side counterpart to [`RichBlock`] — used to build
/// `InputRichMessage::blocks` rather than to read a received `RichMessage`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputRichBlock {
    /// A text paragraph, corresponding to the HTML tag `<p>`.
    Paragraph(InputRichBlockParagraph),
    /// A section heading, corresponding to the HTML tags `<h1>`…`<h6>`.
    #[serde(rename = "heading")]
    SectionHeading(InputRichBlockSectionHeading),
    /// A preformatted text block, corresponding to the nested HTML tags `<pre>` and `<code>`.
    #[serde(rename = "pre")]
    Preformatted(InputRichBlockPreformatted),
    /// A footer, corresponding to the HTML tag `<footer>`.
    Footer(InputRichBlockFooter),
    /// A divider, corresponding to the HTML tag `<hr/>`.
    Divider(InputRichBlockDivider),
    /// A block with a mathematical expression in LaTeX format, corresponding
    /// to the custom HTML tag `<tg-math-block>`.
    MathematicalExpression(InputRichBlockMathematicalExpression),
    /// A block with an anchor, corresponding to the HTML tag `<a>` with the attribute `name`.
    Anchor(InputRichBlockAnchor),
    /// A list of blocks, corresponding to the HTML tag `<ul>` or `<ol>` with nested `<li>` tags.
    List(InputRichBlockList),
    /// A block quotation, corresponding to the HTML tag `<blockquote>`.
    #[serde(rename = "blockquote")]
    BlockQuotation(InputRichBlockBlockQuotation),
    /// A quotation with centered text, loosely corresponding to the HTML tag `<aside>`.
    #[serde(rename = "pullquote")]
    PullQuotation(InputRichBlockPullQuotation),
    /// A collage, corresponding to the custom HTML tag `<tg-collage>`.
    Collage(InputRichBlockCollage),
    /// A slideshow, corresponding to the custom HTML tag `<tg-slideshow>`.
    Slideshow(InputRichBlockSlideshow),
    /// A table, corresponding to the HTML tag `<table>`.
    Table(InputRichBlockTable),
    /// An expandable block for details disclosure, corresponding to the HTML tag `<details>`.
    Details(InputRichBlockDetails),
    /// A block with a map, corresponding to the custom HTML tag `<tg-map>`.
    Map(InputRichBlockMap),
    /// A block with an animation, corresponding to the HTML tag `<video>`.
    Animation(InputRichBlockAnimation),
    /// A block with a music file, corresponding to the HTML tag `<audio>`.
    Audio(InputRichBlockAudio),
    /// A block with a photo, corresponding to the HTML tag `<img>`.
    Photo(InputRichBlockPhoto),
    /// A block with a video, corresponding to the HTML tag `<video>`.
    Video(InputRichBlockVideo),
    /// A block with a voice note, corresponding to the HTML tag `<audio>`.
    VoiceNote(InputRichBlockVoiceNote),
    /// A `Thinking…` placeholder, corresponding to the custom HTML tag `<tg-thinking>`.
    ///
    /// May be used only in [`sendRichMessageDraft`](https://core.telegram.org/bots/api#sendrichmessagedraft)
    /// calls — it cannot be received in messages. See <https://t.me/addemoji/AIActions>
    /// for recommended custom emoji.
    Thinking(InputRichBlockThinking),
}

/// A text paragraph (`<p>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockParagraph {
    /// Text of the block.
    pub text: RichText,
}

/// A section heading (`<h1>`…`<h6>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockSectionHeading {
    /// Text of the block.
    pub text: RichText,
    /// Relative size of the text font; 1–6, `1` is the largest, `6` is the smallest.
    pub size: u8,
}

/// A preformatted text block (`<pre><code>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockPreformatted {
    /// Text of the block.
    pub text: RichText,
    /// The programming language of the text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// A footer (`<footer>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockFooter {
    /// Text of the block.
    pub text: RichText,
}

/// A divider (`<hr/>`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputRichBlockDivider {}

/// A mathematical expression in LaTeX format (`<tg-math-block>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockMathematicalExpression {
    /// The mathematical expression in LaTeX format.
    pub expression: String,
}

/// An anchor (`<a name="…">`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockAnchor {
    /// The name of the anchor.
    pub name: String,
}

/// A list of blocks (`<ul>` / `<ol>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockList {
    /// Items of the list.
    pub items: Vec<InputRichBlockListItem>,
}

/// A block quotation (`<blockquote>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockBlockQuotation {
    /// Content of the block.
    pub blocks: Vec<InputRichBlock>,
    /// Credit of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<RichText>,
}

/// A quotation with centered text (`<aside>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockPullQuotation {
    /// Text of the block.
    pub text: RichText,
    /// Credit of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<RichText>,
}

/// A collage (`<tg-collage>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockCollage {
    /// Elements of the collage.
    pub blocks: Vec<InputRichBlock>,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

/// A slideshow (`<tg-slideshow>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockSlideshow {
    /// Elements of the slideshow.
    pub blocks: Vec<InputRichBlock>,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

/// A table (`<table>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockTable {
    /// Cells of the table.
    pub cells: Vec<Vec<RichBlockTableCell>>,
    /// Pass `true` if the table has borders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bordered: Option<bool>,
    /// Pass `true` if the table is striped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_striped: Option<bool>,
    /// Caption of the table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichText>,
}

/// An expandable details disclosure block (`<details>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockDetails {
    /// Always shown summary of the block.
    pub summary: RichText,
    /// Content of the block.
    pub blocks: Vec<InputRichBlock>,
    /// Pass `true` if the content of the block is visible by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_open: Option<bool>,
}

/// A map (`<tg-map>`).
///
/// The map's width and height must not exceed 10000 in total. The width and
/// height ratio must be at most 20.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockMap {
    /// Location of the center of the map.
    pub location: Location,
    /// Map zoom level; 0–24.
    pub zoom: u8,
    /// Map width; 0–10000.
    pub width: u32,
    /// Map height; 0–10000.
    pub height: u32,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

/// An animation block (`<video>`). Caption on the inner media is ignored —
/// use this block's own `caption` instead.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockAnimation {
    /// The animation.
    pub animation: InputMediaAnimation,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

/// A music file block (`<audio>`). Caption on the inner media is ignored —
/// use this block's own `caption` instead.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockAudio {
    /// The audio.
    pub audio: InputMediaAudio,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

/// A photo block (`<img>`). Caption on the inner media is ignored — use this
/// block's own `caption` instead.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockPhoto {
    /// The photo.
    pub photo: InputMediaPhoto,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

/// A video block (`<video>`). Caption on the inner media is ignored — use
/// this block's own `caption` instead.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockVideo {
    /// The video.
    pub video: InputMediaVideo,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

/// A voice note block (`<audio>` in voice-note context). Caption on the inner
/// media is ignored — use this block's own `caption` instead.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockVoiceNote {
    /// The voice note.
    pub voice_note: InputMediaVoiceNote,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

/// A `Thinking…` placeholder for use while a bot streams an AI response.
///
/// Only valid inside [`sendRichMessageDraft`](https://core.telegram.org/bots/api#sendrichmessagedraft)
/// calls, therefore it can't be received in messages. See
/// <https://t.me/addemoji/AIActions> for examples of custom emoji recommended
/// for use in this block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichBlockThinking {
    /// Text of the block. May include custom emoji — see
    /// <https://t.me/addemoji/AIActions> for recommended examples.
    pub text: RichText,
}

// ─── InputRichMessageMedia ──────────────────────────────────────────────────

/// The media to be sent, embedded in an outgoing rich message.
///
/// Referenced from `InputRichMessage.html`/`.markdown` via
/// `tg://photo?id=`, `tg://video?id=`, and `tg://audio?id=` links.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputRichMessageMediaKind {
    /// An animation (GIF or silent H.264).
    Animation(InputMediaAnimation),
    /// An audio file treated as music.
    Audio(InputMediaAudio),
    /// A photo.
    Photo(InputMediaPhoto),
    /// A video.
    Video(InputMediaVideo),
    /// A voice message.
    VoiceNote(InputMediaVoiceNote),
}

/// Describes a media element embedded in an outgoing rich message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichMessageMedia {
    /// Unique identifier of the media, referenced from a `tg://photo?id=`,
    /// `tg://video?id=`, or `tg://audio?id=` link. 1–64 characters; only
    /// `A-Z`, `a-z`, `0-9`, `_` and `-` are allowed.
    pub id: String,
    /// The media to be sent. Everything except the media itself and its
    /// properties is ignored.
    pub media: InputRichMessageMediaKind,
}

// ─── InputRichMessage ─────────────────────────────────────────────────────────

/// Describes a rich message to be sent.
///
/// Exactly one of `html`, `markdown`, or `blocks` must be set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichMessage {
    /// Rich message content encoded as HTML.
    ///
    /// Mutually exclusive with [`markdown`](Self::markdown) and [`blocks`](Self::blocks).
    /// Use [`media`](Self::media) to specify the media used in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// Rich message content encoded as Markdown.
    ///
    /// Mutually exclusive with [`html`](Self::html) and [`blocks`](Self::blocks).
    /// Use [`media`](Self::media) to specify the media used in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<String>,
    /// Content of the rich message described as a list of blocks.
    ///
    /// Mutually exclusive with [`html`](Self::html) and [`markdown`](Self::markdown).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<InputRichBlock>>,
    /// Media referenced from [`html`](Self::html) or [`markdown`](Self::markdown)
    /// via `tg://photo?id=`, `tg://video?id=`, and `tg://audio?id=` links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<InputRichMessageMedia>>,
    /// Pass `true` to render the message right-to-left.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rtl: Option<bool>,
    /// Pass `true` to disable automatic entity detection (URLs, mentions, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_entity_detection: Option<bool>,
}

impl InputRichMessage {
    /// Creates an `InputRichMessage` from an HTML string.
    pub fn from_html(html: impl Into<String>) -> Self {
        Self {
            html: Some(html.into()),
            markdown: None,
            blocks: None,
            media: None,
            is_rtl: None,
            skip_entity_detection: None,
        }
    }

    /// Creates an `InputRichMessage` from a Markdown string.
    pub fn from_markdown(markdown: impl Into<String>) -> Self {
        Self {
            html: None,
            markdown: Some(markdown.into()),
            blocks: None,
            media: None,
            is_rtl: None,
            skip_entity_detection: None,
        }
    }

    /// Creates an `InputRichMessage` from a list of explicit blocks.
    pub fn from_blocks(blocks: Vec<InputRichBlock>) -> Self {
        Self {
            html: None,
            markdown: None,
            blocks: Some(blocks),
            media: None,
            is_rtl: None,
            skip_entity_detection: None,
        }
    }

    /// Sets the media referenced by `tg://photo?id=`, `tg://video?id=`, and
    /// `tg://audio?id=` links in [`html`](Self::html) or [`markdown`](Self::markdown).
    #[must_use]
    pub fn media(mut self, media: Vec<InputRichMessageMedia>) -> Self {
        self.media = Some(media);
        self
    }

    /// Sets the right-to-left rendering flag.
    #[must_use]
    pub fn rtl(mut self, v: bool) -> Self {
        self.is_rtl = Some(v);
        self
    }

    /// Disables automatic entity detection.
    #[must_use]
    pub fn skip_entity_detection(mut self, v: bool) -> Self {
        self.skip_entity_detection = Some(v);
        self
    }
}

/// Rich message content to be sent as the result of an inline / guest / Web App query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRichMessageContent {
    /// The rich message to be sent.
    pub rich_message: InputRichMessage,
}
