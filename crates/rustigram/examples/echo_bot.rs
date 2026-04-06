//! Echo bot — the classic "Hello World" of Telegram bots.
//!
//! Run with:
//!     BOT_TOKEN=<your-token> cargo run --example echo_bot

use rustigram::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN env var not set");
    let bot = Bot::new(token)?;

    tracing::info!("Bot started: @{}", bot.client.get_me().await?.username.unwrap_or_default());

    bot.dispatcher()
        .on(filters::command("start"), handler_fn(start_handler))
        .on(filters::command("help"),  handler_fn(help_handler))
        .on(filters::message(),        handler_fn(echo_handler))
        .build()
        .polling()
        .await?;

    Ok(())
}

async fn start_handler(ctx: Context) -> BotResult<()> {
    if let Some(reply) = ctx.reply("<b>Welcome!</b>\n\nSend me any message and I'll echo it back.\nTry /help to see all commands.") {
        reply.parse_mode(ParseMode::HTML).await?;
    }
    Ok(())
}

async fn help_handler(ctx: Context) -> BotResult<()> {
    let text = "\
/start — welcome message
/help  — this list of commands";
    if let Some(reply) = ctx.reply(text) {
        reply.await?;
    }
    Ok(())
}

async fn echo_handler(ctx: Context) -> BotResult<()> {
    if let (Some(text), Some(chat_id)) = (ctx.text(), ctx.chat_id()) {
        ctx.bot.send_message(chat_id, text).await?;
    }
    Ok(())
}
