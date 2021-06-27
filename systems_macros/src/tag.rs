/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Eq, Semi};
use syn::{
    parenthesized,
    parse_quote,
    AngleBracketedGenericArguments,
    Attribute,
    GenericArgument,
    Ident,
    ImplItemType,
    Path,
    Token,
    TraitItem,
    TraitItemType,
    TypePath,
    Visibility,
};

use crate::utils::r#where::{add_all_have_clause, add_has_bound, new_where_clause};
use crate::utils::{definition_name_for_path, has_name, tag_name};

fn adjust_trait_item(mut ty: TraitItemType, tag_name: Path) -> TraitItemType {
    let mut where_clause = ty
        .generics
        .where_clause
        .clone()
        .unwrap_or_else(new_where_clause);
    where_clause = add_has_bound(where_clause, &ty);
    where_clause = add_all_have_clause(where_clause, &ty, tag_name);

    ty.generics.where_clause = Some(where_clause);
    ty
}

struct TagSystem {
    attrs: Vec<Attribute>,
    name:  Path,
}
impl TagSystem {
    fn always_available(&self) -> bool {
        !self.attrs.iter().any(|a| a.path.is_ident("cfg"))
    }
}
impl Parse for TagSystem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let name = input.parse()?;

        Ok(Self { attrs, name })
    }
}


struct TagAttrs {
    systems: Punctuated<TagSystem, Token![,]>,
}
impl Parse for TagAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![for]>()?;
        let sys_content;
        parenthesized!(sys_content in input);
        let systems = sys_content.parse_terminated(TagSystem::parse)?;


        Ok(Self { systems })
    }
}


pub fn tag(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parsed_attrs = syn::parse_macro_input!(attr as TagAttrs);
    //let systems_list = systems_list(parsed_attrs.systems.iter());

    let cfgs = if parsed_attrs
        .systems
        .iter()
        .take_while(|system| !system.always_available())
        .count()
        == parsed_attrs.systems.len()
    {
        let conditionals = (parsed_attrs
            .systems
            .iter()
            .flat_map(|system| system.attrs.iter().filter(|a| a.path.is_ident("cfg"))))
        .map(|attr| {
            let content: proc_macro2::TokenStream = attr.parse_args().expect("Parsable arguments");
            content
        });
        let conditionals2 = conditionals.clone();
        let doc_cfg_test = if parsed_attrs.systems.is_empty() {
            None
        } else {
            Some(quote! {, doc})
        };
        quote! {
            #[cfg(any(#(#conditionals),* #doc_cfg_test))]
            #[cfg_attr(doc_cfg, doc(cfg(any(#(#conditionals2),*))))]
        }
    } else {
        quote! {}
    };

    let mut input = syn::parse_macro_input!(item as syn::ItemTrait);
    let system_name = &input.ident;
    let tag_name = tag_name(&input.ident);
    let vis = input.vis.clone();

    input.items = input
        .items
        .iter()
        .map(|item| match item.clone() {
            TraitItem::Type(ty) => TraitItem::Type(adjust_trait_item(ty, tag_name.clone().into())),
            item => item,
        })
        .collect();

    input.attrs.push(syn::parse_quote! {
        #[allow(trivial_bound)]
    });

    let impls: Vec<_> = parsed_attrs
        .systems
        .iter()
        .map(|system| {
            let for_ident = &system.name;
            let cfg_for_system = system.attrs.iter().filter(|a| a.path.is_ident("cfg"));
            let input_items = input.items.clone();
            let type_defs = input_items.iter().filter_map(|i| match i {
                TraitItem::Type(ty) => {
                    let ident = &ty.ident;
                    let has_req = has_name(ident);


                    let gen = AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: ty.generics.lt_token?,
                        args: ty
                            .generics
                            .params
                            .iter()
                            .map(|a| match a {
                                syn::GenericParam::Type(t) => {
                                    GenericArgument::Type(syn::Type::Path(TypePath {
                                        qself: None,
                                        path:  t.ident.clone().into(),
                                    }))
                                },
                                syn::GenericParam::Lifetime(_) => todo!(),
                                syn::GenericParam::Const(_) => todo!(),
                            })
                            .collect(),
                        gt_token: ty.generics.gt_token?,
                    };
                    Some(ImplItemType {
                        attrs: ty.attrs.clone(),
                        vis: Visibility::Inherited,
                        defaultness: None,
                        type_token: ty.type_token,
                        ident: ty.ident.clone(),
                        generics: ty.generics.clone(),
                        eq_token: Eq {
                            spans: [Span::call_site()],
                        },
                        ty: parse_quote! {
                            <Self as #has_req>::#ident #gen
                        },
                        semi_token: Semi {
                            spans: [Span::call_site()],
                        },
                    })
                },
                _item => None,
            });
            quote!(
                #[automatically_derived]
                #(#cfg_for_system)*
                impl #system_name for #for_ident {
                    #(#type_defs)*
                }
            )
        })
        .collect();

    input.items.push(syn::parse_quote! {
        /// The corresponding 'Tag<Sys>' type
        type Tag = #tag_name;
    });

    let autogenerated_message = [
        String::from(""),
        String::from(
            "This typedef is autogenerated by the [tag](`plating_systems_macros::tag`) macro.",
        ),
    ];

    let mut tag_docs =
        vec![format! {"Typedef of the ```Tag<Sys>``` corresponding to [`{}`].", system_name}];
    add_systems_info(&mut tag_docs, &parsed_attrs.systems);
    tag_docs.extend(autogenerated_message.clone());


    let mut trait_docs = vec![String::from(""), String::from("")];
    add_systems_info(&mut trait_docs, &parsed_attrs.systems);
    trait_docs.extend(autogenerated_message);


    let tag_definition = tag_definition(tag_docs, vis, tag_name, parsed_attrs.systems);
    TokenStream::from(quote! {
        #tag_definition

        #cfgs
        #(#[doc=#trait_docs])*
        #input

        #(#impls)*
    })
}


fn tag_systems(
    mut iter: syn::punctuated::Iter<'_, TagSystem>,
) -> Vec<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    let item = iter.next();
    match item {
        Some(obj) => {
            let mut name = obj.name.clone();
            definition_name_for_path(&mut name);
            let inner = tag_systems(iter);
            let mut result = Vec::with_capacity(inner.len() * 2);

            let conditionals: Vec<_> = obj
                .attrs
                .iter()
                .filter(|a| a.path.is_ident("cfg"))
                .map(|attr| {
                    let content: proc_macro2::TokenStream =
                        attr.parse_args().expect("Parsable arguments");
                    content
                })
                .collect();

            for (cfg, system_list) in inner {
                if conditionals.is_empty() {
                    result.push((
                        quote! { #cfg },
                        quote! {
                            (#name, #system_list)
                        },
                    ));
                } else {
                    result.push((
                        quote! {#(#conditionals),*, #cfg},
                        quote! {
                            (#name, #system_list)
                        },
                    ));
                    result.push((quote! {not(#(#conditionals),*), #cfg}, system_list));
                }
            }
            result
        },
        None => {
            let mut result = Vec::new();
            result.push((quote! {}, quote! {()}));
            result
        },
    }
}


fn tag_definition(
    tag_docs: Vec<String>,
    vis: Visibility,
    name: Ident,
    systems: Punctuated<TagSystem, Token![,]>,
) -> proc_macro2::TokenStream {
    let mut tag_sys = tag_systems(systems.iter());
    if tag_sys.len() > 1 {
        tag_sys.pop(); //remove last element, it is always an empty Tag, hence
                       // not useful!
    }
    let mut first = true;
    let tags = tag_sys.iter().map(|(cfg, systems_list)| {
        let quoted_cfg = if first {
            first = false;
            quote! {#[cfg(any(doc, all(#cfg)))]}
        } else {
            quote! {#[cfg(all(not(doc), #cfg))]}
        };
        quote! {
            #[automatically_derived]
            #(#[doc=#tag_docs])*
            #quoted_cfg
            #vis type #name = Tag<#systems_list>;
        }
    });
    quote! {
        #(#tags)*
    }
}

/// Adds a list of systems to a vector of String that represents the docs.
fn add_systems_info(docs: &mut Vec<String>, systems: &Punctuated<TagSystem, Token![,]>) {
    docs.push(String::from(""));
    docs.push(String::from("This Tag consists of the following Systems:"));
    docs.extend(systems.iter().map(|sys| {
        format!{" - [{}](`{}`)", sys.name.segments.last().unwrap().into_token_stream(), sys.name.clone().into_token_stream().to_string().replace(" ", "")}
    }));
}
