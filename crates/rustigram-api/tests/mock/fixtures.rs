//! Minimal valid values for the argument types the client's builders require.
//!
//! Built from JSON rather than struct literals: every one of these implements
//! `Deserialize`, and a JSON body states the wire shape the test actually cares
//! about without restating each type's Rust field names and module paths.

#![allow(dead_code)] // each test binary uses a different subset

use serde::de::DeserializeOwned;
use serde_json::json;

use rustigram_types::checklist::InputChecklist;
use rustigram_types::file::{InputFile, InputMedia, InputPaidMedia, InputProfilePhoto};
use rustigram_types::inline::InlineQueryResult;
use rustigram_types::keyboard::KeyboardButton;
use rustigram_types::message::ReplyParameters;
use rustigram_types::passport::PassportElementError;
use rustigram_types::payments::LabeledPrice;
use rustigram_types::poll::InputPollOption;
use rustigram_types::rich_message::InputRichMessage;
use rustigram_types::sticker::InputSticker;
use rustigram_types::story::InputStoryContent;
use rustigram_types::user::BotCommand;

/// Decodes a fixture, failing loudly: a fixture that silently became something
/// else would make every test built on it assert the wrong thing.
fn build<T: DeserializeOwned>(value: serde_json::Value) -> T {
    serde_json::from_value(value.clone())
        .unwrap_or_else(|e| panic!("fixture does not decode as its type: {e}\n  {value}"))
}

/// A file sent by `file_id`, which takes the JSON send path.
pub fn input_file() -> InputFile {
    InputFile::FileId("test-file-id".to_owned())
}

/// The same file as raw bytes, which takes the multipart send path.
pub fn uploaded_file() -> InputFile {
    InputFile::Bytes {
        filename: "p.jpg".to_owned(),
        data: b"\xff\xd8\xff".to_vec(),
        mime_type: "image/jpeg".to_owned(),
    }
}

pub fn input_media() -> InputMedia {
    build(json!({ "type": "photo", "media": "test-file-id" }))
}

pub fn paid_media() -> InputPaidMedia {
    build(json!({ "type": "photo", "media": "test-file-id" }))
}

pub fn profile_photo() -> InputProfilePhoto {
    build(json!({ "type": "static", "photo": "attach://photo" }))
}

pub fn inline_result() -> InlineQueryResult {
    build(json!({
        "type": "article",
        "id": "1",
        "title": "t",
        "input_message_content": { "message_text": "x" }
    }))
}

pub fn story_content() -> InputStoryContent {
    build(json!({ "type": "photo", "photo": "attach://photo" }))
}

pub fn checklist() -> InputChecklist {
    build(json!({ "title": "t", "tasks": [{ "id": 1, "text": "task" }] }))
}

pub fn rich_message() -> InputRichMessage {
    build(json!({ "blocks": [{ "type": "paragraph", "text": "hi" }] }))
}

pub fn sticker() -> InputSticker {
    build(json!({ "sticker": "test-file-id", "format": "static", "emoji_list": ["🙂"] }))
}

pub fn keyboard_button() -> KeyboardButton {
    build(json!({ "text": "b" }))
}

pub fn passport_error() -> PassportElementError {
    build(json!({
        "source": "unspecified",
        "type": "passport",
        "element_hash": "h",
        "message": "m"
    }))
}

pub fn labeled_price() -> LabeledPrice {
    build(json!({ "label": "l", "amount": 1 }))
}

pub fn poll_option() -> InputPollOption {
    build(json!({ "text": "o" }))
}

/// A reply pointing at one message.
///
/// The media builders take the whole `ReplyParameters`; only `sendMessage` and
/// one other expose a `reply_to(id)` convenience over it.
pub fn reply_to(message_id: i64) -> ReplyParameters {
    build(json!({ "message_id": message_id }))
}

pub fn bot_command() -> BotCommand {
    build(json!({ "command": "start", "description": "d" }))
}
