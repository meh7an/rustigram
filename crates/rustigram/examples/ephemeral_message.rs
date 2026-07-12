//! Demonstrates ephemeral messages (Bot API 10.2) — replies visible only to
//! the user who triggered them and the bot, even inside a group chat.
//!
//! Run with:
//!     BOT_TOKEN=<your-token> cargo run --example ephemeral_message

use rustigram::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN not set");
    let bot = Bot::new(token)?;

    bot.dispatcher()
        .on(filters::command("secret"), handler_fn(secret_handler))
        .on(filters::callback_data("reveal"), handler_fn(reveal_handler))
        .build()
        .polling()
        .await?;

    Ok(())
}

/// Posts a normal, visible-to-everyone message with a button.
async fn secret_handler(ctx: Context) -> BotResult<()> {
    let markup = InlineKeyboardMarkup::new().row(vec![InlineKeyboardButton::callback(
        "Tap for a secret",
        "reveal",
    )]);

    if let Some(chat_id) = ctx.chat_id() {
        ctx.bot
            .send_message(chat_id, "Tap the button below.")
            .reply_markup(ReplyMarkup::InlineKeyboard(markup))
            .await?;
    }
    Ok(())
}

/// Sends an ephemeral reply to whoever tapped the button, edits it a moment
/// later, then deletes it — none of it visible to other chat members.
///
/// `callback_query_id` must be supplied within 15 seconds of the button tap
/// (see the sending rules for ephemeral messages); it identifies the exact
/// client that should receive the reply.
async fn reveal_handler(ctx: Context) -> BotResult<()> {
    let (Some(query), Some(chat_id), Some(user_id)) =
        (ctx.callback_query(), ctx.chat_id(), ctx.from_id())
    else {
        return Ok(());
    };

    ctx.bot.answer_callback_query(&query.id).await?;

    let ephemeral = ctx
        .bot
        .send_message(chat_id.clone(), "🤫 This is just between us.")
        .receiver_user_id(user_id)
        .callback_query_id(&query.id)
        .await?;

    let Some(ephemeral_id) = ephemeral.ephemeral_message_id else {
        return Ok(());
    };

    ctx.bot
        .edit_ephemeral_message_text(
            chat_id.clone(),
            user_id,
            ephemeral_id,
            "Still just between us — vanishing in 3 seconds.",
        )
        .await?;

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    ctx.bot
        .delete_ephemeral_message(chat_id, user_id, ephemeral_id)
        .await?;

    Ok(())
}
