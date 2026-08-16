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

#[cfg(test)]
mod service_messages {
    use rustigram_types::message::Message;
    use rustigram_types::{
        ChatBoostAdded, ChatShared, ForumTopicCreated, ForumTopicEdited, ProximityAlertTriggered,
        SharedUser, UsersShared, VideoChatEnded, VideoChatParticipantsInvited, VideoChatScheduled,
        VideoChatStarted, WriteAccessAllowed,
    };

    /// The four data-free service messages arrive as `{}`, not `null`, so they
    /// must deserialize from an empty object.
    #[test]
    fn empty_service_objects_deserialize_from_empty_json() {
        use rustigram_types::{
            ForumTopicClosed, ForumTopicReopened, GeneralForumTopicHidden,
            GeneralForumTopicUnhidden,
        };
        serde_json::from_str::<VideoChatStarted>("{}").unwrap();
        serde_json::from_str::<ForumTopicClosed>("{}").unwrap();
        serde_json::from_str::<ForumTopicReopened>("{}").unwrap();
        serde_json::from_str::<GeneralForumTopicHidden>("{}").unwrap();
        serde_json::from_str::<GeneralForumTopicUnhidden>("{}").unwrap();
    }

    #[test]
    fn video_chat_types_round_trip() {
        let scheduled: VideoChatScheduled =
            serde_json::from_str(r#"{"start_date":1700000000}"#).unwrap();
        assert_eq!(scheduled.start_date, 1_700_000_000);

        let ended: VideoChatEnded = serde_json::from_str(r#"{"duration":95}"#).unwrap();
        assert_eq!(ended.duration, 95);

        let invited: VideoChatParticipantsInvited = serde_json::from_str(
            r#"{"users":[{"id":42,"is_bot":false,"first_name":"Mehran"}]}"#,
        )
        .unwrap();
        assert_eq!(invited.users.len(), 1);
        assert_eq!(invited.users[0].id, 42);
    }

    #[test]
    fn forum_topic_created_round_trips() {
        let json = r#"{"name":"Bugs","icon_color":7322096,"is_name_implicit":true}"#;
        let created: ForumTopicCreated = serde_json::from_str(json).unwrap();
        assert_eq!(created.name, "Bugs");
        assert_eq!(created.icon_color, 7_322_096);
        assert_eq!(created.is_name_implicit, Some(true));
        assert!(created.icon_custom_emoji_id.is_none());

        // Optional fields must not be emitted when absent.
        let edited: ForumTopicEdited = serde_json::from_str(r#"{"name":"Renamed"}"#).unwrap();
        let back = serde_json::to_value(&edited).unwrap();
        assert_eq!(back["name"], "Renamed");
        assert!(back.get("icon_custom_emoji_id").is_none());
    }

    #[test]
    fn shared_entities_round_trip() {
        let shared: UsersShared = serde_json::from_str(
            r#"{"request_id":3,"users":[{"user_id":7,"username":"mehran"}]}"#,
        )
        .unwrap();
        assert_eq!(shared.request_id, 3);
        assert_eq!(shared.users[0].user_id, 7);
        assert_eq!(shared.users[0].username.as_deref(), Some("mehran"));

        let chat: ChatShared =
            serde_json::from_str(r#"{"request_id":4,"chat_id":-100123,"title":"Team"}"#).unwrap();
        assert_eq!(chat.chat_id, -100_123);
        assert_eq!(chat.title.as_deref(), Some("Team"));

        let user = SharedUser::default();
        assert_eq!(user.user_id, 0);
    }

    #[test]
    fn proximity_boost_and_write_access_round_trip() {
        let alert: ProximityAlertTriggered = serde_json::from_str(
            r#"{"traveler":{"id":1,"is_bot":false,"first_name":"A"},
                "watcher":{"id":2,"is_bot":false,"first_name":"B"},"distance":150}"#,
        )
        .unwrap();
        assert_eq!(alert.distance, 150);
        assert_eq!(alert.traveler.id, 1);
        assert_eq!(alert.watcher.id, 2);

        let boost: ChatBoostAdded = serde_json::from_str(r#"{"boost_count":4}"#).unwrap();
        assert_eq!(boost.boost_count, 4);

        let access: WriteAccessAllowed =
            serde_json::from_str(r#"{"from_request":true}"#).unwrap();
        assert_eq!(access.from_request, Some(true));
        assert!(access.web_app_name.is_none());
    }

    /// The regression this test exists to prevent: these fields used to be
    /// absent or `serde_json::Value`, so a service message deserialized into a
    /// `Message` where everything the bot cared about was `None`.
    #[test]
    fn message_carries_typed_service_fields() {
        let json = r#"{
            "message_id": 0,
            "date": 1700000000,
            "chat": { "id": -100999, "type": "supergroup" },
            "video_chat_started": {},
            "video_chat_ended": { "duration": 60 },
            "boost_added": { "boost_count": 2 },
            "forum_topic_created": { "name": "General", "icon_color": 7322096 },
            "connected_website": "example.com",
            "proximity_alert_triggered": {
                "traveler": { "id": 1, "is_bot": false, "first_name": "A" },
                "watcher": { "id": 2, "is_bot": false, "first_name": "B" },
                "distance": 20
            }
        }"#;
        let msg: Message = serde_json::from_str(json).unwrap();

        assert!(msg.video_chat_started.is_some());
        assert_eq!(msg.video_chat_ended.unwrap().duration, 60);
        assert_eq!(msg.boost_added.unwrap().boost_count, 2);
        assert_eq!(msg.forum_topic_created.unwrap().name, "General");
        assert_eq!(msg.connected_website.as_deref(), Some("example.com"));
        assert_eq!(msg.proximity_alert_triggered.unwrap().distance, 20);
    }
}

#[cfg(test)]
mod giveaways_gifts_and_paid_media {
    use rustigram_types::message::Message;
    use rustigram_types::{Giveaway, GiveawayCompleted, GiveawayWinners, PaidMedia, PaidMediaInfo};

    /// The `PaidMedia` enum is tagged by `type`; all four variants must resolve.
    #[test]
    fn paid_media_variants_resolve_by_tag() {
        let info: PaidMediaInfo = serde_json::from_str(
            r#"{"star_count":50,"paid_media":[
                {"type":"preview","width":640,"height":480},
                {"type":"photo","photo":[{"file_id":"a","file_unique_id":"b","width":1,"height":1}]}
            ]}"#,
        )
        .unwrap();
        assert_eq!(info.star_count, 50);
        assert!(matches!(info.paid_media[0], PaidMedia::Preview(_)));
        assert!(matches!(info.paid_media[1], PaidMedia::Photo(_)));

        // The tag must survive a round trip, or re-sending the value breaks.
        let back = serde_json::to_value(&info).unwrap();
        assert_eq!(back["paid_media"][0]["type"], "preview");
        assert_eq!(back["paid_media"][1]["type"], "photo");
    }

    #[test]
    fn giveaway_round_trips() {
        let g: Giveaway = serde_json::from_str(
            r#"{"chats":[{"id":-100,"type":"channel"}],"winners_selection_date":1700000000,
                "winner_count":10,"only_new_members":true,"prize_star_count":500}"#,
        )
        .unwrap();
        assert_eq!(g.winner_count, 10);
        assert_eq!(g.chats.len(), 1);
        assert_eq!(g.only_new_members, Some(true));
        assert_eq!(g.prize_star_count, Some(500));
        assert!(g.country_codes.is_none());

        let w: GiveawayWinners = serde_json::from_str(
            r#"{"chat":{"id":-100,"type":"channel"},"giveaway_message_id":5,
                "winners_selection_date":1700000000,"winner_count":2,
                "winners":[{"id":1,"is_bot":false,"first_name":"A"}]}"#,
        )
        .unwrap();
        assert_eq!(w.winners.len(), 1);
        assert_eq!(w.giveaway_message_id, 5);
    }

    /// `GiveawayCompleted.giveaway_message` is a boxed `Message`, so this
    /// exercises the type cycle actually resolving at runtime.
    #[test]
    fn giveaway_completed_nests_a_message() {
        let c: GiveawayCompleted = serde_json::from_str(
            r#"{"winner_count":3,"giveaway_message":{
                "message_id":9,"date":1700000000,"chat":{"id":-100,"type":"channel"}}}"#,
        )
        .unwrap();
        assert_eq!(c.winner_count, 3);
        assert_eq!(c.giveaway_message.unwrap().message_id, 9);
    }

    /// These seven fields were `Option<serde_json::Value>` until they were typed:
    /// present in name, so the audit reported them covered, but useless to a bot.
    #[test]
    fn formerly_untyped_message_fields_are_typed() {
        let json = r#"{
            "message_id": 1,
            "date": 1700000000,
            "chat": { "id": 1, "type": "private" },
            "invoice": {
                "title": "Pro", "description": "Plan", "start_parameter": "",
                "currency": "XTR", "total_amount": 100
            },
            "story": { "chat": { "id": 1, "type": "private" }, "id": 4 },
            "paid_media": { "star_count": 5, "paid_media": [{ "type": "preview" }] },
            "giveaway_created": { "prize_star_count": 100 }
        }"#;
        let msg: Message = serde_json::from_str(json).unwrap();

        assert_eq!(msg.invoice.unwrap().total_amount, 100);
        assert_eq!(msg.story.unwrap().id, 4);
        assert_eq!(msg.paid_media.unwrap().star_count, 5);
        assert_eq!(msg.giveaway_created.unwrap().prize_star_count, Some(100));
    }

    #[test]
    fn external_reply_carries_giveaway_and_invoice() {
        let json = r#"{
            "message_id": 1,
            "date": 1700000000,
            "chat": { "id": 1, "type": "private" },
            "external_reply": {
                "origin": { "type": "hidden_user", "date": 1, "sender_user_name": "X" },
                "giveaway": {
                    "chats": [], "winners_selection_date": 1, "winner_count": 1
                }
            }
        }"#;
        let msg: Message = serde_json::from_str(json).unwrap();
        let reply = msg.external_reply.unwrap();
        assert_eq!(reply.giveaway.unwrap().winner_count, 1);
        assert!(reply.invoice.is_none());
    }
}

#[cfg(test)]
mod backgrounds_and_business {
    use rustigram_types::chat::ChatFullInfo;
    use rustigram_types::{BackgroundFill, BackgroundType, Birthdate, BusinessOpeningHours};

    #[test]
    fn background_fill_variants_resolve_by_tag() {
        let solid: BackgroundFill =
            serde_json::from_str(r#"{"type":"solid","color":16777215}"#).unwrap();
        assert!(matches!(solid, BackgroundFill::Solid(ref s) if s.color == 16_777_215));

        let freeform: BackgroundFill =
            serde_json::from_str(r#"{"type":"freeform_gradient","colors":[1,2,3]}"#).unwrap();
        assert!(matches!(freeform, BackgroundFill::FreeformGradient(ref f) if f.colors.len() == 3));

        // The tag must survive serialisation, including the snake_case form.
        assert_eq!(serde_json::to_value(&freeform).unwrap()["type"], "freeform_gradient");
    }

    /// `BackgroundTypePattern` nests a `BackgroundFill`, so this covers a tagged
    /// enum inside a tagged enum.
    #[test]
    fn background_type_nests_a_fill() {
        let bg: BackgroundType = serde_json::from_str(
            r#"{"type":"pattern",
                "document":{"file_id":"a","file_unique_id":"b"},
                "fill":{"type":"gradient","top_color":1,"bottom_color":2,"rotation_angle":45},
                "intensity":50}"#,
        )
        .unwrap();
        match bg {
            BackgroundType::Pattern(p) => {
                assert_eq!(p.intensity, 50);
                assert!(matches!(p.fill, BackgroundFill::Gradient(g) if g.rotation_angle == 45));
            }
            other => panic!("expected pattern, got {other:?}"),
        }
    }

    #[test]
    fn business_and_birthdate_round_trip() {
        let hours: BusinessOpeningHours = serde_json::from_str(
            r#"{"time_zone_name":"Europe/Istanbul",
                "opening_hours":[{"opening_minute":540,"closing_minute":1080}]}"#,
        )
        .unwrap();
        assert_eq!(hours.time_zone_name, "Europe/Istanbul");
        assert_eq!(hours.opening_hours[0].opening_minute, 540);

        let bd: Birthdate = serde_json::from_str(r#"{"day":14,"month":7}"#).unwrap();
        assert_eq!((bd.day, bd.month), (14, 7));
        assert!(bd.year.is_none());
        // Absent optional fields must not be emitted.
        assert!(serde_json::to_value(&bd).unwrap().get("year").is_none());
    }

    /// `ChatFullInfo` was missing 18 of its 54 fields.
    #[test]
    fn chat_full_info_carries_the_business_block() {
        let json = r#"{
            "id": 1, "type": "private",
            "accent_color_id": 3, "max_reaction_count": 11,
            "accepted_gift_types": {
                "unlimited_gifts": true, "limited_gifts": false, "unique_gifts": false,
                "premium_subscription": false, "gifts_from_channels": false
            },
            "birthdate": { "day": 1, "month": 2, "year": 1990 },
            "business_location": { "address": "Somewhere" },
            "paid_message_star_count": 25,
            "guard_bot": { "id": 9, "is_bot": true, "first_name": "Guard" }
        }"#;
        let info: ChatFullInfo = serde_json::from_str(json).unwrap();

        assert_eq!(info.accent_color_id, 3);
        assert_eq!(info.max_reaction_count, 11);
        assert!(info.accepted_gift_types.unlimited_gifts);
        assert_eq!(info.birthdate.unwrap().year, Some(1990));
        assert_eq!(info.business_location.unwrap().address, "Somewhere");
        assert_eq!(info.paid_message_star_count, Some(25));
        // guard_bot lives on ChatFullInfo, matching the spec — it used to be on Chat.
        assert_eq!(info.guard_bot.unwrap().id, 9);
    }

    /// An older Bot API server may omit fields the current spec marks required;
    /// `#[serde(default)]` keeps those responses decodable.
    #[test]
    fn chat_full_info_tolerates_missing_required_scalars() {
        let info: ChatFullInfo = serde_json::from_str(r#"{"id":1,"type":"private"}"#).unwrap();
        assert_eq!(info.accent_color_id, 0);
        assert_eq!(info.max_reaction_count, 0);
        assert!(!info.accepted_gift_types.unlimited_gifts);
    }
}

#[cfg(test)]
mod unions_and_reconciliation {
    use rustigram_types::rich_message::{RichBlock, RichText};
    use rustigram_types::{ChatBoostSource, MaybeInaccessibleMessage};

    #[test]
    fn chat_boost_source_resolves_by_source_tag() {
        let premium: ChatBoostSource = serde_json::from_str(
            r#"{"source":"premium","user":{"id":1,"is_bot":false,"first_name":"A"}}"#,
        )
        .unwrap();
        assert!(matches!(premium, ChatBoostSource::Premium(ref p) if p.user.id == 1));

        let giveaway: ChatBoostSource = serde_json::from_str(
            r#"{"source":"giveaway","giveaway_message_id":12,"is_unclaimed":true}"#,
        )
        .unwrap();
        match giveaway {
            ChatBoostSource::Giveaway(g) => {
                assert_eq!(g.giveaway_message_id, 12);
                assert_eq!(g.is_unclaimed, Some(true));
                assert!(g.user.is_none());
            }
            other => panic!("expected giveaway, got {other:?}"),
        }
    }

    /// The two variants are structurally indistinguishable — an inaccessible
    /// message is a strict subset of a message — so dispatch is on `date == 0`.
    #[test]
    fn maybe_inaccessible_dispatches_on_date() {
        let gone: MaybeInaccessibleMessage = serde_json::from_str(
            r#"{"chat":{"id":1,"type":"private"},"message_id":5,"date":0}"#,
        )
        .unwrap();
        assert!(matches!(gone, MaybeInaccessibleMessage::Inaccessible(ref m) if m.message_id == 5));

        let live: MaybeInaccessibleMessage = serde_json::from_str(
            r#"{"chat":{"id":1,"type":"private"},"message_id":5,"date":1700000000,"text":"hi"}"#,
        )
        .unwrap();
        match live {
            MaybeInaccessibleMessage::Message(m) => assert_eq!(m.text.as_deref(), Some("hi")),
            other => panic!("expected accessible message, got {other:?}"),
        }
    }

    /// Regression: a map block used to fail with "missing field `latitude`",
    /// because the crate flattened what Telegram sends as a nested `location`.
    #[test]
    fn rich_block_map_deserializes_from_spec_shape() {
        let block: RichBlock = serde_json::from_str(
            r#"{"type":"map","location":{"latitude":41.0,"longitude":29.0},
                "zoom":15,"width":600,"height":400}"#,
        )
        .unwrap();
        match block {
            RichBlock::Map(m) => {
                assert!((m.location.latitude - 41.0).abs() < f64::EPSILON);
                assert_eq!(m.zoom, 15);
            }
            other => panic!("expected map, got {other:?}"),
        }
    }

    /// The spec calls this field `name`; the crate exposes it as `footnote_name`
    /// for readability, so the rename must hold on the wire in both directions.
    #[test]
    fn rich_text_reference_uses_the_spec_field_name() {
        let json = serde_json::to_value(rustigram_types::rich_message::RichTextReference {
            kind: "reference".to_owned(),
            text: RichText::Plain("1".to_owned()),
            footnote_name: "fn1".to_owned(),
        })
        .unwrap();
        assert_eq!(json["name"], "fn1");
        assert!(json.get("footnote_name").is_none());
    }
}
