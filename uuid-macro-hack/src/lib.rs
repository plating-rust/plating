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
        while(false){};
    })
}

use proc_macro2::Span;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parenthesized, parse_macro_input, Ident, Token, Type, Visibility};
struct BitflagParameter {
    visibility: Visibility,
    name: Ident,
    ty: Type,
    _underlying_type: Type,
}

impl Parse for BitflagParameter {
    fn parse(input: ParseStream) -> Result<Self> {
        let visibility: Visibility = input.parse()?;
        let name: Ident = input.parse()?;
        //input.parse::<Token![(]>()?;
        let content;
        let _inner = parenthesized!(content in input);

        let ty: Type = content.parse()?;
        content.parse::<Token![:]>()?;
        let _underlying_type: Type = content.parse()?;

        input.parse::<Token![;]>()?;

        Ok(BitflagParameter {
            visibility,
            name,
            ty,
            _underlying_type,
        })
    }
}

#[proc_macro]
pub fn bitflag_parameter(input: TokenStream) -> TokenStream {
    let BitflagParameter {
        visibility,
        name,
        ty,
        _underlying_type,
    } = parse_macro_input!(input as BitflagParameter);

    let serializer_name = Ident::new(&format!("serialize_{}", name), Span::call_site());
    let quoted_serializer_name = format!(r#"{}"#, serializer_name);

    //let deserializer_name = Ident::new(&format!("deserialize_{}", name), Span::call_site());
    //let quoted_deserializer_name = format!(r#"{}"#, serializer_name);

    let expanded = quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Serialize)]
            //#[derive(Deserialize)]
            //todo: better comparator, take X into account?
            #visibility struct #name {
                #[cfg_attr(feature = "serde", serde(serialize_with = #quoted_serializer_name))]
                //#[cfg_attr(feature = "serde", serde(deserialize_with = #quoted_deserializer_name))]
                value: #ty,
                #[cfg_attr(feature = "serde", serde(serialize_with = #quoted_serializer_name))]
                //#[cfg_attr(feature = "serde", serde(deserialize_with = #quoted_deserializer_name))]
                set: #ty,
            }
            impl Default for #name {
                fn default() -> Self {
                    #name {
                        value: #ty::empty(),
                        set: #ty::empty()
                    }
                }
            }
            impl #name {

                fn set(&mut self, b: bool, i: #ty) {
                    self.value.set(i, b);
                    self.set.insert(i)
                }
                fn unset(&mut self, i: #ty) {
                    self.value.remove(i);
                    self.set.remove(i);
                }
                fn is_set(&self, i: #ty) -> Option<bool> {
                    if self.set.contains(i) {
                        Some(self.value.contains(i))
                    } else {
                        None
                    }
                }
                /// Output Value | Orig Value | This Value | Set field
                /// 0 | 0 | 0 | 0
                /// 0 | 0 | 0 | 1
                /// 0 | 0 | 1 | 0
                /// 1 | 0 | 1 | 1
                /// 1 | 1 | 0 | 0
                /// 0 | 1 | 0 | 1
                /// 1 | 1 | 1 | 1

                fn apply_into(&self, i: #ty) -> Option<#ty> {
                    #ty::from_bits(
                        (self.value.bits() & self.set.bits()) |
                        (i.bits() & !self.set.bits())
                    )
                }
                fn apply_into_unchecked(&self, i: #ty) -> #ty {
                    unsafe {
                        #ty::from_bits_unchecked(
                            (self.value.bits() & self.set.bits()) |
                            (i.bits() & !self.set.bits())
                        )
                    }
                }

                fn merge_unchecked(&self, rhs: Self) -> Self {
                    Self {
                        value: self.apply_into_unchecked(rhs.value),
                        set: unsafe { #ty::from_bits_unchecked(self.set.bits() | rhs.set.bits()) },
                    }
                }
            }

            #[cfg(feature = "serde")]
            fn #serializer_name<S>(
                flags: &#ty,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                flags.bits().serialize(serializer)
            }
    /*
            #[cfg(feature = "serde")]
            pub fn #deserializer_name<'de, D>(deserializer: D) -> Result<#ty, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                //let raw: #underlying_type = #underlying_type::deserialize(deserializer)?;
                /*#ty::from_bits(raw).ok_or(serde::de::Error::custom(format!(
                    "Unexpected flags value {}",
                    raw
                )))*/
            }*/

        };

    TokenStream::from(expanded)
}
