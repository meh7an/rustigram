use rustigram_types::passport::PassportElementError;

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
    /// Errors describing why the submitted documents are unacceptable.
    errors: Vec<PassportElementError>,
}

/// Builder for the [`setPassportDataErrors`](https://core.telegram.org/bots/api#setpassportdataerrors) method.
///
/// Informs a user that some Telegram Passport elements they provided contain
/// errors. The user will not be able to re-submit their Passport to the bot
/// until the errors are fixed — the contents of the affected field must change.
///
pub struct SetPassportDataErrors {
    client: BotClient,
    params: SetPassportDataErrorsParams,
}

impl SetPassportDataErrors {
    pub(crate) fn new(client: BotClient, user_id: i64, errors: Vec<PassportElementError>) -> Self {
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
