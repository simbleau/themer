#![feature(proc_macro_quote)]
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

extern crate proc_macro;
use proc_macro::{quote, TokenStream};
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn theme_key(_attr: TokenStream, item: TokenStream) -> TokenStream {
    quote! {
        #[derive(::core::fmt::Debug)]
        #[derive(::core::clone::Clone)]
        #[derive(::core::marker::Copy)]
        #[derive(::core::hash::Hash)]
        #[derive(::core::cmp::PartialEq)]
        $item
    }
    .into()
}

#[proc_macro_attribute]
pub fn theme(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let clone = item.clone();
    let DeriveInput { ident, .. } = parse_macro_input!(clone);
    let name = ident.to_token_stream();
    quote! {
        impl themer::Theme for $name {}

        #[derive(::core::clone::Clone)]
        #[derive(::core::fmt::Debug)]
        #[derive(::core::cmp::PartialEq)]
        $item
    }
    .into()
}
