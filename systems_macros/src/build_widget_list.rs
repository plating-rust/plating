/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{braced, Attribute, Ident, Token, TraitItemType};



struct WidgetListInput {
    attrs:  Vec<Attribute>,
    ident:  Ident,
    fields: Vec<TraitItemType>,
}
impl Parse for WidgetListInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;

        let ident = input.parse()?;
        input.parse::<Token![=]>()?;

        let content;
        braced!(content in input);
        let mut fields = Vec::new();

        loop {
            if content.is_empty() {
                break;
            }
            fields.push(content.parse()?);
        }

        //optional
        let _ = input.parse::<Token![;]>();


        Ok(Self {
            attrs,
            ident,
            fields,
        })
    }
}

pub fn build_widget_list(item: TokenStream) -> TokenStream {
    let WidgetListInput {
        attrs,
        ident,
        fields,
    } = syn::parse_macro_input!(item as WidgetListInput);

    let push_fields = fields.iter().map(|field| {
        quote! {
            input.items.push(
                syn::parse_quote! {
                    #field
                }
            );
        }
    });

    TokenStream::from(quote! {
        //TODO: implement option to 'skip' specific widgets
        #[automatically_derived]
        #[proc_macro_attribute]
        #(#attrs)*
        pub fn #ident(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            use quote::ToTokens;
            let mut input = syn::parse_macro_input!(item as syn::ItemTrait);

            #(#push_fields)*

            input.into_token_stream().into()
        }
    })
}
