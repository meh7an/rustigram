use crate::client::BotClient;
use crate::error::Result;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

// ─── setPassportDataErrors ────────────────────────────────────────────────────

#[derive(Serialize)]
struct SetPassportDataErrorsParams {
    user_id: i64,
    /// Array of `PassportElementError` objects describing the errors.
    ///
    /// Uses `serde_json::Value` since `PassportElementError` is a complex enum
    /// defined in `rustigram_types::passport`. Construct each error using the
    /// appropriate variant from that module and serialise with `serde_json::to_value`.
    errors: Vec<serde_json::Value>,
}

/// Builder for the [`setPassportDataErrors`](https://core.telegram.org/bots/api#setpassportdataerrors) method.
///
/// Informs a user that some Telegram Passport elements they provided contain
/// errors. The user will not be able to re-submit their Passport to the bot
/// until the errors are fixed — the contents of the affected field must change.
///
/// The `errors` parameter accepts `Vec<serde_json::Value>`. Construct each
/// element from `rustigram_types::passport::PassportElementError` variants
/// and serialise with `serde_json::to_value(&error)`.
pub struct SetPassportDataErrors {
    client: BotClient,
    params: SetPassportDataErrorsParams,
}

impl SetPassportDataErrors {
    pub(crate) fn new(client: BotClient, user_id: i64, errors: Vec<serde_json::Value>) -> Self {
        Self {
            client,
            params: SetPassportDataErrorsParams { user_id, errors },
        }
    }
}

impl IntoFuture for SetPassportDataErrors {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("setPassportDataErrors", &self.params)
                .await
        })
    }
}
