//! Every builder puts a well-formed request on the wire.
//!
//! The largest gap this suite set out to close: before it, 16% of builders had
//! any assertion at all, and the five silently-dropped multipart options lived
//! in exactly that unlit space. A builder can be fully covered by every
//! type-level check here and still send nothing Telegram can use.
//!
//! # How coverage is established
//!
//! Not by matching Rust names against spec names, which only proves a naming
//! convention. The sweep calls every entry point the client exposes and reads
//! the *paths the mock server actually received*. A path is the Bot API method
//! name, so the set of methods reached is observed rather than inferred — and a
//! builder that posts to the wrong endpoint shows up as a missing method rather
//! than passing on the strength of its Rust identifier.
//!
//! Each call passes required arguments only and sets no optional ones, which is
//! what makes the three properties below checkable in one pass.

mod mock;

use mock::fixtures;
use rustigram_api::methods::chat_management::JoinRequestResult;
use rustigram_api::methods::sending::ChatAction;
use rustigram_api::BotClient;
use rustigram_types::sticker::StickerFormat;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use wiremock::Request;

// ─── The spec snapshot ───────────────────────────────────────────────────────

/// The snapshot lives with the type-conformance suite that generated it, and is
/// read here by relative path rather than copied. One committed spec, one place
/// to update when Telegram ships a version.
const SNAPSHOT: &str = include_str!("../../rustigram-types/tests/spec/bot-api-10.2.json");

#[derive(serde::Deserialize)]
struct Spec {
    methods: BTreeMap<String, BTreeMap<String, SpecParam>>,
}

#[derive(serde::Deserialize)]
struct SpecParam(String, u8);

impl SpecParam {
    fn optional(&self) -> bool {
        self.1 == 1
    }
}

fn spec() -> Spec {
    let spec: Spec = serde_json::from_str(SNAPSHOT).expect("the spec snapshot parses");
    assert!(
        !spec.methods.is_empty(),
        "the snapshot parsed but lists no methods; every test here would \
         vacuously pass"
    );
    spec
}

// ─── The sweep ───────────────────────────────────────────────────────────────

#[rustfmt::skip]
async fn call_group_00(client: &BotClient) {
    let _ = client.add_sticker_to_set(1_i64, "x", fixtures::sticker()).await;
    let _ = client.answer_callback_query("x").await;
    let _ = client.answer_chat_join_request_query("x", JoinRequestResult::Approve).await;
    let _ = client.answer_guest_query("x", fixtures::inline_result()).await;
    let _ = client.answer_inline_query("x", vec![fixtures::inline_result()]).await;
    let _ = client.answer_pre_checkout_query("x", true).await;
    let _ = client.answer_shipping_query("x", true).await;
    let _ = client.answer_web_app_query("x", fixtures::inline_result()).await;
    let _ = client.approve_chat_join_request(1_i64, 1_i64).await;
    let _ = client.approve_suggested_post(1_i64, 1_i64).await;
    let _ = client.ban_chat_member(1_i64, 1_i64).await;
    let _ = client.ban_chat_sender_chat(1_i64, 1_i64).await;
    let _ = client.close().await;
    let _ = client.close_forum_topic(1_i64, 1_i64).await;
    let _ = client.close_general_forum_topic(1_i64).await;
    let _ = client.convert_gift_to_stars("x", "x").await;
}

#[rustfmt::skip]
async fn call_group_01(client: &BotClient) {
    let _ = client.copy_message(1_i64, 1_i64, 1_i64).await;
    let _ = client.copy_messages(1_i64, 1_i64, vec![1_i64]).await;
    let _ = client.create_chat_invite_link(1_i64).await;
    let _ = client.create_chat_subscription_invite_link(1_i64, 1_u32, 1_u32).await;
    let _ = client.create_forum_topic(1_i64, "x").await;
    let _ = client.create_invoice_link("x", "x", "x", "x", vec![fixtures::labeled_price()]).await;
    let _ = client.create_new_sticker_set(1_i64, "x", "x", vec![fixtures::sticker()]).await;
    let _ = client.decline_chat_join_request(1_i64, 1_i64).await;
    let _ = client.decline_suggested_post(1_i64, 1_i64).await;
    let _ = client.delete_all_message_reactions(1_i64).await;
    let _ = client.delete_business_messages("x", vec![1_i64]).await;
    let _ = client.delete_chat_photo(1_i64).await;
    let _ = client.delete_chat_sticker_set(1_i64).await;
    let _ = client.delete_ephemeral_message(1_i64, 1_i64, 1_i64).await;
    let _ = client.delete_forum_topic(1_i64, 1_i64).await;
    let _ = client.delete_message(1_i64, 1_i64).await;
}

#[rustfmt::skip]
async fn call_group_02(client: &BotClient) {
    let _ = client.delete_message_reaction(1_i64, 1_i64).await;
    let _ = client.delete_messages(1_i64, vec![1_i64]).await;
    let _ = client.delete_my_commands().await;
    let _ = client.delete_sticker_from_set("x").await;
    let _ = client.delete_sticker_set("x").await;
    let _ = client.delete_story("x", 1_i64).await;
    let _ = client.delete_webhook().await;
    let _ = client.edit_chat_invite_link(1_i64, "x").await;
    let _ = client.edit_chat_subscription_invite_link(1_i64, "x").await;
    let _ = client.edit_ephemeral_message_caption(1_i64, 1_i64, 1_i64).await;
    let _ = client.edit_ephemeral_message_media(1_i64, 1_i64, 1_i64, fixtures::input_media()).await;
    let _ = client.edit_ephemeral_message_reply_markup(1_i64, 1_i64, 1_i64).await;
    let _ = client.edit_ephemeral_message_text(1_i64, 1_i64, 1_i64, "x").await;
    let _ = client.edit_forum_topic(1_i64, 1_i64).await;
    let _ = client.edit_general_forum_topic(1_i64, "x").await;
    let _ = client.edit_inline_message_caption("x").await;
}

#[rustfmt::skip]
async fn call_group_03(client: &BotClient) {
    let _ = client.edit_inline_message_live_location("x", 1.0_f64, 1.0_f64).await;
    let _ = client.edit_inline_message_media("x", fixtures::input_media()).await;
    let _ = client.edit_inline_message_reply_markup("x").await;
    let _ = client.edit_inline_message_rich_text("x", fixtures::rich_message()).await;
    let _ = client.edit_inline_message_text("x", "x").await;
    let _ = client.edit_message_caption(1_i64, 1_i64).await;
    let _ = client.edit_message_checklist("x", 1_i64, 1_i64, fixtures::checklist()).await;
    let _ = client.edit_message_live_location(1_i64, 1_i64, 1.0_f64, 1.0_f64).await;
    let _ = client.edit_message_media(1_i64, 1_i64, fixtures::input_media()).await;
    let _ = client.edit_message_reply_markup(1_i64, 1_i64).await;
    let _ = client.edit_message_rich_text(1_i64, 1_i64, fixtures::rich_message()).await;
    let _ = client.edit_message_text(1_i64, 1_i64, "x").await;
    let _ = client.edit_story("x", 1_i64, fixtures::story_content()).await;
    let _ = client.edit_user_star_subscription(1_i64, "x", true).await;
    let _ = client.export_chat_invite_link(1_i64).await;
    let _ = client.forward_message(1_i64, 1_i64, 1_i64).await;
}

#[rustfmt::skip]
async fn call_group_04(client: &BotClient) {
    let _ = client.forward_messages(1_i64, 1_i64, vec![1_i64]).await;
    let _ = client.get_available_gifts().await;
    let _ = client.get_business_account_gifts("x").await;
    let _ = client.get_business_account_star_balance("x").await;
    let _ = client.get_business_connection("x").await;
    let _ = client.get_chat(1_i64).await;
    let _ = client.get_chat_administrators(1_i64).await;
    let _ = client.get_chat_gifts(1_i64).await;
    let _ = client.get_chat_member(1_i64, 1_i64).await;
    let _ = client.get_chat_member_count(1_i64).await;
    let _ = client.get_chat_menu_button().await;
    let _ = client.get_custom_emoji_stickers(vec!["x"]).await;
    let _ = client.get_file("x").await;
    let _ = client.get_forum_topic_icon_stickers().await;
    let _ = client.get_game_high_scores(1_i64).await;
    let _ = client.get_managed_bot_access_settings(1_i64).await;
}

#[rustfmt::skip]
async fn call_group_05(client: &BotClient) {
    let _ = client.get_managed_bot_token(1_i64).await;
    let _ = client.get_me().await;
    let _ = client.get_my_commands().await;
    let _ = client.get_my_default_administrator_rights().await;
    let _ = client.get_my_description().await;
    let _ = client.get_my_name().await;
    let _ = client.get_my_short_description().await;
    let _ = client.get_my_star_balance().await;
    let _ = client.get_star_transactions().await;
    let _ = client.get_sticker_set("x").await;
    let _ = client.get_updates().await;
    let _ = client.get_user_chat_boosts(1_i64, 1_i64).await;
    let _ = client.get_user_gifts(1_i64).await;
    let _ = client.get_user_personal_chat_messages(1_i64, 1_u32).await;
    let _ = client.get_user_profile_audios(1_i64).await;
    let _ = client.get_user_profile_photos(1_i64).await;
}

#[rustfmt::skip]
async fn call_group_06(client: &BotClient) {
    let _ = client.get_webhook_info().await;
    let _ = client.gift_premium_subscription(1_i64, 1_u32, 1_u32).await;
    let _ = client.hide_general_forum_topic(1_i64).await;
    let _ = client.leave_chat(1_i64).await;
    let _ = client.log_out().await;
    let _ = client.pin_chat_message(1_i64, 1_i64).await;
    let _ = client.post_story("x", fixtures::story_content(), 1_u32).await;
    let _ = client.promote_chat_member(1_i64, 1_i64).await;
    let _ = client.read_business_message("x", 1_i64, 1_i64).await;
    let _ = client.refund_star_payment(1_i64, "x").await;
    let _ = client.remove_business_account_profile_photo("x").await;
    let _ = client.remove_chat_verification(1_i64).await;
    let _ = client.remove_my_profile_photo().await;
    let _ = client.remove_user_verification(1_i64).await;
    let _ = client.reopen_forum_topic(1_i64, 1_i64).await;
    let _ = client.reopen_general_forum_topic(1_i64).await;
}

#[rustfmt::skip]
async fn call_group_07(client: &BotClient) {
    let _ = client.replace_managed_bot_token(1_i64).await;
    let _ = client.replace_sticker_in_set(1_i64, "x", "x", fixtures::sticker()).await;
    let _ = client.repost_story("x", 1_i64, 1_i64, 1_u32).await;
    let _ = client.restrict_chat_member(1_i64, 1_i64, Default::default()).await;
    let _ = client.revoke_chat_invite_link(1_i64, "x").await;
    let _ = client.save_prepared_inline_message(1_i64, fixtures::inline_result()).await;
    let _ = client.save_prepared_keyboard_button(1_i64, fixtures::keyboard_button()).await;
    let _ = client.send_animation(1_i64, fixtures::input_file()).await;
    let _ = client.send_audio(1_i64, fixtures::input_file()).await;
    let _ = client.send_chat_action(1_i64, ChatAction::Typing).await;
    let _ = client.send_chat_join_request_web_app("x", "x").await;
    let _ = client.send_checklist("x", 1_i64, fixtures::checklist()).await;
    let _ = client.send_contact(1_i64, "x", "x").await;
    let _ = client.send_dice(1_i64).await;
    let _ = client.send_document(1_i64, fixtures::input_file()).await;
    let _ = client.send_game(1_i64, "x").await;
}

#[rustfmt::skip]
async fn call_group_08(client: &BotClient) {
    let _ = client.send_gift("x").await;
    let _ = client.send_invoice(1_i64, "x", "x", "x", "x", vec![fixtures::labeled_price()]).await;
    let _ = client.send_live_photo(1_i64, fixtures::input_file(), fixtures::input_file()).await;
    let _ = client.send_location(1_i64, 1.0_f64, 1.0_f64).await;
    let _ = client.send_media_group(1_i64, vec![fixtures::input_media()]).await;
    let _ = client.send_message(1_i64, "x").await;
    let _ = client.send_message_draft(1_i64, 1_i64, "x").await;
    let _ = client.send_paid_media(1_i64, 1_u32, vec![fixtures::paid_media()]).await;
    let _ = client.send_photo(1_i64, fixtures::input_file()).await;
    let _ = client.send_poll(1_i64, "x", vec![fixtures::poll_option()]).await;
    let _ = client.send_rich_message(1_i64, fixtures::rich_message()).await;
    let _ = client.send_rich_message_draft(1_i64, 1_i64, fixtures::rich_message()).await;
    let _ = client.send_sticker(1_i64, fixtures::input_file()).await;
    let _ = client.send_venue(1_i64, 1.0_f64, 1.0_f64, "x", "x").await;
    let _ = client.send_video(1_i64, fixtures::input_file()).await;
    let _ = client.send_video_note(1_i64, fixtures::input_file()).await;
}

#[rustfmt::skip]
async fn call_group_09(client: &BotClient) {
    let _ = client.send_voice(1_i64, fixtures::input_file()).await;
    let _ = client.set_business_account_bio("x", None).await;
    let _ = client.set_business_account_gift_settings("x", true, Default::default()).await;
    let _ = client.set_business_account_name("x", "x", None).await;
    let _ = client.set_business_account_profile_photo("x", fixtures::profile_photo()).await;
    let _ = client.set_business_account_username("x", None).await;
    let _ = client.set_chat_administrator_custom_title(1_i64, 1_i64, "x").await;
    let _ = client.set_chat_description(1_i64).await;
    let _ = client.set_chat_member_tag(1_i64, 1_i64).await;
    let _ = client.set_chat_menu_button().await;
    let _ = client.set_chat_permissions(1_i64, Default::default()).await;
    let _ = client.set_chat_photo(1_i64, fixtures::input_file()).await;
    let _ = client.set_chat_sticker_set(1_i64, "x").await;
    let _ = client.set_chat_title(1_i64, "x").await;
    let _ = client.set_custom_emoji_sticker_set_thumbnail("x").await;
    let _ = client.set_game_score(1_i64, 1_u32).await;
}

#[rustfmt::skip]
async fn call_group_10(client: &BotClient) {
    let _ = client.set_managed_bot_access_settings(1_i64, true).await;
    let _ = client.set_message_reaction(1_i64, 1_i64).await;
    let _ = client.set_my_commands(vec![fixtures::bot_command()]).await;
    let _ = client.set_my_default_administrator_rights().await;
    let _ = client.set_my_description().await;
    let _ = client.set_my_name().await;
    let _ = client.set_my_profile_photo(fixtures::profile_photo()).await;
    let _ = client.set_my_short_description().await;
    let _ = client.set_passport_data_errors(1_i64, vec![fixtures::passport_error()]).await;
    let _ = client.set_sticker_emoji_list("x", vec!["x"]).await;
    let _ = client.set_sticker_keywords("x").await;
    let _ = client.set_sticker_mask_position("x").await;
    let _ = client.set_sticker_position_in_set("x", 1_u32).await;
    let _ = client.set_sticker_set_thumbnail("x", 1_i64, "x").await;
    let _ = client.set_sticker_set_title("x", "x").await;
    let _ = client.set_user_emoji_status(1_i64).await;
}

#[rustfmt::skip]
async fn call_group_11(client: &BotClient) {
    let _ = client.set_webhook("x").await;
    let _ = client.stop_inline_message_live_location("x").await;
    let _ = client.stop_message_live_location(1_i64, 1_i64).await;
    let _ = client.stop_poll(1_i64, 1_i64).await;
    let _ = client.transfer_business_account_stars("x", 1_u64).await;
    let _ = client.transfer_gift("x", "x", 1_i64).await;
    let _ = client.unban_chat_member(1_i64, 1_i64).await;
    let _ = client.unban_chat_sender_chat(1_i64, 1_i64).await;
    let _ = client.unhide_general_forum_topic(1_i64).await;
    let _ = client.unpin_all_chat_messages(1_i64).await;
    let _ = client.unpin_all_forum_topic_messages(1_i64, 1_i64).await;
    let _ = client.unpin_all_general_forum_topic_messages(1_i64).await;
    let _ = client.unpin_chat_message(1_i64).await;
    let _ = client.upgrade_gift("x", "x").await;
    let _ = client.upload_sticker_file(1_i64, fixtures::input_file(), StickerFormat::Static).await;
    let _ = client.verify_chat(1_i64).await;
}

#[rustfmt::skip]
async fn call_group_12(client: &BotClient) {
    let _ = client.verify_user(1_i64).await;
}

/// Calls every entry point on the client, with required arguments only.
///
/// Split into groups and boxed rather than written as one long function: each
/// `.await` contributes its future to the enclosing state machine, and 193 of
/// them in a single `async fn` overflows the stack in a debug build before the
/// first request goes out. Boxing puts each group's state on the heap.
///
/// Results are discarded on purpose. The mock answers every path with an empty
/// result, so most calls fail to decode — expected and irrelevant, since the
/// request is recorded before the response is parsed, and the request is the
/// subject.
async fn call_every_builder(client: &BotClient) {
    Box::pin(call_group_00(client)).await;
    Box::pin(call_group_01(client)).await;
    Box::pin(call_group_02(client)).await;
    Box::pin(call_group_03(client)).await;
    Box::pin(call_group_04(client)).await;
    Box::pin(call_group_05(client)).await;
    Box::pin(call_group_06(client)).await;
    Box::pin(call_group_07(client)).await;
    Box::pin(call_group_08(client)).await;
    Box::pin(call_group_09(client)).await;
    Box::pin(call_group_10(client)).await;
    Box::pin(call_group_11(client)).await;
    Box::pin(call_group_12(client)).await;
}

/// Runs the sweep and returns every request the server saw.
///
/// On a dedicated thread with a large stack. Every `.await` contributes its
/// future to the enclosing state machine, and 193 of them — several carrying
/// wide parameter structs — overflow a default 2 MB stack in a debug build
/// before the first request goes out. This is a property of calling every
/// builder in one function, not of the builders: a bot calls one at a time.
fn sweep() -> Vec<Request> {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(|| {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("a test runtime starts")
                .block_on(sweep_inner())
        })
        .expect("the sweep thread starts")
        .join()
        .expect("the sweep thread finishes")
}

async fn sweep_inner() -> Vec<Request> {
    let (server, client) = mock::spawn().await;
    mock::mount_catch_all(&server).await;
    call_every_builder(&client).await;
    let requests = mock::requests(&server).await;
    assert!(
        requests.len() > 150,
        "the sweep only produced {} requests; it is not calling the builders",
        requests.len()
    );
    requests
}

/// The JSON body of a request, or `None` if it went out as multipart.
fn body_of(request: &Request) -> Option<Value> {
    serde_json::from_slice(&request.body).ok()
}

// ─── The properties ──────────────────────────────────────────────────────────

/// Every method in the spec is reachable from `BotClient`, proven by calling it.
///
/// The coverage suite asserts an entry point *exists* for each method by name.
/// This asserts one *works*: it builds a request and sends it to the endpoint
/// the spec names. A builder posting to `sendmessage` passes there and fails
/// here.
#[test]
fn every_spec_method_is_reached_by_calling_a_builder() {
    let spec = spec();
    let reached: BTreeSet<String> = sweep().iter().map(mock::api_method_of).collect();

    let unreached: Vec<&String> = spec
        .methods
        .keys()
        .filter(|name| !reached.contains(*name))
        .collect();
    assert!(
        unreached.is_empty(),
        "{} of {} spec method(s) were never sent by any builder:\n  {}",
        unreached.len(),
        spec.methods.len(),
        unreached
            .iter()
            .map(|m| m.as_str())
            .collect::<Vec<_>>()
            .join("\n  ")
    );

    let unknown: Vec<&String> = reached
        .iter()
        .filter(|name| !spec.methods.contains_key(*name))
        .collect();
    assert!(
        unknown.is_empty(),
        "{} request(s) went to an endpoint the spec does not define, which is a \
         typo Telegram would answer with 404:\n  {unknown:?}",
        unknown.len()
    );
}

/// Every request carries the parameters the spec marks required.
///
/// The builders take these as constructor arguments, so a required parameter
/// missing from the body means an argument was accepted and then dropped on the
/// way to the wire — the multipart bug's exact shape, in the general case.
#[test]
fn every_request_carries_its_required_parameters() {
    let spec = spec();
    let mut missing = Vec::new();

    for request in sweep() {
        let method = mock::api_method_of(&request);
        let Some(params) = spec.methods.get(&method) else {
            continue; // reported by the reachability test
        };
        let sent: BTreeSet<String> = match body_of(&request) {
            Some(Value::Object(body)) => body.keys().cloned().collect(),
            // A multipart request states its fields in the part headers.
            _ => mock::multipart_field_names(&request).into_iter().collect(),
        };
        for (name, param) in params {
            if !param.optional() && !sent.contains(name) {
                missing.push(format!("  {method}.{name}"));
            }
        }
    }

    missing.sort();
    missing.dedup();
    assert!(
        missing.is_empty(),
        "{} required parameter(s) never reached the wire, though the builder \
         accepts them:\n{}",
        missing.len(),
        missing.join("\n")
    );
}

/// No unset optional parameter is sent as an explicit `null`.
///
/// Telegram distinguishes an omitted field from a null one for several methods —
/// notably the edit* family, where a null clears a value that omitting it would
/// leave alone. Since the sweep sets no optional parameters, any null in a body
/// is a `skip_serializing_if` that was left off.
#[test]
fn no_request_sends_a_null_for_an_unset_parameter() {
    let mut nulls = Vec::new();

    for request in sweep() {
        let method = mock::api_method_of(&request);
        let Some(Value::Object(body)) = body_of(&request) else {
            continue;
        };
        for (key, value) in &body {
            if value.is_null() {
                nulls.push(format!("  {method}.{key}"));
            }
        }
    }

    nulls.sort();
    nulls.dedup();
    assert!(
        nulls.is_empty(),
        "{} parameter(s) were sent as an explicit null while unset. Telegram \
         reads a null as \"clear this\" where an absent field means \"leave it\" — \
         add #[serde(skip_serializing_if = \"Option::is_none\")]:\n{}",
        nulls.len(),
        nulls.join("\n")
    );
}
