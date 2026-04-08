//! Typed builder structs for every Telegram Bot API method.
//!
//! Every method is implemented as a builder that carries its required and
//! optional parameters, then executes via `.await`. All builders implement
//! `IntoFuture` so they can be `await`-ed directly.
//!
//! # Example
//!
//! ```rust,no_run
//! use rustigram_api::BotClient;
//!
//! # async fn example(client: BotClient) -> rustigram_api::Result<()> {
//! let msg = client
//!     .send_message(12345, "Hello!")
//!     .parse_mode(rustigram_types::message::ParseMode::HTML)
//!     .disable_notification(true)
//!     .await?;
//! # Ok(())
//! # }
//! ```

/// Bot identity and command configuration methods.
pub mod bot_settings;
/// Business account management methods.
pub mod business;
/// Chat membership and administration methods.
pub mod chat_management;
/// Message editing methods.
pub mod editing;
/// Forum topic management methods.
pub mod forum;
/// Game score and high score methods.
pub mod games;
/// Information retrieval methods.
pub mod getters;
/// Gift sending and management methods.
pub mod gifts;
/// Inline query handling methods.
pub mod inline;
/// Mini App keyboard button and emoji status methods.
pub mod miniapp;
/// Telegram Passport error reporting methods.
pub mod passport;
/// Payment and Stars methods.
pub mod payments;
/// Message reaction methods.
pub mod reactions;
/// Message and media sending methods.
pub mod sending;
/// Sticker set management methods.
pub mod stickers;
/// Story posting and management methods (business bots).
pub mod stories;
/// Update fetching and webhook configuration methods.
pub mod updates;
/// User and chat verification methods.
pub mod verification;

pub use bot_settings::*;
pub use business::*;
pub use chat_management::*;
pub use editing::*;
pub use forum::*;
pub use games::*;
pub use getters::*;
pub use gifts::*;
pub use inline::*;
pub use miniapp::*;
pub use passport::*;
pub use payments::*;
pub use reactions::*;
pub use sending::*;
pub use stickers::*;
pub use stories::*;
pub use updates::*;
pub use verification::*;
