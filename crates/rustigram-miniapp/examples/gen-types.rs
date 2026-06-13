//! Generates TypeScript type declarations from all public `rustigram-miniapp` types.
//!
//! ```bash
//! cargo run --example gen-types --features ts
//! ```
//!
//! Output directory defaults to `../../rustigram-tma/packages/core/src/generated/`.
//! Override with the `TMA_TYPES_OUT` environment variable.

use rustigram_miniapp::types::{
    ColorScheme, ContentSafeAreaInset, InitDataChatType, SafeAreaInset, ThemeParams, WebAppChat,
    WebAppChatType, WebAppInitData, WebAppUser,
};
use ts_rs::TS;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = std::env::var("TMA_TYPES_OUT")
        .unwrap_or_else(|_| "../../rustigram-tma/packages/core/src/generated".to_owned());

    std::fs::create_dir_all(&out)?;

    // export_all_to walks the full type graph so referenced types are never missed.
    WebAppUser::export_all_to(&out)?;
    WebAppChat::export_all_to(&out)?;
    WebAppChatType::export_all_to(&out)?;
    InitDataChatType::export_all_to(&out)?;
    WebAppInitData::export_all_to(&out)?;
    ThemeParams::export_all_to(&out)?;
    ColorScheme::export_all_to(&out)?;
    SafeAreaInset::export_all_to(&out)?;
    ContentSafeAreaInset::export_all_to(&out)?;

    println!("generated → {out}");
    Ok(())
}
