//! Webhook-based bot.
//!
//! Before running, make sure your server is reachable via HTTPS and set the
//! webhook URL:
//!
//!     BOT_TOKEN=<token> WEBHOOK_URL=https://example.com cargo run --example webhook_bot
//!
//! The server listens on 0.0.0.0:8443 by default.

use rustigram::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN not set");
    let webhook_url = std::env::var("WEBHOOK_URL").expect("WEBHOOK_URL not set");

    let bot = Bot::new(&token)?;
    let secret = "super_secret_token_change_me";

    // Register the webhook with Telegram.
    bot.client
        .set_webhook(&webhook_url)
        .secret_token(secret)
        .drop_pending_updates(true)
        .await?;

    tracing::info!("Webhook set to {}", webhook_url);

    let addr: std::net::SocketAddr = "0.0.0.0:8443".parse()?;

    bot.dispatcher()
        .on(
            filters::command("start"),
            handler_fn(|ctx: Context| async move {
                if let Some(r) = ctx.reply("Hello via webhook!") {
                    r.await?;
                }
                Ok(())
            }),
        )
        .on(
            filters::message(),
            handler_fn(|ctx: Context| async move {
                if let (Some(text), Some(chat_id)) = (ctx.text(), ctx.chat_id()) {
                    ctx.bot
                        .send_message(chat_id, format!("Echo: {text}"))
                        .await?;
                }
                Ok(())
            }),
        )
        .build()
        .webhook(addr)
        .await?;

    Ok(())
}
