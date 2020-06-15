/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Empty dummy docs!

extern crate proc_macro;
extern crate proc_quote;
extern crate uuid;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use proc_quote::quote;

///
#[proc_macro_hack]
pub fn uuid(_item: TokenStream) -> TokenStream {
    let generated = uuid::Uuid::new_v4().to_string();
    TokenStream::from(quote! {
        #generated
    })
}

///
#[proc_macro_hack]
pub fn noop(_item: TokenStream) -> TokenStream {
    TokenStream::from(quote! {
        while(false){}; //todo: find better way for noop?
    })
}
