#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(non_snake_case)]
#![deny(missing_debug_implementations)]
#![deny(clippy::cognitive_complexity)]
#![cfg_attr(documenting, feature(doc_cfg))]
#![cfg_attr(
    any(releasing, not(debug_assertions)),
    deny(dead_code, unused_imports)
)]

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Item};

#[proc_macro_attribute]
pub fn theme_key(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);
    quote! {
        #[derive(::core::fmt::Debug)]
        #[derive(::core::clone::Clone)]
        #[derive(::core::marker::Copy)]
        #[derive(::core::hash::Hash)]
        #[derive(::core::cmp::PartialEq)]
        #input
    }
    .into()
}

#[proc_macro_attribute]
pub fn theme(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = {
        let item = item.clone();
        parse_macro_input!(item)
    };
    let name = ident.to_token_stream();
    let input = parse_macro_input!(item as Item);
    quote! {
        impl themer::traits::Theme for #name {}

        #[derive(::core::clone::Clone)]
        #[derive(::core::fmt::Debug)]
        #[derive(::core::cmp::PartialEq)]
        #input
    }
    .into()
}
