//! Reacting to service messages — the events Telegram posts into a chat when
//! something happens to the chat itself, rather than when someone sends text.
//!
//! Before Bot API 10.2 support landed these arrived as a `Message` with every
//! interesting field `None`: the data was on the wire, but nothing typed was
//! there to receive it. This example handles four of them.
//!
//! Run with:
//!     BOT_TOKEN=<your-token> cargo run --example service_messages
//!
//! Then add the bot to a group and start a video chat, boost the chat, create a
//! forum topic, or run a giveaway.

use rustigram::prelude::*;
use rustigram_bot::filter::filter_fn;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN not set");
    let bot = Bot::new(token)?;

    // Service messages carry no text, so the text filters never match them.
    // Filter on the field itself instead.
    bot.dispatcher()
        .on(
            filter_fn(|ctx: &Context| {
                ctx.message().is_some_and(|m| {
                    m.video_chat_started.is_some()
                        || m.video_chat_ended.is_some()
                        || m.video_chat_participants_invited.is_some()
                })
            }),
            handler_fn(video_chat),
        )
        .on(
            filter_fn(|ctx: &Context| ctx.message().is_some_and(|m| m.boost_added.is_some())),
            handler_fn(boost),
        )
        .on(
            filter_fn(|ctx: &Context| {
                ctx.message()
                    .is_some_and(|m| m.forum_topic_created.is_some())
            }),
            handler_fn(forum_topic),
        )
        .on(
            filter_fn(|ctx: &Context| ctx.message().is_some_and(|m| m.giveaway.is_some())),
            handler_fn(giveaway),
        )
        .build()
        .polling()
        .await?;

    Ok(())
}

async fn video_chat(ctx: Context) -> BotResult<()> {
    let Some(message) = ctx.message() else {
        return Ok(());
    };

    let note = if message.video_chat_started.is_some() {
        "Video chat started.".to_owned()
    } else if let Some(ended) = &message.video_chat_ended {
        format!("Video chat ended after {} seconds.", ended.duration)
    } else if let Some(invited) = &message.video_chat_participants_invited {
        let names: Vec<&str> = invited.users.iter().map(|u| u.first_name.as_str()).collect();
        format!("Invited to the video chat: {}.", names.join(", "))
    } else {
        return Ok(());
    };

    if let Some(reply) = ctx.reply(note) {
        reply.await?;
    }
    Ok(())
}

async fn boost(ctx: Context) -> BotResult<()> {
    let Some(added) = ctx.message().and_then(|m| m.boost_added.as_ref()) else {
        return Ok(());
    };
    if let Some(reply) = ctx.reply(format!("Thanks for {} boost(s).", added.boost_count)) {
        reply.await?;
    }
    Ok(())
}

async fn forum_topic(ctx: Context) -> BotResult<()> {
    let Some(created) = ctx.message().and_then(|m| m.forum_topic_created.as_ref()) else {
        return Ok(());
    };
    // `is_name_implicit` tells you Telegram derived the name from the first
    // message rather than the creator choosing it.
    let origin = if created.is_name_implicit.unwrap_or(false) {
        "auto-named"
    } else {
        "named by its creator"
    };
    if let Some(reply) = ctx.reply(format!("New topic \"{}\", {origin}.", created.name)) {
        reply.await?;
    }
    Ok(())
}

async fn giveaway(ctx: Context) -> BotResult<()> {
    let Some(giveaway) = ctx.message().and_then(|m| m.giveaway.as_ref()) else {
        return Ok(());
    };
    let prize = match giveaway.prize_star_count {
        Some(stars) => format!("{stars} Stars"),
        None => "Telegram Premium".to_owned(),
    };
    if let Some(reply) = ctx.reply(format!(
        "Giveaway running: {} winner(s), prize {prize}, across {} chat(s).",
        giveaway.winner_count,
        giveaway.chats.len()
    )) {
        reply.await?;
    }
    Ok(())
}
