/// Reaction types that can be set on messages.
///
/// A reaction is either a standard emoji (`ReactionType::Emoji`), a custom
/// emoji (`ReactionType::CustomEmoji`), or a paid star reaction
/// (`ReactionType::Paid`).
///
/// Use `BotClient::set_message_reaction` in `rustigram-api` to set or remove reactions.
pub use crate::message::ReactionType;
