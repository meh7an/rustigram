# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Completes Bot API 10.2 coverage and adds a conformance test suite that checks the
crate against a committed snapshot of the spec. Nine defects were found and
fixed; four of them silently sent or dropped data with no error anywhere.

### Breaking

Four changes require edits at the call site. Each fixes behaviour that was
already broken.

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

- `send_video` and `send_animation` gain `.show_caption_above_media()` and
  `.has_spoiler()`; the five captioned media builders gain `.caption_entities()`.
  All are parameters the Bot API accepts and no setter reached.
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

Bot API coverage is now complete: 185 of 185 methods, 388 types, 1740 fields.
The suite grew from 119 to 254 tests and runs offline.

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
