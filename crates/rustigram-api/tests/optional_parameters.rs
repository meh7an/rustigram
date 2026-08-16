//! Every optional parameter a builder accepts actually reaches Telegram.
//!
//! `request_construction` calls each builder with required arguments only, and
//! `builder_reachability` proves a setter *exists*. Neither shows that a setter
//! does anything: one writing the wrong field, or a params field whose
//! serialisation is suppressed, passes both while silently discarding whatever
//! the caller set.
//!
//! That is not hypothetical here. Two defects of exactly this shape have already
//! shipped — seven media builders dropping `protect_content` and
//! `reply_parameters` on byte uploads, and `send_live_photo` dropping
//! `has_spoiler` — and both were invisible until something inspected the bytes
//! on the wire.
//!
//! # The property
//!
//! With **every** setter called, every optional parameter the spec defines for
//! that method appears in the request. Checked against the request the mock
//! server received, so it is a statement about what left the process rather than
//! about what the builder stored.

mod mock;

use mock::fixtures;
use rustigram_api::methods::chat_management::JoinRequestResult;
use rustigram_api::methods::sending::ChatAction;
use rustigram_api::BotClient;
use rustigram_types::message::ParseMode;
use rustigram_types::sticker::StickerFormat;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;
use wiremock::Request;

const SNAPSHOT: &str = include_str!("../../rustigram-types/tests/spec/bot-api-10.2.json");

#[derive(serde::Deserialize)]
struct Spec {
    methods: BTreeMap<String, BTreeMap<String, SpecParam>>,
}

#[derive(serde::Deserialize)]
struct SpecParam(#[allow(dead_code)] String, u8);

impl SpecParam {
    fn optional(&self) -> bool {
        self.1 == 1
    }
}

/// Parameters that cannot be set by a plain setter call, with the reason.
///
/// Both come from setters this sweep deliberately does not call: one clears a
/// value rather than setting it, and one takes two arguments at once. Neither is
/// unreachable — they are simply not reachable in the uniform way this sweep
/// drives every other setter.
const NOT_SET_BY_SWEEP: &[(&str, &str, &str)] = &[
    ("copyMessages", "remove_caption", "set by `remove_caption()`, which clears a value rather than setting one; the sweep skips every `remove_*`/`clear_*` setter so it does not undo the fields it is checking"),
    (
        "getGameHighScores",
        "chat_id",
        "set by `chat_message(chat, message)`, which takes two arguments",
    ),
    (
        "getGameHighScores",
        "message_id",
        "same `chat_message` setter",
    ),
    ("setGameScore", "chat_id", "same `chat_message` setter"),
    ("setGameScore", "message_id", "same `chat_message` setter"),
];

#[rustfmt::skip]
async fn set_group_00(client: &BotClient) {
    let _ = client.add_sticker_to_set(1_i64, "x", fixtures::sticker()).await;
    let _ = client.answer_callback_query("x").text("x").show_alert(true).url("x").cache_time(1_u32).await;
    let _ = client.answer_chat_join_request_query("x", JoinRequestResult::Approve).await;
    let _ = client.answer_guest_query("x", fixtures::inline_result()).await;
    let _ = client.answer_inline_query("x", vec![fixtures::inline_result()]).cache_time(1_u32).is_personal(true).next_offset("x").button(fixtures::results_button()).await;
    let _ = client.answer_pre_checkout_query("x", true).error_message("x").await;
    let _ = client.answer_shipping_query("x", true).shipping_options(Vec::new()).error_message("x").await;
    let _ = client.answer_web_app_query("x", fixtures::inline_result()).await;
}

#[rustfmt::skip]
async fn set_group_01(client: &BotClient) {
    let _ = client.approve_chat_join_request(1_i64, 1_i64).await;
    let _ = client.approve_suggested_post(1_i64, 1_i64).send_date(1_i64).await;
    let _ = client.ban_chat_member(1_i64, 1_i64).until_date(1_i64).revoke_messages(true).await;
    let _ = client.ban_chat_sender_chat(1_i64, 1_i64).await;
    let _ = client.close().await;
    let _ = client.close_forum_topic(1_i64, 1_i64).await;
    let _ = client.close_general_forum_topic(1_i64).await;
    let _ = client.convert_gift_to_stars("x", "x").await;
}

#[rustfmt::skip]
async fn set_group_02(client: &BotClient) {
    let _ = client.copy_message(1_i64, 1_i64, 1_i64).message_thread_id(1_i64).direct_messages_topic_id(1_i64).video_start_timestamp(1_i64).caption("x").parse_mode(ParseMode::HTML).disable_notification(true).reply_markup(fixtures::reply_markup()).allow_paid_broadcast(true).message_effect_id("x").suggested_post_parameters(fixtures::suggested_post()).caption_entities(Vec::new()).show_caption_above_media(true).protect_content(true).reply_parameters(fixtures::reply_to(7)).await;
    let _ = client.copy_messages(1_i64, 1_i64, vec![1_i64]).message_thread_id(1_i64).direct_messages_topic_id(1_i64).disable_notification(true).protect_content(true).await;
    let _ = client.create_chat_invite_link(1_i64).name("x").expire_date(1_i64).member_limit(1_u32).creates_join_request(true).await;
    let _ = client.create_chat_subscription_invite_link(1_i64, 1_u32, 1_u32).name("x").await;
    let _ = client.create_forum_topic(1_i64, "x").icon_color(1_u32).icon_custom_emoji_id("x").await;
    let _ = client.create_invoice_link("x", "x", "x", "x", vec![fixtures::labeled_price()]).business_connection_id("x").provider_token("x").subscription_period(1_i64).max_tip_amount(1_i64).photo_url("x").need_name(true).need_phone_number(true).need_email(true).need_shipping_address(true).send_phone_number_to_provider(true).send_email_to_provider(true).is_flexible(true).suggested_tip_amounts(vec![1_i64]).provider_data("x").photo_size(1_i64).photo_width(1_i64).photo_height(1_i64).await;
    let _ = client.create_new_sticker_set(1_i64, "x", "x", vec![fixtures::sticker()]).sticker_type(fixtures::sticker_type()).needs_repainting(true).await;
    let _ = client.decline_chat_join_request(1_i64, 1_i64).await;
}

#[rustfmt::skip]
async fn set_group_03(client: &BotClient) {
    let _ = client.decline_suggested_post(1_i64, 1_i64).comment("x").await;
    let _ = client.delete_all_message_reactions(1_i64).user_id(1_i64).actor_chat_id(1_i64).await;
    let _ = client.delete_business_messages("x", vec![1_i64]).await;
    let _ = client.delete_chat_photo(1_i64).await;
    let _ = client.delete_chat_sticker_set(1_i64).await;
    let _ = client.delete_ephemeral_message(1_i64, 1_i64, 1_i64).await;
    let _ = client.delete_forum_topic(1_i64, 1_i64).await;
    let _ = client.delete_message(1_i64, 1_i64).await;
}

#[rustfmt::skip]
async fn set_group_04(client: &BotClient) {
    let _ = client.delete_message_reaction(1_i64, 1_i64).user_id(1_i64).actor_chat_id(1_i64).await;
    let _ = client.delete_messages(1_i64, vec![1_i64]).await;
    let _ = client.delete_my_commands().scope(fixtures::command_scope()).language_code("x").await;
    let _ = client.delete_sticker_from_set("x").await;
    let _ = client.delete_sticker_set("x").await;
    let _ = client.delete_story("x", 1_i64).await;
    let _ = client.delete_webhook().drop_pending_updates(true).await;
    let _ = client.edit_chat_invite_link(1_i64, "x").name("x").expire_date(1_i64).member_limit(1_u32).creates_join_request(true).await;
}

#[rustfmt::skip]
async fn set_group_05(client: &BotClient) {
    let _ = client.edit_chat_subscription_invite_link(1_i64, "x").name("x").await;
    let _ = client.edit_ephemeral_message_caption(1_i64, 1_i64, 1_i64).caption("x").parse_mode(ParseMode::HTML).caption_entities(Vec::new()).reply_markup(fixtures::inline_keyboard()).await;
    let _ = client.edit_ephemeral_message_media(1_i64, 1_i64, 1_i64, fixtures::input_media()).reply_markup(fixtures::inline_keyboard()).await;
    let _ = client.edit_ephemeral_message_reply_markup(1_i64, 1_i64, 1_i64).reply_markup(fixtures::inline_keyboard()).await;
    let _ = client.edit_ephemeral_message_text(1_i64, 1_i64, 1_i64, "x").parse_mode(ParseMode::HTML).entities(Vec::new()).reply_markup(fixtures::inline_keyboard()).link_preview_options(Default::default()).await;
    let _ = client.edit_forum_topic(1_i64, 1_i64).name("x").icon_custom_emoji_id("x").await;
    let _ = client.edit_general_forum_topic(1_i64, "x").await;
    let _ = client.edit_inline_message_caption("x").caption("x").parse_mode(ParseMode::HTML).show_caption_above_media(true).reply_markup(fixtures::inline_keyboard()).business_connection_id("x").caption_entities(Vec::new()).await;
}

#[rustfmt::skip]
async fn set_group_06(client: &BotClient) {
    let _ = client.edit_inline_message_live_location("x", 1.0_f64, 1.0_f64).live_period(1_u32).heading(1_u16).reply_markup(fixtures::inline_keyboard()).business_connection_id("x").horizontal_accuracy(1.0_f64).proximity_alert_radius(1_u32).await;
    let _ = client.edit_inline_message_media("x", fixtures::input_media()).business_connection_id("x").reply_markup(fixtures::inline_keyboard()).await;
    let _ = client.edit_inline_message_reply_markup("x").reply_markup(fixtures::inline_keyboard()).business_connection_id("x").await;
    let _ = client.edit_inline_message_rich_text("x", fixtures::rich_message()).parse_mode(ParseMode::HTML).entities(Vec::new()).link_preview_options(Default::default()).reply_markup(fixtures::inline_keyboard()).business_connection_id("x").await;
    let _ = client.edit_inline_message_text("x", "x").parse_mode(ParseMode::HTML).entities(Vec::new()).link_preview_options(Default::default()).reply_markup(fixtures::inline_keyboard()).business_connection_id("x").await;
    let _ = client.edit_message_caption(1_i64, 1_i64).caption("x").parse_mode(ParseMode::HTML).show_caption_above_media(true).reply_markup(fixtures::inline_keyboard()).business_connection_id("x").caption_entities(Vec::new()).await;
    let _ = client.edit_message_checklist("x", 1_i64, 1_i64, fixtures::checklist()).reply_markup(fixtures::inline_keyboard()).await;
    let _ = client.edit_message_live_location(1_i64, 1_i64, 1.0_f64, 1.0_f64).live_period(1_u32).heading(1_u16).reply_markup(fixtures::inline_keyboard()).business_connection_id("x").horizontal_accuracy(1.0_f64).proximity_alert_radius(1_u32).await;
}

#[rustfmt::skip]
async fn set_group_07(client: &BotClient) {
    let _ = client.edit_message_media(1_i64, 1_i64, fixtures::input_media()).business_connection_id("x").reply_markup(fixtures::inline_keyboard()).await;
    let _ = client.edit_message_reply_markup(1_i64, 1_i64).reply_markup(fixtures::inline_keyboard()).business_connection_id("x").await;
    let _ = client.edit_message_rich_text(1_i64, 1_i64, fixtures::rich_message()).parse_mode(ParseMode::HTML).entities(Vec::new()).link_preview_options(Default::default()).reply_markup(fixtures::inline_keyboard()).business_connection_id("x").await;
    let _ = client.edit_message_text(1_i64, 1_i64, "x").parse_mode(ParseMode::HTML).entities(Vec::new()).link_preview_options(Default::default()).reply_markup(fixtures::inline_keyboard()).business_connection_id("x").await;
    let _ = client.edit_story("x", 1_i64, fixtures::story_content()).caption("x").parse_mode(ParseMode::HTML).caption_entities(Vec::new()).areas(Vec::new()).await;
    let _ = client.edit_user_star_subscription(1_i64, "x", true).await;
    let _ = client.export_chat_invite_link(1_i64).await;
    let _ = client.forward_message(1_i64, 1_i64, 1_i64).message_thread_id(1_i64).direct_messages_topic_id(1_i64).video_start_timestamp(1_i64).disable_notification(true).protect_content(true).message_effect_id("x").suggested_post_parameters(fixtures::suggested_post()).await;
}

#[rustfmt::skip]
async fn set_group_08(client: &BotClient) {
    let _ = client.forward_messages(1_i64, 1_i64, vec![1_i64]).message_thread_id(1_i64).direct_messages_topic_id(1_i64).disable_notification(true).protect_content(true).await;
    let _ = client.get_available_gifts().await;
    let _ = client.get_business_account_gifts("x").exclude_unsaved(true).exclude_saved(true).exclude_unlimited(true).exclude_limited_upgradable(true).exclude_limited_non_upgradable(true).exclude_unique(true).exclude_from_blockchain(true).sort_by_price(true).offset("x").limit(1_u32).await;
    let _ = client.get_business_account_star_balance("x").await;
    let _ = client.get_business_connection("x").await;
    let _ = client.get_chat(1_i64).await;
    let _ = client.get_chat_administrators(1_i64).return_bots(true).await;
    let _ = client.get_chat_gifts(1_i64).exclude_unsaved(true).exclude_saved(true).exclude_unlimited(true).exclude_limited_upgradable(true).exclude_limited_non_upgradable(true).exclude_unique(true).exclude_from_blockchain(true).sort_by_price(true).offset("x").limit(1_u32).await;
}

#[rustfmt::skip]
async fn set_group_09(client: &BotClient) {
    let _ = client.get_chat_member(1_i64, 1_i64).await;
    let _ = client.get_chat_member_count(1_i64).await;
    let _ = client.get_chat_menu_button().chat_id(1_i64).await;
    let _ = client.get_custom_emoji_stickers(vec!["x"]).await;
    let _ = client.get_file("x").await;
    let _ = client.get_forum_topic_icon_stickers().await;
    let _ = client.get_game_high_scores(1_i64).inline_message_id("x").await;
    let _ = client.get_managed_bot_access_settings(1_i64).await;
}

#[rustfmt::skip]
async fn set_group_10(client: &BotClient) {
    let _ = client.get_managed_bot_token(1_i64).await;
    let _ = client.get_me().await;
    let _ = client.get_my_commands().scope(fixtures::command_scope()).language_code("x").await;
    let _ = client.get_my_default_administrator_rights().for_channels(true).await;
    let _ = client.get_my_description().language_code("x").await;
    let _ = client.get_my_name().language_code("x").await;
    let _ = client.get_my_short_description().language_code("x").await;
    let _ = client.get_my_star_balance().await;
}

#[rustfmt::skip]
async fn set_group_11(client: &BotClient) {
    let _ = client.get_star_transactions().offset(1_u32).limit(1_u32).await;
    let _ = client.get_sticker_set("x").await;
    let _ = client.get_updates().offset(1_i64).limit(1_u8).timeout(1_u32).allowed_updates(vec!["x"]).await;
    let _ = client.get_user_chat_boosts(1_i64, 1_i64).await;
    let _ = client.get_user_gifts(1_i64).exclude_unlimited(true).exclude_limited_upgradable(true).exclude_limited_non_upgradable(true).exclude_unique(true).exclude_from_blockchain(true).sort_by_price(true).offset("x").limit(1_u32).await;
    let _ = client.get_user_personal_chat_messages(1_i64, 1_u32).await;
    let _ = client.get_user_profile_audios(1_i64).offset(1_u32).limit(1_u8).await;
    let _ = client.get_user_profile_photos(1_i64).offset(1_u32).limit(1_u8).await;
}

#[rustfmt::skip]
async fn set_group_12(client: &BotClient) {
    let _ = client.get_webhook_info().await;
    let _ = client.gift_premium_subscription(1_i64, 1_u32, 1_u32).text("x").text_parse_mode(ParseMode::HTML).text_entities(Vec::new()).await;
    let _ = client.hide_general_forum_topic(1_i64).await;
    let _ = client.leave_chat(1_i64).await;
    let _ = client.log_out().await;
    let _ = client.pin_chat_message(1_i64, 1_i64).disable_notification(true).business_connection_id("x").await;
    let _ = client.post_story("x", fixtures::story_content(), 1_u32).caption("x").parse_mode(ParseMode::HTML).caption_entities(Vec::new()).areas(Vec::new()).post_to_chat_page(true).protect_content(true).await;
    let _ = client.promote_chat_member(1_i64, 1_i64).can_manage_chat(true).can_delete_messages(true).can_manage_video_chats(true).can_restrict_members(true).can_promote_members(true).can_change_info(true).can_invite_users(true).can_pin_messages(true).can_manage_topics(true).can_post_stories(true).can_manage_direct_messages(true).can_manage_tags(true).is_anonymous(true).can_post_messages(true).can_edit_messages(true).can_edit_stories(true).can_delete_stories(true).await;
}

#[rustfmt::skip]
async fn set_group_13(client: &BotClient) {
    let _ = client.read_business_message("x", 1_i64, 1_i64).await;
    let _ = client.refund_star_payment(1_i64, "x").await;
    let _ = client.remove_business_account_profile_photo("x").is_public(true).await;
    let _ = client.remove_chat_verification(1_i64).await;
    let _ = client.remove_my_profile_photo().await;
    let _ = client.remove_user_verification(1_i64).await;
    let _ = client.reopen_forum_topic(1_i64, 1_i64).await;
    let _ = client.reopen_general_forum_topic(1_i64).await;
}

#[rustfmt::skip]
async fn set_group_14(client: &BotClient) {
    let _ = client.replace_managed_bot_token(1_i64).await;
    let _ = client.replace_sticker_in_set(1_i64, "x", "x", fixtures::sticker()).await;
    let _ = client.repost_story("x", 1_i64, 1_i64, 1_u32).post_to_chat_page(true).protect_content(true).await;
    let _ = client.restrict_chat_member(1_i64, 1_i64, Default::default()).until_date(1_i64).use_independent_chat_permissions(true).await;
    let _ = client.revoke_chat_invite_link(1_i64, "x").await;
    let _ = client.save_prepared_inline_message(1_i64, fixtures::inline_result()).allow_user_chats(true).allow_bot_chats(true).allow_group_chats(true).allow_channel_chats(true).await;
    let _ = client.save_prepared_keyboard_button(1_i64, fixtures::keyboard_button()).await;
    let _ = client.send_animation(1_i64, fixtures::input_file()).business_connection_id("x").message_thread_id(1_i64).direct_messages_topic_id(1_i64).disable_notification(true).message_effect_id("x").protect_content(true).allow_paid_broadcast(true).reply_parameters(fixtures::reply_to(7)).reply_markup(fixtures::reply_markup()).suggested_post_parameters(fixtures::suggested_post()).receiver_user_id(1_i64).callback_query_id("x").caption("x").parse_mode(ParseMode::HTML).caption_entities(Vec::new()).show_caption_above_media(true).has_spoiler(true).duration(1_u32).width(1_u32).height(1_u32).thumbnail("x".to_owned()).await;
}

#[rustfmt::skip]
async fn set_group_15(client: &BotClient) {
    let _ = client.send_audio(1_i64, fixtures::input_file()).business_connection_id("x").message_thread_id(1_i64).direct_messages_topic_id(1_i64).disable_notification(true).message_effect_id("x").protect_content(true).allow_paid_broadcast(true).reply_parameters(fixtures::reply_to(7)).reply_markup(fixtures::reply_markup()).suggested_post_parameters(fixtures::suggested_post()).receiver_user_id(1_i64).callback_query_id("x").caption("x").parse_mode(ParseMode::HTML).caption_entities(Vec::new()).duration(1_u32).performer("x".to_owned()).title("x".to_owned()).thumbnail("x".to_owned()).await;
    let _ = client.send_chat_action(1_i64, ChatAction::Typing).business_connection_id("x").message_thread_id(1_i64).await;
    let _ = client.send_chat_join_request_web_app("x", "x").await;
    let _ = client.send_checklist("x", 1_i64, fixtures::checklist()).direct_messages_topic_id(1_i64).disable_notification(true).protect_content(true).message_effect_id("x").reply_parameters(fixtures::reply_to(7)).reply_markup(fixtures::inline_keyboard()).suggested_post_parameters(fixtures::suggested_post()).await;
    let _ = client.send_contact(1_i64, "x", "x").message_thread_id(1_i64).direct_messages_topic_id(1_i64).last_name("x").vcard("x").disable_notification(true).reply_markup(fixtures::reply_markup()).receiver_user_id(1_i64).callback_query_id("x").business_connection_id("x").allow_paid_broadcast(true).message_effect_id("x").suggested_post_parameters(fixtures::suggested_post()).protect_content(true).reply_parameters(fixtures::reply_to(7)).await;
    let _ = client.send_dice(1_i64).emoji("x").message_thread_id(1_i64).direct_messages_topic_id(1_i64).disable_notification(true).reply_markup(fixtures::reply_markup()).business_connection_id("x").allow_paid_broadcast(true).message_effect_id("x").suggested_post_parameters(fixtures::suggested_post()).protect_content(true).reply_parameters(fixtures::reply_to(7)).await;
    let _ = client.send_document(1_i64, fixtures::input_file()).business_connection_id("x").message_thread_id(1_i64).direct_messages_topic_id(1_i64).disable_notification(true).message_effect_id("x").protect_content(true).allow_paid_broadcast(true).reply_parameters(fixtures::reply_to(7)).reply_markup(fixtures::reply_markup()).suggested_post_parameters(fixtures::suggested_post()).receiver_user_id(1_i64).callback_query_id("x").caption("x").parse_mode(ParseMode::HTML).caption_entities(Vec::new()).disable_content_type_detection(true).thumbnail("x".to_owned()).await;
    let _ = client.send_game(1_i64, "x").business_connection_id("x").message_thread_id(1_i64).direct_messages_topic_id(1_i64).disable_notification(true).protect_content(true).reply_parameters(fixtures::reply_to(7)).reply_markup(fixtures::inline_keyboard()).allow_paid_broadcast(true).message_effect_id("x").await;
}

#[rustfmt::skip]
async fn set_group_16(client: &BotClient) {
    let _ = client.send_gift("x").user_id(1_i64).chat_id(1_i64).pay_for_upgrade(true).text("x").text_parse_mode(ParseMode::HTML).text_entities(Vec::new()).await;
    let _ = client.send_invoice(1_i64, "x", "x", "x", "x", vec![fixtures::labeled_price()]).message_thread_id(1_i64).direct_messages_topic_id(1_i64).provider_token("x").need_name(true).need_shipping_address(true).is_flexible(true).reply_markup(fixtures::inline_keyboard()).suggested_post_parameters(fixtures::suggested_post()).message_effect_id("x").max_tip_amount(1_u32).suggested_tip_amounts(vec![1_u32]).start_parameter("x").provider_data("x").photo_url("x").photo_size(1_u32).photo_width(1_u32).photo_height(1_u32).need_phone_number(true).need_email(true).send_phone_number_to_provider(true).send_email_to_provider(true).disable_notification(true).protect_content(true).allow_paid_broadcast(true).reply_parameters(fixtures::reply_to(7)).await;
    let _ = client.send_live_photo(1_i64, fixtures::input_file(), fixtures::input_file()).business_connection_id("x").message_thread_id(1_i64).direct_messages_topic_id(1_i64).caption("x").parse_mode(ParseMode::HTML).show_caption_above_media(true).has_spoiler(true).caption_entities(Vec::new()).receiver_user_id(1_i64).callback_query_id("x").disable_notification(true).protect_content(true).allow_paid_broadcast(true).message_effect_id("x").reply_parameters(fixtures::reply_to(7)).reply_markup(fixtures::reply_markup()).suggested_post_parameters(fixtures::suggested_post()).await;
    let _ = client.send_location(1_i64, 1.0_f64, 1.0_f64).message_thread_id(1_i64).direct_messages_topic_id(1_i64).horizontal_accuracy(1.0_f64).live_period(1_u32).heading(1_u16).proximity_alert_radius(1_u32).disable_notification(true).reply_markup(fixtures::reply_markup()).receiver_user_id(1_i64).callback_query_id("x").business_connection_id("x").allow_paid_broadcast(true).message_effect_id("x").suggested_post_parameters(fixtures::suggested_post()).protect_content(true).reply_parameters(fixtures::reply_to(7)).await;
    let _ = client.send_media_group(1_i64, vec![fixtures::input_media()]).message_thread_id(1_i64).direct_messages_topic_id(1_i64).business_connection_id("x").disable_notification(true).protect_content(true).reply_parameters(fixtures::reply_to(7)).allow_paid_broadcast(true).message_effect_id("x").await;
    let _ = client.send_message(1_i64, "x").business_connection_id("x").message_thread_id(1_i64).direct_messages_topic_id(1_i64).parse_mode(ParseMode::HTML).entities(Vec::new()).link_preview_options(Default::default()).disable_notification(true).protect_content(true).allow_paid_broadcast(true).message_effect_id("x").reply_parameters(fixtures::reply_to(7)).reply_to(1_i64).reply_to_ephemeral(1_i64).reply_markup(fixtures::reply_markup()).suggested_post_parameters(fixtures::suggested_post()).receiver_user_id(1_i64).callback_query_id("x").await;
    let _ = client.send_message_draft(1_i64, 1_i64, "x").parse_mode(ParseMode::HTML).entities(Vec::new()).message_thread_id(1_i64).await;
    let _ = client.send_paid_media(1_i64, 1_u32, vec![fixtures::paid_media()]).business_connection_id("x").payload("x").caption("x").parse_mode(ParseMode::HTML).show_caption_above_media(true).disable_notification(true).protect_content(true).reply_parameters(fixtures::reply_to(7)).reply_markup(fixtures::reply_markup()).allow_paid_broadcast(true).caption_entities(Vec::new()).direct_messages_topic_id(1_i64).message_thread_id(1_i64).suggested_post_parameters(fixtures::suggested_post()).await;
}

#[rustfmt::skip]
async fn set_group_17(client: &BotClient) {
    let _ = client.send_photo(1_i64, fixtures::input_file()).business_connection_id("x").message_thread_id(1_i64).direct_messages_topic_id(1_i64).caption("x").parse_mode(ParseMode::HTML).has_spoiler(true).caption_entities(Vec::new()).show_caption_above_media(true).disable_notification(true).protect_content(true).allow_paid_broadcast(true).reply_parameters(fixtures::reply_to(7)).reply_markup(fixtures::reply_markup()).suggested_post_parameters(fixtures::suggested_post()).receiver_user_id(1_i64).callback_query_id("x").message_effect_id("x").await;
    let _ = client.send_poll(1_i64, "x", vec![fixtures::poll_option()]).message_thread_id(1_i64).direct_messages_topic_id(1_i64).is_anonymous(true).allows_multiple_answers(true).allows_revoting(true).quiz(vec![1_u8]).explanation("x").explanation_parse_mode(ParseMode::HTML).explanation_entities(Vec::new()).open_period(1_u32).close_date(1_i64).shuffle_options(true).allow_adding_options(true).hide_results_until_closes(true).description("x").description_parse_mode(ParseMode::HTML).description_entities(Vec::new()).question_parse_mode(ParseMode::HTML).question_entities(Vec::new()).disable_notification(true).reply_markup(fixtures::reply_markup()).suggested_post_parameters(fixtures::suggested_post()).members_only(true).country_codes(vec!["x"]).media(fixtures::poll_media()).explanation_media(fixtures::poll_media()).business_connection_id("x").allow_paid_broadcast(true).message_effect_id("x").is_closed(true).protect_content(true).reply_parameters(fixtures::reply_to(7)).await;
    let _ = client.send_rich_message(1_i64, fixtures::rich_message()).business_connection_id("x").message_thread_id(1_i64).direct_messages_topic_id(1_i64).disable_notification(true).protect_content(true).allow_paid_broadcast(true).message_effect_id("x").suggested_post_parameters(fixtures::suggested_post()).reply_parameters(fixtures::reply_to(7)).reply_markup(fixtures::reply_markup()).await;
    let _ = client.send_rich_message_draft(1_i64, 1_i64, fixtures::rich_message()).message_thread_id(1_i64).await;
    let _ = client.send_sticker(1_i64, fixtures::input_file()).business_connection_id("x").message_thread_id(1_i64).direct_messages_topic_id(1_i64).disable_notification(true).message_effect_id("x").protect_content(true).allow_paid_broadcast(true).reply_parameters(fixtures::reply_to(7)).reply_markup(fixtures::reply_markup()).suggested_post_parameters(fixtures::suggested_post()).receiver_user_id(1_i64).callback_query_id("x").emoji("x".to_owned()).await;
    let _ = client.send_venue(1_i64, 1.0_f64, 1.0_f64, "x", "x").message_thread_id(1_i64).direct_messages_topic_id(1_i64).foursquare_id("x").foursquare_type("x").google_place_id("x").google_place_type("x").disable_notification(true).protect_content(true).reply_parameters(fixtures::reply_to(7)).reply_markup(fixtures::reply_markup()).receiver_user_id(1_i64).callback_query_id("x").business_connection_id("x").allow_paid_broadcast(true).message_effect_id("x").suggested_post_parameters(fixtures::suggested_post()).await;
    let _ = client.send_video(1_i64, fixtures::input_file()).business_connection_id("x").message_thread_id(1_i64).direct_messages_topic_id(1_i64).disable_notification(true).message_effect_id("x").protect_content(true).allow_paid_broadcast(true).reply_parameters(fixtures::reply_to(7)).reply_markup(fixtures::reply_markup()).suggested_post_parameters(fixtures::suggested_post()).receiver_user_id(1_i64).callback_query_id("x").caption("x").parse_mode(ParseMode::HTML).caption_entities(Vec::new()).show_caption_above_media(true).has_spoiler(true).duration(1_u32).width(1_u32).height(1_u32).supports_streaming(true).cover("x".to_owned()).start_timestamp(1_i64).thumbnail("x".to_owned()).await;
    let _ = client.send_video_note(1_i64, fixtures::input_file()).business_connection_id("x").message_thread_id(1_i64).direct_messages_topic_id(1_i64).disable_notification(true).message_effect_id("x").protect_content(true).allow_paid_broadcast(true).reply_parameters(fixtures::reply_to(7)).reply_markup(fixtures::reply_markup()).suggested_post_parameters(fixtures::suggested_post()).receiver_user_id(1_i64).callback_query_id("x").duration(1_u32).length(1_u32).thumbnail("x".to_owned()).await;
}

#[rustfmt::skip]
async fn set_group_18(client: &BotClient) {
    let _ = client.send_voice(1_i64, fixtures::input_file()).business_connection_id("x").message_thread_id(1_i64).direct_messages_topic_id(1_i64).disable_notification(true).message_effect_id("x").protect_content(true).allow_paid_broadcast(true).reply_parameters(fixtures::reply_to(7)).reply_markup(fixtures::reply_markup()).suggested_post_parameters(fixtures::suggested_post()).receiver_user_id(1_i64).callback_query_id("x").caption("x").parse_mode(ParseMode::HTML).caption_entities(Vec::new()).duration(1_u32).await;
    let _ = client.set_business_account_bio("x", Some("x".to_owned())).await;
    let _ = client.set_business_account_gift_settings("x", true, Default::default()).await;
    let _ = client.set_business_account_name("x", "x", Some("x".to_owned())).await;
    let _ = client.set_business_account_profile_photo("x", fixtures::profile_photo()).is_public(true).await;
    let _ = client.set_business_account_username("x", Some("x".to_owned())).await;
    let _ = client.set_chat_administrator_custom_title(1_i64, 1_i64, "x").await;
    let _ = client.set_chat_description(1_i64).description("x").await;
}

#[rustfmt::skip]
async fn set_group_19(client: &BotClient) {
    let _ = client.set_chat_member_tag(1_i64, 1_i64).tag("x").await;
    let _ = client.set_chat_menu_button().chat_id(1_i64).menu_button(fixtures::menu_button()).await;
    let _ = client.set_chat_permissions(1_i64, Default::default()).use_independent_chat_permissions(true).await;
    let _ = client.set_chat_photo(1_i64, fixtures::input_file()).await;
    let _ = client.set_chat_sticker_set(1_i64, "x").await;
    let _ = client.set_chat_title(1_i64, "x").await;
    let _ = client.set_custom_emoji_sticker_set_thumbnail("x").custom_emoji_id("x").await;
    let _ = client.set_game_score(1_i64, 1_u32).force(true).disable_edit_message(true).inline_message_id("x").await;
}

#[rustfmt::skip]
async fn set_group_20(client: &BotClient) {
    let _ = client.set_managed_bot_access_settings(1_i64, true).added_user_ids(vec![1_i64]).await;
    let _ = client.set_message_reaction(1_i64, 1_i64).reaction(Vec::new()).is_big(true).await;
    let _ = client.set_my_commands(vec![fixtures::bot_command()]).scope(fixtures::command_scope()).language_code("x").await;
    let _ = client.set_my_default_administrator_rights().rights(Default::default()).for_channels(true).await;
    let _ = client.set_my_description().description("x").language_code("x").await;
    let _ = client.set_my_name().name("x").language_code("x").await;
    let _ = client.set_my_profile_photo(fixtures::profile_photo()).await;
    let _ = client.set_my_short_description().short_description("x").language_code("x").await;
}

#[rustfmt::skip]
async fn set_group_21(client: &BotClient) {
    let _ = client.set_passport_data_errors(1_i64, vec![fixtures::passport_error()]).await;
    let _ = client.set_sticker_emoji_list("x", vec!["x"]).await;
    let _ = client.set_sticker_keywords("x").keywords(vec!["x"]).await;
    let _ = client.set_sticker_mask_position("x").mask_position(fixtures::mask_position()).await;
    let _ = client.set_sticker_position_in_set("x", 1_u32).await;
    let _ = client.set_sticker_set_thumbnail("x", 1_i64, "x").thumbnail(fixtures::input_file()).await;
    let _ = client.set_sticker_set_title("x", "x").await;
    let _ = client.set_user_emoji_status(1_i64).emoji_status_custom_emoji_id("x").emoji_status_expiration_date(1_i64).await;
}

#[rustfmt::skip]
async fn set_group_22(client: &BotClient) {
    let _ = client.set_webhook("x").certificate(fixtures::input_file()).ip_address("x").max_connections(1_u8).allowed_updates(vec!["x"]).drop_pending_updates(true).secret_token("x").await;
    let _ = client.stop_inline_message_live_location("x").reply_markup(fixtures::inline_keyboard()).business_connection_id("x").await;
    let _ = client.stop_message_live_location(1_i64, 1_i64).reply_markup(fixtures::inline_keyboard()).business_connection_id("x").await;
    let _ = client.stop_poll(1_i64, 1_i64).reply_markup(fixtures::inline_keyboard()).business_connection_id("x").await;
    let _ = client.transfer_business_account_stars("x", 1_u64).await;
    let _ = client.transfer_gift("x", "x", 1_i64).star_count(1_u32).await;
    let _ = client.unban_chat_member(1_i64, 1_i64).only_if_banned(true).await;
    let _ = client.unban_chat_sender_chat(1_i64, 1_i64).await;
}

#[rustfmt::skip]
async fn set_group_23(client: &BotClient) {
    let _ = client.unhide_general_forum_topic(1_i64).await;
    let _ = client.unpin_all_chat_messages(1_i64).await;
    let _ = client.unpin_all_forum_topic_messages(1_i64, 1_i64).await;
    let _ = client.unpin_all_general_forum_topic_messages(1_i64).await;
    let _ = client.unpin_chat_message(1_i64).message_id(1_i64).business_connection_id("x").await;
    let _ = client.upgrade_gift("x", "x").keep_original_details(true).star_count(1_u32).await;
    let _ = client.upload_sticker_file(1_i64, fixtures::input_file(), StickerFormat::Static).await;
    let _ = client.verify_chat(1_i64).custom_description("x").await;
}

#[rustfmt::skip]
async fn set_group_24(client: &BotClient) {
    let _ = client.verify_user(1_i64).custom_description("x").await;
}

/// Calls every entry point with **every setter applied**.
///
/// Split into boxed groups: each `.await` contributes its future to the
/// enclosing state machine, and these chains are far wider than the
/// required-args-only sweep, so one function overflows the stack outright.
async fn call_every_builder_fully(client: &BotClient) {
    Box::pin(set_group_00(client)).await;
    Box::pin(set_group_01(client)).await;
    Box::pin(set_group_02(client)).await;
    Box::pin(set_group_03(client)).await;
    Box::pin(set_group_04(client)).await;
    Box::pin(set_group_05(client)).await;
    Box::pin(set_group_06(client)).await;
    Box::pin(set_group_07(client)).await;
    Box::pin(set_group_08(client)).await;
    Box::pin(set_group_09(client)).await;
    Box::pin(set_group_10(client)).await;
    Box::pin(set_group_11(client)).await;
    Box::pin(set_group_12(client)).await;
    Box::pin(set_group_13(client)).await;
    Box::pin(set_group_14(client)).await;
    Box::pin(set_group_15(client)).await;
    Box::pin(set_group_16(client)).await;
    Box::pin(set_group_17(client)).await;
    Box::pin(set_group_18(client)).await;
    Box::pin(set_group_19(client)).await;
    Box::pin(set_group_20(client)).await;
    Box::pin(set_group_21(client)).await;
    Box::pin(set_group_22(client)).await;
    Box::pin(set_group_23(client)).await;
    Box::pin(set_group_24(client)).await;
}

/// Runs the sweep on a large-stack thread and returns every request sent.
fn sweep() -> Vec<Request> {
    std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("a test runtime starts")
                .block_on(async {
                    let (server, client) = mock::spawn().await;
                    mock::mount_catch_all(&server).await;
                    tokio::time::timeout(
                        Duration::from_secs(120),
                        call_every_builder_fully(&client),
                    )
                    .await
                    .expect("the sweep finishes");
                    mock::requests(&server).await
                })
        })
        .expect("the sweep thread starts")
        .join()
        .expect("the sweep thread finishes")
}

/// Field names a request carries, whichever encoding it used.
fn fields_of(request: &Request) -> BTreeSet<String> {
    match serde_json::from_slice::<Value>(&request.body) {
        Ok(Value::Object(body)) => body.keys().cloned().collect(),
        _ => mock::multipart_field_names(request).into_iter().collect(),
    }
}

/// With every setter called, every optional spec parameter is on the wire.
#[test]
fn every_optional_parameter_reaches_the_wire() {
    let spec: Spec = serde_json::from_str(SNAPSHOT).expect("the snapshot parses");
    let requests = sweep();
    assert!(
        requests.len() > 150,
        "the sweep produced only {} requests; it is not calling the builders",
        requests.len()
    );

    // Union the fields across every request that hit a method, rather than
    // requiring each one to carry everything. Several methods have more than
    // one entry point with mutually exclusive targets — `editMessageText` can
    // address a chat message or an inline one, never both — so a per-request
    // rule would demand fields that cannot coexist.
    let mut seen: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for request in &requests {
        seen.entry(mock::api_method_of(request))
            .or_default()
            .extend(fields_of(request));
    }

    let mut absent = Vec::new();
    for (method, sent) in &seen {
        let Some(params) = spec.methods.get(method) else {
            continue;
        };
        for (name, param) in params {
            if !param.optional() || sent.contains(name) {
                continue;
            }
            if NOT_SET_BY_SWEEP
                .iter()
                .any(|(m, p, _)| m == method && p == name)
            {
                continue;
            }
            absent.push(format!("  {method}.{name}"));
        }
    }

    absent.sort();
    absent.dedup();
    assert!(
        absent.is_empty(),
        "{} optional parameter(s) were set on the builder and never reached the \
         request. A setter that writes the wrong field, or a field whose \
         serialisation is suppressed, looks exactly like this:\n{}",
        absent.len(),
        absent.join("\n")
    );
}

/// Each documented exception still describes a real spec parameter.
#[test]
fn every_sweep_exception_still_applies() {
    let spec: Spec = serde_json::from_str(SNAPSHOT).expect("the snapshot parses");
    let stale: Vec<String> = NOT_SET_BY_SWEEP
        .iter()
        .filter(|(m, p, _)| !spec.methods.get(*m).is_some_and(|ps| ps.contains_key(*p)))
        .map(|(m, p, _)| format!("  {m}.{p}"))
        .collect();
    assert!(
        stale.is_empty(),
        "{} exception(s) name a parameter the spec no longer has:\n{}",
        stale.len(),
        stale.join("\n")
    );
}
