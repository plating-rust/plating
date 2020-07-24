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
use syn::{parse_macro_input, DeriveInput, Lit};

#[proc_macro_derive(NativeWidget, attributes(native_handle, system))]
pub fn native(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ident = &input.ident;

    let system = input
        .attrs
        .iter()
        .find(|&attr| {
            let derive_type = &attr.path.segments[0].ident;
            derive_type == "system"
        })
        .map(|attr| match attr.parse_meta() {
            Ok(Meta::NameValue(n)) => match n.lit {
                Lit::Str(path) => Ident::new(&path.value(), input.ident.span()),
                _ => panic!(
                    "print_partial requires utf8 string. e.g. #[print_partial=\"some/path\"]"
                ),
            },
            Ok(_) => panic!("System attributes needs to be in the form of '#[system=CocoaSystem]'"),
            Err(e) => panic!(e),
        })
        .expect("NativeWidget derive requires a '#[system=CocoaSystem]' declaration on the struct");

    let (native_fn, native_mut_fn) = match &input.data {
        syn::Data::Struct(s) => {
            let handle_field = s
                .fields
                .iter()
                .find(|field| {
                    field
                        .attrs
                        .iter()
                        .find(|&attr| {
                            let derive_type = &attr.path.segments[0].ident;
                            derive_type == "native_handle"
                        })
                        .is_some()
                })
                .map(|f| &f.ident)
                .expect("Exactly one field needs the #[native_handle] attribute");

            let native_fn = quote!(&self.#handle_field);
            let native_mut_fn = quote!(&mut self.#handle_field);

            (native_fn, native_mut_fn)
        }
        syn::Data::Enum(_) => panic!("NativeWidget not supported for enums"),
        syn::Data::Union(_) => panic!("NativeWidget not supported for unions"),
    };

    let generated = quote! {
        impl #impl_generics NativeWidget<#system> for #ident #ty_generics #where_clause {
            fn native(&self) -> &<#system as System>::InternalHandle {
                #native_fn
            }
            unsafe fn native_mut(&mut self) -> &mut <#system as System>::InternalHandle {
                #native_mut_fn
            }
        }
    };
    proc_macro::TokenStream::from(generated)
}

#[proc_macro_derive(Identifiable, attributes(id))]
pub fn identifiable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ident = &input.ident;

    let function_content = match &input.data {
        syn::Data::Struct(s) => {
            match s.fields.iter().find(|field| {
                field
                    .attrs
                    .iter()
                    .find(|&attr| {
                        let derive_type = &attr.path.segments[0].ident;
                        derive_type == "id"
                    })
                    .is_some()
            }) {
                Some(id_field) => {
                    let field_ident = &id_field.ident;
                    quote! {
                        &self.#field_ident.as_str()
                    }
                }
                None => panic!("No id field provided for Identifiable derive"),
            }
        }
        syn::Data::Enum(e) => {
            let variants = e.variants.iter().map(|v| {
                let ident = &v.ident;
                quote! {
                    Self::#ident(val) => val.id()
                }
            });

            quote! {
                match self {
                    #(#variants),*
                }
            }
        }
        syn::Data::Union(_) => panic!("union not supported, just yet :("),
    };

    let generated = quote! {
        impl #impl_generics Identity for #ident #ty_generics #where_clause {
            fn id(&self) -> &str {
                #function_content
            }
        }
    };

    proc_macro::TokenStream::from(generated)
}

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
use syn::{parenthesized, Ident, Meta, Token, Type, Visibility};
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
