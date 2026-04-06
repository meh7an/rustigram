use rustigram_types::update::UpdateKind;

use crate::context::Context;

/// A predicate evaluated against an incoming [`Context`].
///
/// Filters are `Send + Sync + 'static` and cheap to clone, making them safe
/// to share across tasks. Combine filters with [`FilterExt::and`],
/// [`FilterExt::or`], and [`FilterExt::not`].
///
/// Implement this trait to create custom filters:
///
/// ```rust,ignore
/// use rustigram_bot::filter::Filter;
/// use rustigram_bot::Context;
///
/// #[derive(Clone)]
/// struct HasPhotoFilter;
///
/// impl Filter for HasPhotoFilter {
///     fn check(&self, ctx: &Context) -> bool {
///         ctx.message().and_then(|m| m.photo.as_ref()).is_some()
///     }
/// }
/// ```
pub trait Filter: Send + Sync + 'static {
    /// Returns `true` if this filter matches the given context.
    fn check(&self, ctx: &Context) -> bool;
}

/// Extension methods for composing [`Filter`] values.
///
/// Automatically implemented for every type that implements [`Filter`].
pub trait FilterExt: Filter + Sized + Clone {
    /// Passes only when both `self` and `other` match.
    fn and<F: Filter + Clone>(self, other: F) -> And<Self, F> {
        And {
            left: self,
            right: other,
        }
    }

    /// Passes when `self` or `other` (or both) match.
    fn or<F: Filter + Clone>(self, other: F) -> Or<Self, F> {
        Or {
            left: self,
            right: other,
        }
    }

    /// Inverts this filter.
    fn not(self) -> Not<Self> {
        Not { inner: self }
    }
}

impl<F: Filter + Clone> FilterExt for F {}

// ─── Combinators ─────────────────────────────────────────────────────────────

/// Combines two filters with logical AND: passes only if both filters pass.
#[derive(Clone)]
pub struct And<L, R> {
    left: L,
    right: R,
}
impl<L: Filter, R: Filter> Filter for And<L, R> {
    fn check(&self, ctx: &Context) -> bool {
        self.left.check(ctx) && self.right.check(ctx)
    }
}

/// Combines two filters with logical OR: passes if either filter (or both) pass.
#[derive(Clone)]
pub struct Or<L, R> {
    left: L,
    right: R,
}
impl<L: Filter, R: Filter> Filter for Or<L, R> {
    fn check(&self, ctx: &Context) -> bool {
        self.left.check(ctx) || self.right.check(ctx)
    }
}

/// Inverts a filter: passes when the inner filter fails, and vice versa.
#[derive(Clone)]
pub struct Not<F> {
    inner: F,
}
impl<F: Filter> Filter for Not<F> {
    fn check(&self, ctx: &Context) -> bool {
        !self.inner.check(ctx)
    }
}

// ─── Function filter ──────────────────────────────────────────────────────────

#[derive(Clone)]
/// Wraps a plain function or closure as a [`Filter`].
pub struct FnFilter<F>(pub F);

impl<F: Fn(&Context) -> bool + Send + Sync + Clone + 'static> Filter for FnFilter<F> {
    fn check(&self, ctx: &Context) -> bool {
        (self.0)(ctx)
    }
}

/// Creates a [`Filter`] from any closure with signature `fn(&Context) -> bool`.
pub fn filter_fn<F>(f: F) -> FnFilter<F>
where
    F: Fn(&Context) -> bool + Send + Sync + Clone + 'static,
{
    FnFilter(f)
}

// ─── Built-in filters ─────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
/// Passes only for [`Message`](rustigram_types::update::UpdateKind::Message) updates.
pub struct MessageFilter;
impl Filter for MessageFilter {
    fn check(&self, ctx: &Context) -> bool {
        matches!(ctx.update.kind, UpdateKind::Message(_))
    }
}

/// Passes only for `EditedMessage` updates.
#[derive(Clone, Copy)]
/// Passes only for [`EditedMessage`](rustigram_types::update::UpdateKind::EditedMessage) updates.
pub struct EditedMessageFilter;
impl Filter for EditedMessageFilter {
    fn check(&self, ctx: &Context) -> bool {
        matches!(ctx.update.kind, UpdateKind::EditedMessage(_))
    }
}

/// Passes only for `CallbackQuery` updates.
#[derive(Clone, Copy)]
/// Passes only for [`CallbackQuery`](rustigram_types::update::UpdateKind::CallbackQuery) updates.
pub struct CallbackQueryFilter;
impl Filter for CallbackQueryFilter {
    fn check(&self, ctx: &Context) -> bool {
        matches!(ctx.update.kind, UpdateKind::CallbackQuery(_))
    }
}

/// Passes only for `InlineQuery` updates.
#[derive(Clone, Copy)]
/// Passes only for [`InlineQuery`](rustigram_types::update::UpdateKind::InlineQuery) updates.
pub struct InlineQueryFilter;
impl Filter for InlineQueryFilter {
    fn check(&self, ctx: &Context) -> bool {
        matches!(ctx.update.kind, UpdateKind::InlineQuery(_))
    }
}

#[derive(Clone)]
/// Passes when the message is a bot command matching `command`.
///
/// The check is case-insensitive and strips the leading `/` and any
/// `@BotName` suffix automatically.
pub struct CommandFilter {
    command: String,
}

impl CommandFilter {
    /// Creates a new filter matching the command `command`.
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
        }
    }
}

impl Filter for CommandFilter {
    fn check(&self, ctx: &Context) -> bool {
        ctx.command()
            .map_or(false, |cmd| cmd.eq_ignore_ascii_case(&self.command))
    }
}

#[derive(Clone)]
/// Passes when the message text exactly equals `text`.
pub struct TextFilter {
    text: String,
}

impl TextFilter {
    /// Creates a new filter matching the exact text `text`.
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Filter for TextFilter {
    fn check(&self, ctx: &Context) -> bool {
        ctx.text().map_or(false, |t| t == self.text)
    }
}

#[derive(Clone)]
/// Passes when the message text contains `needle` as a substring.
pub struct TextContainsFilter {
    needle: String,
}

impl TextContainsFilter {
    /// Creates a new filter matching text containing `needle`.
    pub fn new(needle: impl Into<String>) -> Self {
        Self {
            needle: needle.into(),
        }
    }
}

impl Filter for TextContainsFilter {
    fn check(&self, ctx: &Context) -> bool {
        ctx.text()
            .map_or(false, |t| t.contains(self.needle.as_str()))
    }
}

#[derive(Clone)]
/// Passes when the callback query data exactly equals `data`.
pub struct CallbackDataFilter {
    data: String,
}

impl CallbackDataFilter {
    /// Creates a new filter matching the callback data `data`.
    pub fn new(data: impl Into<String>) -> Self {
        Self { data: data.into() }
    }
}

impl Filter for CallbackDataFilter {
    fn check(&self, ctx: &Context) -> bool {
        ctx.callback_query()
            .and_then(|q| q.data.as_deref())
            .map_or(false, |d| d == self.data)
    }
}

#[derive(Clone)]
/// Passes when the callback query data starts with `prefix`.
pub struct CallbackDataPrefixFilter {
    prefix: String,
}

impl CallbackDataPrefixFilter {
    /// Creates a new filter matching callback data starting with `prefix`.
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
        }
    }
}

impl Filter for CallbackDataPrefixFilter {
    fn check(&self, ctx: &Context) -> bool {
        ctx.callback_query()
            .and_then(|q| q.data.as_deref())
            .map_or(false, |d| d.starts_with(self.prefix.as_str()))
    }
}

#[derive(Clone, Copy)]
/// Passes only for messages in private chats.
pub struct PrivateChatFilter;
impl Filter for PrivateChatFilter {
    fn check(&self, ctx: &Context) -> bool {
        ctx.message().map_or(false, |m| {
            matches!(m.chat.kind, rustigram_types::chat::ChatType::Private)
        })
    }
}

#[derive(Clone, Copy)]
/// Passes only for messages in group and supergroup chats.
pub struct GroupFilter;
impl Filter for GroupFilter {
    fn check(&self, ctx: &Context) -> bool {
        ctx.message().map_or(false, |m| {
            matches!(
                m.chat.kind,
                rustigram_types::chat::ChatType::Group
                    | rustigram_types::chat::ChatType::Supergroup
            )
        })
    }
}

/// Convenience constructors for all built-in filters.
///
/// Import this module and call functions to create filters:
///
/// ```rust,ignore
/// use rustigram_bot::filter::filters;
/// use rustigram_bot::filter::FilterExt;
///
/// let f = filters::command("start")
///             .and(filters::private());
/// ```
pub mod filters {
    use super::*;

    /// Passes for any `Message` update.
    pub fn message() -> MessageFilter {
        MessageFilter
    }
    /// Passes for any `EditedMessage` update.
    pub fn edited_message() -> EditedMessageFilter {
        EditedMessageFilter
    }
    /// Passes for any `CallbackQuery` update.
    pub fn callback_query() -> CallbackQueryFilter {
        CallbackQueryFilter
    }
    /// Passes for any `InlineQuery` update.
    pub fn inline_query() -> InlineQueryFilter {
        InlineQueryFilter
    }
    /// Passes when the message is the given bot command (case-insensitive).
    pub fn command(cmd: impl Into<String>) -> CommandFilter {
        CommandFilter::new(cmd)
    }
    /// Passes when the message text exactly equals `t`.
    pub fn text(t: impl Into<String>) -> TextFilter {
        TextFilter::new(t)
    }
    /// Passes when the message text contains `needle` as a substring.
    pub fn text_contains(needle: impl Into<String>) -> TextContainsFilter {
        TextContainsFilter::new(needle)
    }
    /// Passes when the callback query data exactly equals `data`.
    pub fn callback_data(data: impl Into<String>) -> CallbackDataFilter {
        CallbackDataFilter::new(data)
    }
    /// Passes when the callback query data starts with `prefix`.
    pub fn callback_data_prefix(prefix: impl Into<String>) -> CallbackDataPrefixFilter {
        CallbackDataPrefixFilter::new(prefix)
    }
    /// Passes for messages in private chats.
    pub fn private() -> PrivateChatFilter {
        PrivateChatFilter
    }
    /// Passes for messages in group and supergroup chats.
    pub fn group() -> GroupFilter {
        GroupFilter
    }
    /// Always passes — useful as a catch-all fallback route.
    pub fn any() -> FnFilter<fn(&Context) -> bool> {
        FnFilter(|_| true)
    }
}
