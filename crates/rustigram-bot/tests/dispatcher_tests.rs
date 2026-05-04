//! Unit tests for the dispatcher and filter system.
//!
//! These tests use hand-crafted `Update` values so no network is needed.

use rustigram_types::chat::{Chat, ChatType};
use rustigram_types::message::Message;
use rustigram_types::update::{Update, UpdateKind};
use rustigram_types::user::User;

fn make_user(id: i64) -> User {
    User {
        id,
        is_bot: false,
        first_name: "Test".into(),
        last_name: None,
        username: None,
        language_code: None,
        is_premium: None,
        added_to_attachment_menu: None,
        can_join_groups: None,
        can_read_all_group_messages: None,
        supports_inline_queries: None,
        can_connect_to_business: None,
        has_main_web_app: None,
        can_manage_bots: None,
        has_topics_enabled: None,
        allows_users_to_create_topics: None,
        supports_guest_queries: None,
    }
}

fn make_chat(id: i64, kind: ChatType) -> Chat {
    Chat {
        id,
        kind,
        title: None,
        username: None,
        first_name: None,
        last_name: None,
        is_forum: None,
        is_direct_messages: None,
    }
}

fn make_text_update(text: &str, chat_type: ChatType) -> Update {
    Update {
        update_id: 1,
        kind: UpdateKind::Message(Message {
            message_id: 1,
            message_thread_id: None,
            from: Some(make_user(42)),
            sender_chat: None,
            sender_boost_count: None,
            sender_business_bot: None,
            sender_tag: None,
            date: 0,
            business_connection_id: None,
            chat: make_chat(100, chat_type),
            forward_origin: None,
            is_topic_message: None,
            is_automatic_forward: None,
            reply_to_message: None,
            reply_to_checklist_task_id: None,
            reply_to_poll_option_id: None,
            external_reply: None,
            quote: None,
            reply_to_story: None,
            via_bot: None,
            edit_date: None,
            has_protected_content: None,
            is_from_offline: None,
            media_group_id: None,
            author_signature: None,
            text: Some(text.to_owned()),
            entities: None,
            link_preview_options: None,
            effect_id: None,
            animation: None,
            audio: None,
            document: None,
            paid_media: None,
            photo: None,
            sticker: None,
            story: None,
            video: None,
            video_note: None,
            voice: None,
            caption: None,
            caption_entities: None,
            show_caption_above_media: None,
            has_media_spoiler: None,
            contact: None,
            dice: None,
            game: None,
            poll: None,
            venue: None,
            location: None,
            new_chat_members: None,
            left_chat_member: None,
            new_chat_title: None,
            new_chat_photo: None,
            delete_chat_photo: None,
            group_chat_created: None,
            supergroup_chat_created: None,
            channel_chat_created: None,
            message_auto_delete_timer_changed: None,
            migrate_to_chat_id: None,
            migrate_from_chat_id: None,
            pinned_message: None,
            reply_markup: None,
            invoice: None,
            successful_payment: None,
            refunded_payment: None,
            web_app_data: None,
            forum_topic_created: None,
            forum_topic_edited: None,
            forum_topic_closed: None,
            forum_topic_reopened: None,
            general_forum_topic_hidden: None,
            general_forum_topic_unhidden: None,
            direct_messages_topic: None,
            is_paid_post: None,
            paid_star_count: None,
            suggested_post_info: None,
            checklist: None,
            chat_owner_left: None,
            chat_owner_changed: None,
            managed_bot_created: None,
            poll_option_added: None,
            poll_option_deleted: None,
            checklist_tasks_done: None,
            checklist_tasks_added: None,
            direct_message_price_changed: None,
            paid_message_price_changed: None,
            suggested_post_approved: None,
            suggested_post_approval_failed: None,
            suggested_post_declined: None,
            suggested_post_paid: None,
            suggested_post_refunded: None,
            guest_bot_caller_chat: None,
            guest_bot_caller_user: None,
            guest_query_id: None,
        }),
    }
}

fn make_command_update(cmd: &str) -> Update {
    use rustigram_types::message::{MessageEntity, MessageEntityKind};
    let text = format!("/{cmd}");
    let length = text.len() as u32;
    let mut update = make_text_update(&text, ChatType::Private);
    if let UpdateKind::Message(ref mut msg) = update.kind {
        msg.entities = Some(vec![MessageEntity {
            kind: MessageEntityKind::BotCommand,
            offset: 0,
            length,
            url: None,
            user: None,
            language: None,
            custom_emoji_id: None,
            unix_time: None,
            date_time_format: None,
        }]);
    }
    update
}

#[cfg(test)]
mod filter_tests {
    use super::*;
    use rustigram_api::BotClient;
    use rustigram_bot::filter::filters;
    use rustigram_bot::filter::Filter;
    use rustigram_bot::Context;

    fn ctx_from(update: Update) -> Context {
        let client = BotClient::from_token("123456:ABCDEFabcdef").unwrap();
        Context::new(update, client)
    }

    #[test]
    fn message_filter_passes_messages() {
        let ctx = ctx_from(make_text_update("hello", ChatType::Private));
        assert!(filters::message().check(&ctx));
    }

    #[test]
    fn command_filter_matches_exact() {
        let ctx = ctx_from(make_command_update("start"));
        assert!(filters::command("start").check(&ctx));
        assert!(!filters::command("help").check(&ctx));
    }

    #[test]
    fn command_filter_case_insensitive() {
        let ctx = ctx_from(make_command_update("Start"));
        assert!(filters::command("start").check(&ctx));
    }

    #[test]
    fn text_filter_exact_match() {
        let ctx = ctx_from(make_text_update("hello world", ChatType::Private));
        assert!(filters::text("hello world").check(&ctx));
        assert!(!filters::text("hello").check(&ctx));
    }

    #[test]
    fn text_contains_filter() {
        let ctx = ctx_from(make_text_update("I love Rust", ChatType::Private));
        assert!(filters::text_contains("Rust").check(&ctx));
        assert!(!filters::text_contains("Python").check(&ctx));
    }

    #[test]
    fn private_chat_filter() {
        let private_ctx = ctx_from(make_text_update("hi", ChatType::Private));
        let group_ctx = ctx_from(make_text_update("hi", ChatType::Group));
        assert!(filters::private().check(&private_ctx));
        assert!(!filters::private().check(&group_ctx));
    }

    #[test]
    fn group_filter() {
        let private_ctx = ctx_from(make_text_update("hi", ChatType::Private));
        let group_ctx = ctx_from(make_text_update("hi", ChatType::Group));
        let supergroup_ctx = ctx_from(make_text_update("hi", ChatType::Supergroup));
        assert!(!filters::group().check(&private_ctx));
        assert!(filters::group().check(&group_ctx));
        assert!(filters::group().check(&supergroup_ctx));
    }

    #[test]
    fn and_combinator() {
        use rustigram_bot::filter::FilterExt;
        let ctx = ctx_from(make_text_update("hello", ChatType::Private));
        let combined = filters::message().and(filters::private());
        assert!(combined.check(&ctx));

        let group_ctx = ctx_from(make_text_update("hello", ChatType::Group));
        assert!(!combined.check(&group_ctx));
    }

    #[test]
    fn or_combinator() {
        use rustigram_bot::filter::FilterExt;
        let private_ctx = ctx_from(make_text_update("hi", ChatType::Private));
        let channel_ctx = ctx_from(make_text_update("hi", ChatType::Channel));
        let combined = filters::private().or(filters::group());
        assert!(combined.check(&private_ctx));
        assert!(!combined.check(&channel_ctx));
    }

    #[test]
    fn not_combinator() {
        use rustigram_bot::filter::FilterExt;
        let ctx = ctx_from(make_text_update("hi", ChatType::Private));
        let not_private = filters::private().not();
        assert!(!not_private.check(&ctx));

        let group_ctx = ctx_from(make_text_update("hi", ChatType::Group));
        assert!(not_private.check(&group_ctx));
    }
}

#[cfg(test)]
mod context_tests {
    use super::*;
    use rustigram_api::BotClient;
    use rustigram_bot::Context;
    use rustigram_types::user::ChatId;

    fn ctx_from(update: Update) -> Context {
        let client = BotClient::from_token("123456:ABCDEFabcdef").unwrap();
        Context::new(update, client)
    }

    #[test]
    fn context_text_extracts_message_text() {
        let ctx = ctx_from(make_text_update("hello rustigram", ChatType::Private));
        assert_eq!(ctx.text(), Some("hello rustigram"));
    }

    #[test]
    fn context_command_extracts_command() {
        let ctx = ctx_from(make_command_update("start"));
        assert_eq!(ctx.command(), Some("start"));
    }

    #[test]
    fn context_chat_id_from_message() {
        let ctx = ctx_from(make_text_update("hi", ChatType::Private));
        assert_eq!(ctx.chat_id(), Some(ChatId::Id(100)));
    }

    #[test]
    fn context_from_id() {
        let ctx = ctx_from(make_text_update("hi", ChatType::Private));
        assert_eq!(ctx.from_id(), Some(42));
    }
}

#[cfg(test)]
mod state_tests {
    use rustigram_bot::state::{DialogueStorage, StateStorage};

    #[test]
    fn state_storage_insert_and_get() {
        let store = StateStorage::new();
        store.insert(42_u32);
        assert_eq!(store.get::<u32>(), Some(42));
    }

    #[test]
    fn state_storage_overwrite() {
        let store = StateStorage::new();
        store.insert(1_i32);
        store.insert(99_i32);
        assert_eq!(store.get::<i32>(), Some(99));
    }

    #[test]
    fn state_storage_missing_returns_none() {
        let store = StateStorage::new();
        assert!(store.get::<String>().is_none());
    }

    #[test]
    fn dialogue_storage_set_and_get() {
        #[derive(Clone, Debug, PartialEq)]
        enum State {
            Active,
        }

        let store = DialogueStorage::new();
        store.set(100, 42, State::Active);
        assert_eq!(store.get::<State>(100, 42), Some(State::Active));
    }

    #[test]
    fn dialogue_storage_remove() {
        #[derive(Clone)]
        enum State {
            A,
        }
        let store = DialogueStorage::new();
        store.set(1, 1, State::A);
        store.remove(1, 1);
        assert!(store.get::<State>(1, 1).is_none());
    }

    #[test]
    fn dialogue_storage_isolated_per_user() {
        #[derive(Clone, Debug, PartialEq)]
        enum State {
            One,
            Two,
        }
        let store = DialogueStorage::new();
        store.set(100, 1, State::One);
        store.set(100, 2, State::Two);
        assert_eq!(store.get::<State>(100, 1), Some(State::One));
        assert_eq!(store.get::<State>(100, 2), Some(State::Two));
    }
}
