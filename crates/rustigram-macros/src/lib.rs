#![warn(missing_docs)]
//! Procedural macros for the rustigram framework.
//!
//! - [`handler`] — marks an async function as a rustigram handler
//! - [`DialogueState`] — derive macro for FSM state enums

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Marks an async function as a rustigram handler.
///
/// Generates an impl of the `Handler` trait and a `handler_fn` wrapper.
///
/// # Example
///
/// ```rust,ignore
/// #[rustigram::handler]
/// async fn start(ctx: Context) -> BotResult<()> {
///     ctx.reply("Hello!").unwrap().await?;
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let block = &input.block;
    let _inputs = &input.sig.inputs;
    let _asyncness = &input.sig.asyncness;

    let expanded = quote! {
        #[allow(non_camel_case_types)]
        pub struct #name;

        impl rustigram_bot::handler::Handler for #name {
            fn handle<'life0, 'async_trait>(
                &'life0 self,
                ctx: rustigram_bot::Context,
            ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = rustigram_bot::BotResult<()>> + Send + 'async_trait>>
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    let ctx = ctx;
                    #block
                })
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derives a typed FSM state enum for use with `DialogueStorage`.
///
/// # Example
///
/// ```rust,ignore
/// #[derive(DialogueState, Clone)]
/// pub enum OrderState {
///     Start,
///     AwaitingName,
///     AwaitingAddress { name: String },
///     Confirming { name: String, address: String },
/// }
/// ```
#[proc_macro_derive(DialogueState)]
pub fn dialogue_state_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl #name {
            /// Returns the type name of this dialogue state as a static string.
            pub fn type_name() -> &'static str {
                stringify!(#name)
            }
        }
    };

    TokenStream::from(expanded)
}
