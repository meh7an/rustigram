use crate::client::BotClient;
use crate::error::Result;
use reqwest::multipart::{Form, Part};
use rustigram_types::sticker::{
    InputSticker, MaskPosition, Sticker, StickerFormat, StickerSet, StickerType,
};
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Serialize)]
struct GetStickerSetParams {
    name: String,
}

/// Builder for the [`getStickerSet`](https://core.telegram.org/bots/api#getstickerset) method.
pub struct GetStickerSet {
    client: BotClient,
    params: GetStickerSetParams,
}
impl GetStickerSet {
    pub(crate) fn new(client: BotClient, name: impl Into<String>) -> Self {
        Self {
            client,
            params: GetStickerSetParams { name: name.into() },
        }
    }
}
impl IntoFuture for GetStickerSet {
    type Output = Result<StickerSet>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.client.post_json("getStickerSet", &self.params).await })
    }
}

#[derive(Serialize)]
struct GetCustomEmojiStickersParams {
    custom_emoji_ids: Vec<String>,
}

/// Builder for the [`getCustomEmojiStickers`](https://core.telegram.org/bots/api#getcustomemojistickers) method.
pub struct GetCustomEmojiStickers {
    client: BotClient,
    params: GetCustomEmojiStickersParams,
}
impl GetCustomEmojiStickers {
    pub(crate) fn new(client: BotClient, ids: Vec<impl Into<String>>) -> Self {
        Self {
            client,
            params: GetCustomEmojiStickersParams {
                custom_emoji_ids: ids.into_iter().map(Into::into).collect(),
            },
        }
    }
}
impl IntoFuture for GetCustomEmojiStickers {
    type Output = Result<Vec<Sticker>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("getCustomEmojiStickers", &self.params)
                .await
        })
    }
}

/// Builder for the [`uploadStickerFile`](https://core.telegram.org/bots/api#uploadstickerfile) method.
pub struct UploadStickerFile {
    client: BotClient,
    user_id: i64,
    sticker: rustigram_types::file::InputFile,
    sticker_format: StickerFormat,
}
impl UploadStickerFile {
    pub(crate) fn new(
        client: BotClient,
        user_id: i64,
        sticker: rustigram_types::file::InputFile,
        format: StickerFormat,
    ) -> Self {
        Self {
            client,
            user_id,
            sticker,
            sticker_format: format,
        }
    }
}
impl IntoFuture for UploadStickerFile {
    type Output = Result<rustigram_types::file::File>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            match self.sticker {
                rustigram_types::file::InputFile::Bytes {
                    filename,
                    data,
                    mime_type,
                } => {
                    let part = Part::bytes(data)
                        .file_name(filename)
                        .mime_str(&mime_type)
                        .map_err(|e| crate::error::Error::Decode(e.to_string()))?;
                    let fmt = match self.sticker_format {
                        StickerFormat::Static => "static",
                        StickerFormat::Animated => "animated",
                        StickerFormat::Video => "video",
                    };
                    let form = Form::new()
                        .text("user_id", self.user_id.to_string())
                        .text("sticker_format", fmt)
                        .part("sticker", part);
                    self.client.post_multipart("uploadStickerFile", form).await
                }
                ref other => {
                    let body = serde_json::json!({ "user_id": self.user_id, "sticker": other.as_str(), "sticker_format": self.sticker_format });
                    self.client.post_json("uploadStickerFile", &body).await
                }
            }
        })
    }
}

#[derive(Serialize)]
struct InputStickerJson {
    sticker: String,
    format: StickerFormat,
    emoji_list: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mask_position: Option<MaskPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keywords: Option<Vec<String>>,
}

#[derive(Serialize)]
struct CreateNewStickerSetParams {
    user_id: i64,
    name: String,
    title: String,
    stickers: Vec<InputStickerJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sticker_type: Option<StickerType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    needs_repainting: Option<bool>,
}

/// Builder for the [`createNewStickerSet`](https://core.telegram.org/bots/api#createnewstickerset) method.
pub struct CreateNewStickerSet {
    client: BotClient,
    params: CreateNewStickerSetParams,
}
impl CreateNewStickerSet {
    pub(crate) fn new(
        client: BotClient,
        user_id: i64,
        name: impl Into<String>,
        title: impl Into<String>,
        stickers: Vec<InputSticker>,
    ) -> Self {
        let stickers_json = stickers
            .into_iter()
            .map(|s| InputStickerJson {
                sticker: s.sticker,
                format: s.format,
                emoji_list: s.emoji_list,
                mask_position: s.mask_position,
                keywords: s.keywords,
            })
            .collect();
        Self {
            client,
            params: CreateNewStickerSetParams {
                user_id,
                name: name.into(),
                title: title.into(),
                stickers: stickers_json,
                sticker_type: None,
                needs_repainting: None,
            },
        }
    }
    /// Sets the sticker set type (`Regular`, `Mask`, or `CustomEmoji`).
    pub fn sticker_type(mut self, k: StickerType) -> Self {
        self.params.sticker_type = Some(k);
        self
    }
    /// Enables colour replacement for custom emoji stickers when `true`.
    pub fn needs_repainting(mut self, v: bool) -> Self {
        self.params.needs_repainting = Some(v);
        self
    }
}
impl IntoFuture for CreateNewStickerSet {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("createNewStickerSet", &self.params)
                .await
        })
    }
}

#[derive(Serialize)]
struct AddStickerToSetParams {
    user_id: i64,
    name: String,
    sticker: InputStickerJson,
}

/// Builder for the [`addStickerToSet`](https://core.telegram.org/bots/api#addstickertoset) method.
pub struct AddStickerToSet {
    client: BotClient,
    params: AddStickerToSetParams,
}
impl AddStickerToSet {
    pub(crate) fn new(
        client: BotClient,
        user_id: i64,
        name: impl Into<String>,
        sticker: InputSticker,
    ) -> Self {
        Self {
            client,
            params: AddStickerToSetParams {
                user_id,
                name: name.into(),
                sticker: InputStickerJson {
                    sticker: sticker.sticker,
                    format: sticker.format,
                    emoji_list: sticker.emoji_list,
                    mask_position: sticker.mask_position,
                    keywords: sticker.keywords,
                },
            },
        }
    }
}
impl IntoFuture for AddStickerToSet {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.client.post_json("addStickerToSet", &self.params).await })
    }
}

macro_rules! simple_sticker_action {
    ($(#[$doc:meta])* $name:ident, $params_ty:ident { $($f:ident: $t:ty),+ }, $method:literal, $ret:ty) => {
        #[derive(Serialize)]
        struct $params_ty { $($f: $t),+ }

        $(#[$doc])*
        pub struct $name { client: BotClient, params: $params_ty }

        impl IntoFuture for $name {
            type Output = Result<$ret>;
            type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.client.post_json($method, &self.params).await })
            }
        }
    };
}

simple_sticker_action!(
    /// Builder for the [`setStickerPositionInSet`](https://core.telegram.org/bots/api#setstickerpositioninset) method.
    SetStickerPositionInSet,
    SetStickerPositionInSetParams { sticker: String, position: u32 },
    "setStickerPositionInSet",
    bool
);
impl SetStickerPositionInSet {
    pub(crate) fn new(client: BotClient, sticker: impl Into<String>, position: u32) -> Self {
        Self {
            client,
            params: SetStickerPositionInSetParams {
                sticker: sticker.into(),
                position,
            },
        }
    }
}

simple_sticker_action!(
    /// Builder for the [`deleteStickerFromSet`](https://core.telegram.org/bots/api#deletestickerfromset) method.
    DeleteStickerFromSet,
    DeleteStickerFromSetParams { sticker: String },
    "deleteStickerFromSet",
    bool
);
impl DeleteStickerFromSet {
    pub(crate) fn new(client: BotClient, sticker: impl Into<String>) -> Self {
        Self {
            client,
            params: DeleteStickerFromSetParams {
                sticker: sticker.into(),
            },
        }
    }
}

simple_sticker_action!(
    /// Builder for the [`setStickerSetTitle`](https://core.telegram.org/bots/api#setstickersettitle) method.
    SetStickerSetTitle,
    SetStickerSetTitleParams { name: String, title: String },
    "setStickerSetTitle",
    bool
);
impl SetStickerSetTitle {
    pub(crate) fn new(
        client: BotClient,
        name: impl Into<String>,
        title: impl Into<String>,
    ) -> Self {
        Self {
            client,
            params: SetStickerSetTitleParams {
                name: name.into(),
                title: title.into(),
            },
        }
    }
}

simple_sticker_action!(
    /// Builder for the [`deleteStickerSet`](https://core.telegram.org/bots/api#deletestickerset) method.
    DeleteStickerSet,
    DeleteStickerSetParams { name: String },
    "deleteStickerSet",
    bool
);
impl DeleteStickerSet {
    pub(crate) fn new(client: BotClient, name: impl Into<String>) -> Self {
        Self {
            client,
            params: DeleteStickerSetParams { name: name.into() },
        }
    }
}

#[derive(Serialize)]
struct SetStickerEmojiListParams {
    sticker: String,
    emoji_list: Vec<String>,
}

/// Builder for the [`setStickerEmojiList`](https://core.telegram.org/bots/api#setstickeremojilist) method.
pub struct SetStickerEmojiList {
    client: BotClient,
    params: SetStickerEmojiListParams,
}
impl SetStickerEmojiList {
    pub(crate) fn new(
        client: BotClient,
        sticker: impl Into<String>,
        emoji_list: Vec<impl Into<String>>,
    ) -> Self {
        Self {
            client,
            params: SetStickerEmojiListParams {
                sticker: sticker.into(),
                emoji_list: emoji_list.into_iter().map(Into::into).collect(),
            },
        }
    }
}
impl IntoFuture for SetStickerEmojiList {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("setStickerEmojiList", &self.params)
                .await
        })
    }
}

#[derive(Serialize)]
struct SetStickerKeywordsParams {
    sticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    keywords: Option<Vec<String>>,
}

/// Builder for the [`setStickerKeywords`](https://core.telegram.org/bots/api#setstickerkeywords) method.
pub struct SetStickerKeywords {
    client: BotClient,
    params: SetStickerKeywordsParams,
}
impl SetStickerKeywords {
    pub(crate) fn new(client: BotClient, sticker: impl Into<String>) -> Self {
        Self {
            client,
            params: SetStickerKeywordsParams {
                sticker: sticker.into(),
                keywords: None,
            },
        }
    }
    /// Sets the search keywords for this sticker (up to 20 words).
    pub fn keywords(mut self, kw: Vec<impl Into<String>>) -> Self {
        self.params.keywords = Some(kw.into_iter().map(Into::into).collect());
        self
    }
}
impl IntoFuture for SetStickerKeywords {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("setStickerKeywords", &self.params)
                .await
        })
    }
}

#[derive(Serialize)]
struct SetStickerMaskPositionParams {
    sticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    mask_position: Option<MaskPosition>,
}

/// Builder for the [`setStickerMaskPosition`](https://core.telegram.org/bots/api#setstickermaskposition) method.
pub struct SetStickerMaskPosition {
    client: BotClient,
    params: SetStickerMaskPositionParams,
}
impl SetStickerMaskPosition {
    pub(crate) fn new(client: BotClient, sticker: impl Into<String>) -> Self {
        Self {
            client,
            params: SetStickerMaskPositionParams {
                sticker: sticker.into(),
                mask_position: None,
            },
        }
    }
    /// Sets the mask position for this mask sticker.
    pub fn mask_position(mut self, mp: MaskPosition) -> Self {
        self.params.mask_position = Some(mp);
        self
    }
}
impl IntoFuture for SetStickerMaskPosition {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("setStickerMaskPosition", &self.params)
                .await
        })
    }
}

/// Builder for the [`getForumTopicIconStickers`](https://core.telegram.org/bots/api#getforumtopiciconstickers) method.
pub struct GetForumTopicIconStickers {
    client: BotClient,
}
impl GetForumTopicIconStickers {
    pub(crate) fn new(client: BotClient) -> Self {
        Self { client }
    }
}
impl IntoFuture for GetForumTopicIconStickers {
    type Output = Result<Vec<Sticker>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("getForumTopicIconStickers", &serde_json::json!({}))
                .await
        })
    }
}
