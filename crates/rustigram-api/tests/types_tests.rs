//! Unit tests for rustigram-types serialisation and helper methods.

#[cfg(test)]
mod chat_id {
    use rustigram_types::user::ChatId;

    #[test]
    fn numeric_id_display() {
        assert_eq!(ChatId::Id(-100123456).to_string(), "-100123456");
    }

    #[test]
    fn username_display() {
        assert_eq!(
            ChatId::Username("@rustigram".to_owned()).to_string(),
            "@rustigram"
        );
    }

    #[test]
    fn from_i64() {
        let id: ChatId = 12345_i64.into();
        assert!(matches!(id, ChatId::Id(12345)));
    }

    #[test]
    fn from_str() {
        let id: ChatId = "@testbot".into();
        assert!(matches!(id, ChatId::Username(_)));
    }

    #[test]
    fn serialises_as_integer() {
        let id = ChatId::Id(-100987654321);
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "-100987654321");
    }

    #[test]
    fn serialises_as_string() {
        let id = ChatId::Username("@mybot".to_owned());
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"@mybot\"");
    }
}

#[cfg(test)]
mod user {
    use rustigram_types::user::User;

    fn make_user(first: &str, last: Option<&str>, username: Option<&str>) -> User {
        User {
            id: 1,
            is_bot: false,
            first_name: first.to_owned(),
            last_name: last.map(str::to_owned),
            username: username.map(str::to_owned),
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
        }
    }

    #[test]
    fn full_name_with_last() {
        let u = make_user("John", Some("Doe"), None);
        assert_eq!(u.full_name(), "John Doe");
    }

    #[test]
    fn full_name_without_last() {
        let u = make_user("Alice", None, None);
        assert_eq!(u.full_name(), "Alice");
    }

    #[test]
    fn mention_with_username() {
        let u = make_user("Bob", None, Some("bobbot"));
        assert_eq!(u.mention(), Some("@bobbot".to_owned()));
    }

    #[test]
    fn mention_without_username() {
        let u = make_user("Charlie", None, None);
        assert_eq!(u.mention(), None);
    }
}

#[cfg(test)]
mod message {
    use rustigram_types::chat::{Chat, ChatType};
    use rustigram_types::message::{Message, MessageEntity, MessageEntityKind};

    fn make_message(text: Option<&str>, entities: Option<Vec<MessageEntity>>) -> Message {
        Message {
            message_id: 1,
            message_thread_id: None,
            from: None,
            sender_chat: None,
            sender_boost_count: None,
            sender_business_bot: None,
            date: 0,
            business_connection_id: None,
            chat: Chat {
                id: 1,
                kind: ChatType::Private,
                title: None,
                username: None,
                first_name: None,
                last_name: None,
                is_forum: None,
                is_direct_messages: None,
            },
            forward_origin: None,
            is_topic_message: None,
            is_automatic_forward: None,
            reply_to_message: None,
            external_reply: None,
            quote: None,
            reply_to_story: None,
            via_bot: None,
            edit_date: None,
            has_protected_content: None,
            is_from_offline: None,
            media_group_id: None,
            author_signature: None,
            text: text.map(str::to_owned),
            entities,
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
            paid_message_price_changed: None,
            sender_tag: None,
            reply_to_checklist_task_id: None,
            reply_to_poll_option_id: None,
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
            suggested_post_approved: None,
            suggested_post_approval_failed: None,
            suggested_post_declined: None,
            suggested_post_paid: None,
            suggested_post_refunded: None,
        }
    }

    #[test]
    fn command_extracted_from_entity() {
        let msg = make_message(
            Some("/start"),
            Some(vec![MessageEntity {
                kind: MessageEntityKind::BotCommand,
                offset: 0,
                length: 6,
                url: None,
                user: None,
                language: None,
                custom_emoji_id: None,
                unix_time: None,
                date_time_format: None,
            }]),
        );
        assert!(msg.is_command());
        assert_eq!(msg.command(), Some("start"));
    }

    #[test]
    fn command_strips_bot_suffix() {
        let msg = make_message(
            Some("/start@mybot"),
            Some(vec![MessageEntity {
                kind: MessageEntityKind::BotCommand,
                offset: 0,
                length: 12,
                url: None,
                user: None,
                language: None,
                custom_emoji_id: None,
                unix_time: None,
                date_time_format: None,
            }]),
        );
        assert_eq!(msg.command(), Some("start"));
    }

    #[test]
    fn not_command_when_no_entities() {
        let msg = make_message(Some("/start"), None);
        assert!(!msg.is_command());
        assert_eq!(msg.command(), None);
    }

    #[test]
    fn effective_text_prefers_text_over_caption() {
        let mut msg = make_message(Some("hello"), None);
        msg.caption = Some("caption".to_owned());
        assert_eq!(msg.effective_text(), Some("hello"));
    }

    #[test]
    fn effective_text_falls_back_to_caption() {
        let mut msg = make_message(None, None);
        msg.caption = Some("a caption".to_owned());
        assert_eq!(msg.effective_text(), Some("a caption"));
    }
}

#[cfg(test)]
mod keyboard {
    use rustigram_types::keyboard::{InlineKeyboardButton, InlineKeyboardMarkup};

    #[test]
    fn builder_adds_rows() {
        let markup = InlineKeyboardMarkup::new()
            .row(vec![InlineKeyboardButton::callback("A", "a")])
            .row(vec![InlineKeyboardButton::callback("B", "b")]);
        assert_eq!(markup.inline_keyboard.len(), 2);
    }

    #[test]
    fn callback_button_sets_data() {
        let btn = InlineKeyboardButton::callback("Click me", "my_data");
        assert_eq!(btn.text, "Click me");
        assert_eq!(btn.callback_data.as_deref(), Some("my_data"));
        assert!(btn.url.is_none());
    }

    #[test]
    fn url_button_sets_url() {
        let btn = InlineKeyboardButton::url("Visit", "https://example.com");
        assert_eq!(btn.url.as_deref(), Some("https://example.com"));
        assert!(btn.callback_data.is_none());
    }

    #[test]
    fn markup_serialises_correctly() {
        let markup =
            InlineKeyboardMarkup::new().row(vec![InlineKeyboardButton::callback("OK", "ok")]);
        let json = serde_json::to_value(&markup).unwrap();
        assert!(json["inline_keyboard"].is_array());
        assert_eq!(json["inline_keyboard"][0][0]["text"], "OK");
        assert_eq!(json["inline_keyboard"][0][0]["callback_data"], "ok");
    }
}

#[cfg(test)]
mod poll {
    use rustigram_types::poll::InputPollOption;

    #[test]
    fn input_poll_option_new() {
        let opt = InputPollOption::new("Option A");
        assert_eq!(opt.text, "Option A");
        assert!(opt.text_parse_mode.is_none());
    }
}

#[cfg(test)]
mod update_deserialization {
    use rustigram_types::update::{Update, UpdateKind};

    #[test]
    fn deserialises_message_update() {
        let json = r#"{
            "update_id": 42,
            "message": {
                "message_id": 1,
                "date": 1700000000,
                "chat": { "id": 123, "type": "private" },
                "text": "hello"
            }
        }"#;

        let update: Update = serde_json::from_str(json).unwrap();
        assert_eq!(update.update_id, 42);
        assert!(matches!(update.kind, UpdateKind::Message(_)));

        if let UpdateKind::Message(msg) = &update.kind {
            assert_eq!(msg.text.as_deref(), Some("hello"));
            assert_eq!(msg.chat.id, 123);
        }
    }

    #[test]
    fn deserialises_callback_query_update() {
        let json = r#"{
            "update_id": 99,
            "callback_query": {
                "id": "cbq_id",
                "from": { "id": 777, "is_bot": false, "first_name": "Test" },
                "chat_instance": "inst_123",
                "data": "btn_click"
            }
        }"#;

        let update: Update = serde_json::from_str(json).unwrap();
        assert_eq!(update.update_id, 99);
        assert!(matches!(update.kind, UpdateKind::CallbackQuery(_)));

        if let UpdateKind::CallbackQuery(cq) = &update.kind {
            assert_eq!(cq.data.as_deref(), Some("btn_click"));
            assert_eq!(cq.from.id, 777);
        }
    }

    #[test]
    fn update_chat_id_from_message() {
        let json = r#"{
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 0,
                "chat": { "id": 555, "type": "group" }
            }
        }"#;
        let update: Update = serde_json::from_str(json).unwrap();
        assert_eq!(update.chat_id(), Some(555));
    }
}
