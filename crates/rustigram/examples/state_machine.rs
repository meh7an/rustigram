//! FSM-based conversation — collecting user name and email.
//!
//! Demonstrates `DialogueStorage` for per-user state tracking.

use rustigram::prelude::*;
use rustigram_bot::state::DialogueStorage;
use std::sync::Arc;

#[derive(Clone, Debug)]
enum RegistrationState {
    AwaitingName,
    AwaitingEmail { name: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN not set");
    let bot = Bot::new(token)?;

    // Shared dialogue state store.
    let storage = Arc::new(DialogueStorage::new());
    let storage_clone = storage.clone();

    bot.dispatcher()
        .on(filters::command("register"), {
            let storage = storage.clone();
            handler_fn(move |ctx: Context| {
                let storage = storage.clone();
                async move { start_registration(ctx, storage).await }
            })
        })
        .on(filters::message(), {
            let storage = storage_clone;
            handler_fn(move |ctx: Context| {
                let storage = storage.clone();
                async move { handle_text(ctx, storage).await }
            })
        })
        .build()
        .polling()
        .await?;

    Ok(())
}

async fn start_registration(ctx: Context, storage: Arc<DialogueStorage>) -> BotResult<()> {
    let (chat_id, user_id) = match (ctx.chat_id(), ctx.from_id()) {
        (Some(ChatId::Id(cid)), Some(uid)) => (cid, uid),
        _ => return Ok(()),
    };

    storage.set(chat_id, user_id, RegistrationState::AwaitingName);

    if let Some(r) = ctx.reply("What is your name?") {
        r.await?;
    }
    Ok(())
}

async fn handle_text(ctx: Context, storage: Arc<DialogueStorage>) -> BotResult<()> {
    let (chat_id, user_id) = match (ctx.chat_id(), ctx.from_id()) {
        (Some(ChatId::Id(cid)), Some(uid)) => (cid, uid),
        _ => return Ok(()),
    };

    let state: Option<RegistrationState> = storage.get(chat_id, user_id);
    let text = match ctx.text() {
        Some(t) => t.to_owned(),
        None => return Ok(()),
    };

    match state {
        Some(RegistrationState::AwaitingName) => {
            storage.set(
                chat_id,
                user_id,
                RegistrationState::AwaitingEmail { name: text.clone() },
            );
            if let Some(r) = ctx.reply(format!("Nice to meet you, {text}! What is your email?")) {
                r.await?;
            }
        }
        Some(RegistrationState::AwaitingEmail { name }) => {
            storage.remove(chat_id, user_id);
            if let Some(r) = ctx.reply(format!("Registered: {name} <{text}>. All done! ✅")) {
                r.await?;
            }
        }
        None => {
            // No active dialogue — do nothing.
        }
    }
    Ok(())
}
