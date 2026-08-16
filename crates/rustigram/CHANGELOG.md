# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Completes Bot API 10.2 coverage and adds a conformance test suite that checks the
crate against a committed snapshot of the spec. Nine defects were found and
fixed; four of them silently sent or dropped data with no error anywhere.

### Breaking

Nine changes require edits at the call site. The first affects every user; the
rest apply only if you touch the named API.

**Response types are now `#[non_exhaustive]`.** Fifty-two types — `Message`,
`Chat`, `ChatFullInfo`, `User`, and most other things Telegram sends you — can no
longer be built with a struct literal from outside the crate. This is deliberate:
Telegram adds fields constantly, and it means a future Bot API addition is no
longer a breaking change for you. Note that `..Default::default()` does **not**
rescue a struct literal here; build the value and assign instead.

```rust
// before
let msg = Message { message_id: 1, date: 0, chat, ..Default::default() };
// after
let mut msg = Message::default();
msg.message_id = 1;
msg.chat = chat;
```

If you only ever *receive* these types — which is the common case — nothing
changes.

**`RichText` is three shapes, not twenty-seven.** It was an enum of 27 variants
mixing the three wire forms with the 25 node kinds. Those kinds moved into a new
`RichTextNode`, leaving `RichText` as `Plain`, `Array`, and `Node`. The old shape
could not decode correctly at all — every value resolved to the first variant —
so any code matching on it was matching on something that never happened.

```rust
// before
match rich { RichText::Bold(b) => …, RichText::Italic(i) => …, … }
// after
match rich {
    RichText::Node(node) => match *node {
        RichTextNode::Bold(b) => …, RichTextNode::Italic(i) => …, …
    },
    RichText::Plain(s) => …,
    RichText::Array(items) => …,
}
```

**`Chat::guard_bot` moved to `ChatFullInfo`.** The Bot API only ever returns it
from `getChat`, and `Chat` is the summary form embedded in updates — it was never
populated there.

**Three public items were removed**: `RichTextFootnote` (Bot API 10.2 has no
`footnote` discriminant), `update::Updates` (an unused response wrapper), and the
`UpdateListener` / `UpdateStream` pair, which nothing implemented or consumed.

**`set_my_profile_photo` takes a typed value.** It previously required a
pre-serialised JSON string, so callers had to know the wire format.

```rust
// before
client.set_my_profile_photo(serde_json::to_string(&photo)?).await?;
// after
client.set_my_profile_photo(photo).await?;
```

**`StoryAreaType::Location` carries flat coordinates.** It was modelled with a
nested `location` object where Telegram sends `latitude` and `longitude` at the
top level, so this variant could never decode a real payload.

```rust
// before
StoryAreaType::Location { location: Location { latitude, longitude, .. }, address }
// after
StoryAreaType::Location { latitude, longitude, address }
```

**`InputMediaLink` no longer has a `kind` field.** The `type` discriminant
belongs to the enclosing `InputPollOptionMedia`, which is internally tagged.
Declaring it twice meant a link poll option serialised correctly and never
decoded — a bot could send one and never read one back. Construct with
`InputMediaLink::new(url)`, which is unchanged.

**Media builders expose only the options their method accepts.** `send_sticker`
and `send_video_note` no longer have `.caption()` or `.parse_mode()`; the Bot API
defines neither for those methods, so anything set was discarded by Telegram.

### Added

- **43 parameters across 13 methods became settable.** They were declared and
  serialised but had no setter, so a caller could not reach them —
  `sendInvoice` alone accounted for 16, including `protect_content` and
  `reply_parameters`; also `createInvoiceLink`, `promoteChatMember`, `sendPoll`,
  `sendContact`, `sendDice`, `sendLocation`, `answerInlineQuery`, the `edit*`
  family's `business_connection_id`, and `sendMessageDraft`.
- **Every media builder now matches the Bot API's parameter list exactly.**
  `send_video` and `send_animation` gain `.show_caption_above_media()` and
  `.has_spoiler()`; the five captioned builders gain `.caption_entities()`;
  `send_photo` gains `.caption_entities()`; `send_live_photo` gains
  `.caption_entities()`, `.receiver_user_id()`, and `.callback_query_id()`. All
  are parameters the spec defines and no setter reached.
- `.message_effect_id()` on every media builder. It was written to the wire and
  settable from nowhere.
- `sendMediaGroup` gains `allow_paid_broadcast` and `message_effect_id`;
  `sendPaidMedia` gains `allow_paid_broadcast`, `caption_entities`,
  `direct_messages_topic_id`, `message_thread_id`, and
  `suggested_post_parameters`.
- `editEphemeralMessageMedia`, and certificate upload for `setWebhook`.
- Service message, giveaway, gift, paid media, game, background, and business
  types, completing `Message` and `ChatFullInfo`.

### Fixed

- **Options dropped on byte uploads.** `send_audio`, `send_document`,
  `send_video`, `send_animation`, `send_voice`, `send_video_note`, and
  `send_sticker` silently discarded `protect_content`, `reply_parameters`, and
  five other options when the file was uploaded as bytes rather than sent by
  `file_id`. The call succeeded and Telegram never saw them.
- **`message_effect_id` dropped on JSON sends.** The inverse of the above, for
  media sent by `file_id` or URL.
- **`send_live_photo(...).has_spoiler(true)` dropped on byte uploads.** The
  builder kept its own copy of the flag rather than using the shared options, so
  the multipart encoder never saw it. The photo arrived unblurred and the call
  reported success.
- **Failed downloads returned as file contents.** `download_file` did not check
  the HTTP status, so an expired `file_path` produced `Ok` carrying the error
  page instead of an error.
- **`RichText` always decoded as `Bold`.** Every variant resolved to the first
  one; the enum now dispatches on its `type` discriminant.
- **`RichBlockMap` never decoded**, having flat coordinates where the spec nests
  a `location` object. `RichTextReference` sent its name under the wrong key.
- **`OwnedGift::Regular` held the wrong payload type**, so no regular owned gift
  could be decoded.
- **The webhook secret token never reached the server**, so a configured secret
  was not enforced.
- **The Mini App gateway signed a header nothing sets**, leaving the extractor
  unable to validate.
- Cached inline query results sent non-spec `type` values.
- 22 fields and parameters typed `serde_json::Value` are now modelled.

### Notes

Bot API coverage is now complete: 185 of 185 methods, 388 types, 1838 fields.
The suite grew from 119 to 256 tests and runs offline.

`ClientConfig::max_retries` governs JSON requests only — a byte upload is never
retried, because the multipart form is consumed by the send. This was always the
behaviour and is now documented.

## [0.11.0](https://github.com/meh7an/rustigram/compare/rustigram-v0.10.3...rustigram-v0.11.0) - 2026-07-12

### Added

- add Bot API 10.2 ephemeral messages, communities, and rich message blocks

### Fixed

- bump version to 0.11.0

## [0.10.3](https://github.com/meh7an/rustigram/compare/rustigram-v0.10.2...rustigram-v0.10.3) - 2026-06-25

### Other

- Fix captions parse_mode

## [0.10.2](https://github.com/meh7an/rustigram/compare/rustigram-v0.10.1...rustigram-v0.10.2) - 2026-06-13

### Other

- add gateway for tma

## [0.10.1](https://github.com/meh7an/rustigram/compare/rustigram-v0.10.0...rustigram-v0.10.1) - 2026-06-13

### Fixed

- add version to website home

### Other

- release v0.10.0

## [0.10.0](https://github.com/meh7an/rustigram/compare/rustigram-v0.9.12...rustigram-v0.10.0) - 2026-06-13

### Other

- add rustigram-tma support for mini-apps

## [0.9.8](https://github.com/meh7an/rustigram/compare/rustigram-v0.9.7...rustigram-v0.9.8) - 2026-05-21

### Fixed

- update optional fields

### Other

- Revise README with new logo and documentation link
- add website for docs

## [0.9.7](https://github.com/meh7an/rustigram/compare/rustigram-v0.9.6...rustigram-v0.9.7) - 2026-05-07

### Added

- [**breaking**] live photos and media support in polls
