//! The three upload paths that are not media sends.
//!
//! `media_send_paths` covers the ten builders that take an `InputFile` as their
//! subject and can send it either way. Three more methods build a multipart form
//! and are not media sends at all — `setMyProfilePhoto` always uploads,
//! `setStickerSetThumbnail` and `setWebhook` upload only when given bytes — and
//! nothing exercised any of them.
//!
//! That is the same unlit space the two multipart defects came from: seven
//! builders dropping `protect_content` and `reply_parameters`, then
//! `send_live_photo` dropping `has_spoiler`. Each was hand-written form-building
//! code that no test had ever run.
//!
//! These are small surfaces, so the tests are correspondingly small. The point
//! is that the code executes at all, and that what the caller sets arrives.

mod mock;

use mock::fixtures;
use rustigram_types::file::{InputFile, InputProfilePhoto, InputProfilePhotoStatic};

/// A profile photo is uploaded as multipart with the JSON descriptor attached.
///
/// `setMyProfilePhoto` has no JSON path — Telegram requires the file itself, so
/// the form is built on every call. It also takes a typed `InputProfilePhoto`
/// now, which the crate serialises; before, callers passed a pre-serialised
/// string and this code never ran under test.
#[tokio::test]
async fn setting_a_profile_photo_uploads_a_multipart_form() {
    let (server, client) = mock::spawn().await;
    mock::mount_catch_all(&server).await;

    let _ = client
        .set_my_profile_photo(InputProfilePhoto::Static(InputProfilePhotoStatic {
            photo: "attach://photo".to_owned(),
        }))
        .await;

    let request = mock::only_request(&server).await;
    let content_type = request
        .headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    assert!(
        content_type.starts_with("multipart/form-data"),
        "a profile photo must go out as multipart, got `{content_type}`"
    );

    let fields = mock::multipart_field_names(&request);
    assert!(
        fields.iter().any(|f| f == "photo"),
        "the photo descriptor never reached the form: {fields:?}"
    );
}

/// A sticker set thumbnail sent as bytes takes the multipart path.
#[tokio::test]
async fn a_sticker_set_thumbnail_uploads_its_bytes() {
    let (server, client) = mock::spawn().await;
    mock::mount_catch_all(&server).await;

    let _ = client
        .set_sticker_set_thumbnail("set_name", 7, "static")
        .thumbnail(fixtures::uploaded_file())
        .await;

    let request = mock::only_request(&server).await;
    let fields = mock::multipart_field_names(&request);
    for expected in ["name", "user_id", "format", "thumbnail"] {
        assert!(
            fields.iter().any(|f| f == expected),
            "`{expected}` never reached the form: {fields:?}"
        );
    }
}

/// The same thumbnail sent by `file_id` takes the JSON path, carrying the same
/// fields.
///
/// Both paths exist, so both can drift — the property that failed for every
/// media builder.
#[tokio::test]
async fn a_sticker_set_thumbnail_by_file_id_carries_the_same_fields() {
    let (server, client) = mock::spawn().await;
    mock::mount_catch_all(&server).await;

    let _ = client
        .set_sticker_set_thumbnail("set_name", 7, "static")
        .thumbnail(fixtures::input_file())
        .await;

    let body = mock::json_body(&mock::only_request(&server).await);
    for expected in ["name", "user_id", "format", "thumbnail"] {
        assert!(
            body.get(expected).is_some(),
            "`{expected}` is missing from the JSON body: {body}"
        );
    }
}

/// A webhook certificate uploaded as bytes reaches the form, with the options.
///
/// `setWebhook` builds its form by hand and carries options beside the
/// certificate — `secret_token` among them, which is the one that silently
/// never reached the server once already.
#[tokio::test]
async fn a_webhook_certificate_uploads_with_its_options() {
    let (server, client) = mock::spawn().await;
    mock::mount_catch_all(&server).await;

    let _ = client
        .set_webhook("https://example.test/hook")
        .certificate(InputFile::Bytes {
            filename: "cert.pem".to_owned(),
            data: b"-----BEGIN CERTIFICATE-----".to_vec(),
            mime_type: "application/x-pem-file".to_owned(),
        })
        .secret_token("s3cret")
        .max_connections(40)
        .drop_pending_updates(true)
        .await;

    let request = mock::only_request(&server).await;
    let content_type = request
        .headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    assert!(
        content_type.starts_with("multipart/form-data"),
        "a certificate upload must go out as multipart, got `{content_type}`"
    );

    let fields = mock::multipart_field_names(&request);
    for expected in [
        "url",
        "certificate",
        "secret_token",
        "max_connections",
        "drop_pending_updates",
    ] {
        assert!(
            fields.iter().any(|f| f == expected),
            "`{expected}` was set on the builder and never reached the form: {fields:?}"
        );
    }
}

/// `setWebhook` without a certificate stays on the JSON path and keeps its
/// options.
#[tokio::test]
async fn a_webhook_without_a_certificate_carries_the_same_options() {
    let (server, client) = mock::spawn().await;
    mock::mount_catch_all(&server).await;

    let _ = client
        .set_webhook("https://example.test/hook")
        .secret_token("s3cret")
        .max_connections(40)
        .drop_pending_updates(true)
        .await;

    let body = mock::json_body(&mock::only_request(&server).await);
    for expected in [
        "url",
        "secret_token",
        "max_connections",
        "drop_pending_updates",
    ] {
        assert!(
            body.get(expected).is_some(),
            "`{expected}` is missing from the JSON body: {body}"
        );
    }
}
