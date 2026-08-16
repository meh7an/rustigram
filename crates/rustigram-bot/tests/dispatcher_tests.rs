//! Unit tests for the dispatcher and filter system.
//!
//! These tests use hand-crafted `Update` values so no network is needed.

use rustigram_types::chat::{Chat, ChatType};
use rustigram_types::message::Message;
use rustigram_types::update::{Update, UpdateKind};
use rustigram_types::user::User;

// The core types are `#[non_exhaustive]`, so a struct literal is not available
// from another crate -- `..Default::default()` included. Assigning after
// `default()` is the only form left. The upside is that these fixtures no
// longer enumerate every field, so a Bot API field addition stops being a
// compile error here.
fn make_user(id: i64) -> User {
    let mut user = User::default();
    user.id = id;
    user.first_name = "Test".into();
    user
}

fn make_chat(id: i64, kind: ChatType) -> Chat {
    let mut chat = Chat::default();
    chat.id = id;
    chat.kind = kind;
    chat
}

fn make_text_update(text: &str, chat_type: ChatType) -> Update {
    let mut message = Message::default();
    message.message_id = 1;
    message.from = Some(make_user(42));
    message.chat = make_chat(100, chat_type);
    message.text = Some(text.to_owned());

    Update {
        update_id: 1,
        kind: UpdateKind::Message(message),
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

    #[test]
    fn context_is_ephemeral_false_for_regular_message() {
        let ctx = ctx_from(make_text_update("hi", ChatType::Private));
        assert!(!ctx.is_ephemeral());
        assert_eq!(ctx.ephemeral_message_id(), None);
    }

    #[test]
    fn context_is_ephemeral_true_when_set() {
        let mut update = make_text_update("shh", ChatType::Group);
        if let UpdateKind::Message(ref mut msg) = update.kind {
            msg.ephemeral_message_id = Some(7);
        }
        let ctx = ctx_from(update);
        assert!(ctx.is_ephemeral());
        assert_eq!(ctx.ephemeral_message_id(), Some(7));
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
