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

    // `User` is `#[non_exhaustive]`, so struct-literal construction -- including
    // `..Default::default()` -- is unavailable from this crate.
    fn make_user(first: &str, last: Option<&str>, username: Option<&str>) -> User {
        let mut user = User::default();
        user.id = 1;
        user.first_name = first.to_owned();
        user.last_name = last.map(str::to_owned);
        user.username = username.map(str::to_owned);
        user
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

    // `Message` and `Chat` are `#[non_exhaustive]`; see the note on `make_user`.
    fn make_message(text: Option<&str>, entities: Option<Vec<MessageEntity>>) -> Message {
        let mut chat = Chat::default();
        chat.id = 1;
        chat.kind = ChatType::Private;

        let mut message = Message::default();
        message.message_id = 1;
        message.chat = chat;
        message.text = text.map(str::to_owned);
        message.entities = entities;
        message
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
mod rich_message {
    use rustigram_types::file::{InputMediaPhoto, InputMediaVoiceNote};
    use rustigram_types::rich_message::{
        InputRichBlock, InputRichBlockListItem, InputRichBlockParagraph,
        InputRichBlockSectionHeading, InputRichMessage, InputRichMessageMedia,
        InputRichMessageMediaKind, RichText,
    };

    #[test]
    fn input_media_voice_note_round_trip() {
        let voice = InputMediaVoiceNote {
            media: "file_id_123".to_owned(),
            caption: Some("a note".to_owned()),
            parse_mode: None,
            caption_entities: None,
            duration: Some(12),
        };
        let json = serde_json::to_string(&voice).unwrap();
        let back: InputMediaVoiceNote = serde_json::from_str(&json).unwrap();
        assert_eq!(back.media, "file_id_123");
        assert_eq!(back.duration, Some(12));
    }

    #[test]
    fn input_rich_message_media_tags_by_type() {
        let media = InputRichMessageMedia {
            id: "photo-1".to_owned(),
            media: InputRichMessageMediaKind::Photo(InputMediaPhoto {
                media: "file_id_abc".to_owned(),
                caption: None,
                parse_mode: None,
                caption_entities: None,
                show_caption_above_media: None,
                has_spoiler: None,
            }),
        };
        let json = serde_json::to_value(&media).unwrap();
        assert_eq!(json["id"], "photo-1");
        assert_eq!(json["media"]["type"], "photo");
        assert_eq!(json["media"]["media"], "file_id_abc");
    }

    #[test]
    fn input_rich_block_paragraph_round_trip() {
        let block = InputRichBlock::Paragraph(InputRichBlockParagraph {
            text: RichText::Plain("hello".to_owned()),
        });
        let json = serde_json::to_value(&block).unwrap();
        assert_eq!(json["type"], "paragraph");
        let back: InputRichBlock = serde_json::from_value(json).unwrap();
        assert!(matches!(back, InputRichBlock::Paragraph(_)));
    }

    #[test]
    fn input_rich_block_section_heading_uses_heading_tag() {
        let block = InputRichBlock::SectionHeading(InputRichBlockSectionHeading {
            text: RichText::Plain("Title".to_owned()),
            size: 2,
        });
        let json = serde_json::to_value(&block).unwrap();
        assert_eq!(json["type"], "heading");
        assert_eq!(json["size"], 2);
    }

    #[test]
    fn input_rich_block_list_nests_paragraphs() {
        let block = InputRichBlock::List(rustigram_types::rich_message::InputRichBlockList {
            items: vec![InputRichBlockListItem {
                blocks: vec![InputRichBlock::Paragraph(InputRichBlockParagraph {
                    text: RichText::Plain("item one".to_owned()),
                })],
                has_checkbox: None,
                is_checked: None,
                value: None,
                kind: None,
            }],
        });
        let json = serde_json::to_value(&block).unwrap();
        assert_eq!(json["type"], "list");
        assert_eq!(json["items"][0]["blocks"][0]["type"], "paragraph");
    }

    #[test]
    fn input_rich_message_from_blocks() {
        let msg = InputRichMessage::from_blocks(vec![InputRichBlock::Paragraph(
            InputRichBlockParagraph {
                text: RichText::Plain("body".to_owned()),
            },
        )]);
        assert!(msg.html.is_none());
        assert!(msg.markdown.is_none());
        assert!(msg.blocks.is_some());
        let json = serde_json::to_value(&msg).unwrap();
        assert!(json.get("html").is_none());
        assert!(json.get("blocks").is_some());
    }

    #[test]
    fn input_rich_message_media_setter() {
        let msg = InputRichMessage::from_html("<p>hi</p>").media(vec![InputRichMessageMedia {
            id: "img-1".to_owned(),
            media: InputRichMessageMediaKind::Photo(InputMediaPhoto {
                media: "file_id".to_owned(),
                caption: None,
                parse_mode: None,
                caption_entities: None,
                show_caption_above_media: None,
                has_spoiler: None,
            }),
        }]);
        assert_eq!(msg.media.as_ref().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod ephemeral {
    use rustigram_types::message::ReplyParameters;
    use rustigram_types::user::BotCommand;

    #[test]
    fn bot_command_is_ephemeral_round_trip() {
        let cmd = BotCommand {
            command: "secret".to_owned(),
            description: "sends an ephemeral reply".to_owned(),
            is_ephemeral: Some(true),
        };
        let json = serde_json::to_value(&cmd).unwrap();
        assert_eq!(json["is_ephemeral"], true);
        let back: BotCommand = serde_json::from_value(json).unwrap();
        assert_eq!(back.is_ephemeral, Some(true));
    }

    #[test]
    fn bot_command_omits_is_ephemeral_when_none() {
        let cmd = BotCommand {
            command: "start".to_owned(),
            description: "greeting".to_owned(),
            is_ephemeral: None,
        };
        let json = serde_json::to_value(&cmd).unwrap();
        assert!(json.get("is_ephemeral").is_none());
    }

    #[test]
    fn message_deserialises_ephemeral_fields() {
        let json = r#"{
            "message_id": 0,
            "date": 1700000000,
            "chat": { "id": 1, "type": "group" },
            "receiver_user": { "id": 42, "is_bot": false, "first_name": "Test" },
            "ephemeral_message_id": 7
        }"#;
        let msg: rustigram_types::message::Message = serde_json::from_str(json).unwrap();
        assert_eq!(msg.ephemeral_message_id, Some(7));
        assert_eq!(msg.receiver_user.map(|u| u.id), Some(42));
    }

    #[test]
    fn reply_parameters_ephemeral_only() {
        let rp = ReplyParameters {
            message_id: None,
            ephemeral_message_id: Some(99),
            chat_id: None,
            allow_sending_without_reply: None,
            quote: None,
            quote_parse_mode: None,
            quote_entities: None,
            quote_position: None,
            poll_option_id: None,
            checklist_task_id: None,
        };
        let json = serde_json::to_value(&rp).unwrap();
        assert!(json.get("message_id").is_none());
        assert_eq!(json["ephemeral_message_id"], 99);
    }
}

#[cfg(test)]
mod community {
    use rustigram_types::community::{Community, CommunityChatAdded, CommunityChatRemoved};

    #[test]
    fn community_round_trip() {
        let c = Community {
            id: 555,
            name: "Rustaceans".to_owned(),
        };
        let json = serde_json::to_string(&c).unwrap();
        let back: Community = serde_json::from_str(&json).unwrap();
        assert_eq!(back.id, 555);
        assert_eq!(back.name, "Rustaceans");
    }

    #[test]
    fn community_chat_added_round_trip() {
        let added = CommunityChatAdded {
            community: Community {
                id: 1,
                name: "Group".to_owned(),
            },
        };
        let json = serde_json::to_value(&added).unwrap();
        assert_eq!(json["community"]["id"], 1);
    }

    #[test]
    fn community_chat_removed_serialises_empty_object() {
        let removed = CommunityChatRemoved::default();
        let json = serde_json::to_value(&removed).unwrap();
        assert!(json.as_object().unwrap().is_empty());
    }

    #[test]
    fn chat_full_info_deserialises_community() {
        let json = r#"{
            "id": 10,
            "type": "supergroup",
            "community": { "id": 20, "name": "Rustaceans" }
        }"#;
        let info: rustigram_types::chat::ChatFullInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.community.map(|c| c.id), Some(20));
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

    #[test]
    fn deserialises_subscription_update() {
        let json = r#"{
            "update_id": 7,
            "subscription": {
                "user": { "id": 1, "is_bot": false, "first_name": "Test" },
                "invoice_payload": "premium_monthly",
                "state": "active"
            }
        }"#;
        let update: Update = serde_json::from_str(json).unwrap();
        assert!(matches!(update.kind, UpdateKind::Subscription(_)));
        assert_eq!(update.from().map(|u| u.id), Some(1));

        if let UpdateKind::Subscription(sub) = &update.kind {
            assert_eq!(sub.state, "active");
            assert_eq!(sub.invoice_payload, "premium_monthly");
        }
    }
}
