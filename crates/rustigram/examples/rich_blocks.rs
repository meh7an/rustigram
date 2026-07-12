//! Demonstrates sending a rich message built from explicit blocks (Bot API
//! 10.2) — a section heading, a paragraph, and a list — instead of HTML or
//! Markdown.
//!
//! Run with:
//!     BOT_TOKEN=<your-token> cargo run --example rich_blocks

use rustigram::prelude::*;
use rustigram_types::rich_message::{
    InputRichBlock, InputRichBlockListItem, InputRichBlockParagraph, InputRichBlockSectionHeading,
    InputRichMessage, RichText,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN not set");
    let bot = Bot::new(token)?;

    bot.dispatcher()
        .on(filters::command("rich"), handler_fn(rich_handler))
        .build()
        .polling()
        .await?;

    Ok(())
}

async fn rich_handler(ctx: Context) -> BotResult<()> {
    let Some(chat_id) = ctx.chat_id() else {
        return Ok(());
    };

    let blocks = vec![
        InputRichBlock::SectionHeading(InputRichBlockSectionHeading {
            text: RichText::Plain("Rich message blocks".to_owned()),
            size: 2,
        }),
        InputRichBlock::Paragraph(InputRichBlockParagraph {
            text: RichText::Plain(
                "This message was built entirely from InputRichBlock values — no HTML or Markdown."
                    .to_owned(),
            ),
        }),
        InputRichBlock::List(rustigram_types::rich_message::InputRichBlockList {
            items: vec![
                InputRichBlockListItem {
                    blocks: vec![InputRichBlock::Paragraph(InputRichBlockParagraph {
                        text: RichText::Plain("Paragraphs".to_owned()),
                    })],
                    has_checkbox: None,
                    is_checked: None,
                    value: None,
                    kind: None,
                },
                InputRichBlockListItem {
                    blocks: vec![InputRichBlock::Paragraph(InputRichBlockParagraph {
                        text: RichText::Plain("Headings".to_owned()),
                    })],
                    has_checkbox: None,
                    is_checked: None,
                    value: None,
                    kind: None,
                },
                InputRichBlockListItem {
                    blocks: vec![InputRichBlock::Paragraph(InputRichBlockParagraph {
                        text: RichText::Plain("Lists — like this one".to_owned()),
                    })],
                    has_checkbox: None,
                    is_checked: None,
                    value: None,
                    kind: None,
                },
            ],
        }),
    ];

    ctx.bot
        .send_rich_message(chat_id, InputRichMessage::from_blocks(blocks))
        .await?;

    Ok(())
}
