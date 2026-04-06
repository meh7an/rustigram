//! Demonstrates inline keyboards and callback query handling.
//!
//! Run with:
//!     BOT_TOKEN=<your-token> cargo run --example inline_keyboard

use rustigram::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN not set");
    let bot = Bot::new(token)?;

    bot.dispatcher()
        .on(filters::command("menu"), handler_fn(menu_handler))
        .on(filters::callback_data("btn_yes"), handler_fn(yes_handler))
        .on(filters::callback_data("btn_no"), handler_fn(no_handler))
        .on(
            filters::callback_data_prefix("nav:"),
            handler_fn(nav_handler),
        )
        .build()
        .polling()
        .await?;

    Ok(())
}

async fn menu_handler(ctx: Context) -> BotResult<()> {
    let markup = InlineKeyboardMarkup::new()
        .row(vec![
            InlineKeyboardButton::callback("✅ Yes", "btn_yes"),
            InlineKeyboardButton::callback("❌ No", "btn_no"),
        ])
        .row(vec![
            InlineKeyboardButton::callback("◀ Back", "nav:back"),
            InlineKeyboardButton::callback("▶ Forward", "nav:forward"),
        ])
        .row(vec![InlineKeyboardButton::url(
            "Open rustigram repo",
            "https://github.com/your-org/rustigram",
        )]);

    if let Some(chat_id) = ctx.chat_id() {
        ctx.bot
            .send_message(chat_id, "Choose an option:")
            .reply_markup(ReplyMarkup::InlineKeyboard(markup))
            .await?;
    }
    Ok(())
}

async fn yes_handler(ctx: Context) -> BotResult<()> {
    if let Some(q) = ctx.callback_query() {
        ctx.bot
            .answer_callback_query(&q.id)
            .text("You chose Yes!")
            .show_alert(false)
            .await?;
        if let Some(chat_id) = ctx.chat_id() {
            ctx.bot.send_message(chat_id, "Great choice! 🎉").await?;
        }
    }
    Ok(())
}

async fn no_handler(ctx: Context) -> BotResult<()> {
    if let Some(q) = ctx.callback_query() {
        ctx.bot
            .answer_callback_query(&q.id)
            .alert("You chose No — that's okay too!")
            .await?;
    }
    Ok(())
}

async fn nav_handler(ctx: Context) -> BotResult<()> {
    if let Some(q) = ctx.callback_query() {
        let direction = q.data.as_deref().unwrap_or("").trim_start_matches("nav:");
        ctx.bot
            .answer_callback_query(&q.id)
            .text(format!("Navigating: {direction}"))
            .await?;
    }
    Ok(())
}
