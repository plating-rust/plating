/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use proc_macro2::Span;
use quote::ToTokens;
use syn::{Ident, Path};

pub(crate) fn has_name(name: &Ident) -> Ident {
    Ident::new(&format! {"Has{}", name}, Span::call_site())
}

pub(crate) fn available_name(name: &Ident) -> Ident {
    Ident::new(&format!("{}Available", name), Span::call_site())
}

pub(crate) fn tag_name(name: &Ident) -> Ident {
    Ident::new(&format! {"{}Tag", name}, Span::call_site())
}

pub(crate) fn all_trait_name(name: &Ident) -> Ident {
    Ident::new(&format!("AllHave{}", name), Span::call_site())
}

pub(crate) fn definition_name(name: &Ident) -> Ident {
    Ident::new(&format!("{}Definition", name), Span::call_site())
}

pub(crate) fn definition_name_for_path(name: &mut Path) {
    let mut path_part = name.segments.pop().unwrap();
    path_part = match path_part {
        syn::punctuated::Pair::Punctuated(mut segment, p) => {
            segment.ident = definition_name(&segment.ident);
            syn::punctuated::Pair::Punctuated(segment, p)
        },
        syn::punctuated::Pair::End(mut segment) => {
            segment.ident = definition_name(&segment.ident);
            syn::punctuated::Pair::End(segment)
        },
    };

    name.segments.push(path_part.into_value());
}



// Copied from syn crate, but it's private in there
pub(crate) struct TokensOrDefault<'a, T: 'a>(pub &'a Option<T>);

impl<'a, T> ToTokens for TokensOrDefault<'a, T>
where
    T: ToTokens + Default,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self.0 {
            Some(t) => t.to_tokens(tokens),
            None => T::default().to_tokens(tokens),
        }
    }
}

pub(crate) mod r#where {
    use proc_macro2::Span;
    use syn::punctuated::Punctuated;
    use syn::{
        Ident,
        Path,
        PredicateType,
        TraitBound,
        TraitItemType,
        TypeParamBound,
        TypePath,
        WhereClause,
        WherePredicate,
    };

    pub(crate) fn new_where_clause() -> WhereClause {
        WhereClause {
            where_token: syn::token::Where {
                span: Span::call_site(),
            },
            predicates:  Punctuated::new(),
        }
    }

    pub(crate) fn add_has_bound(mut where_clause: WhereClause, ty: &TraitItemType) -> WhereClause {
        let mut has_bound = Punctuated::new();
        has_bound.push(TypeParamBound::Trait(TraitBound {
            paren_token: None,
            modifier: syn::TraitBoundModifier::None,
            lifetimes: None,
            path: super::has_name(&ty.ident).into(),
        }));
        where_clause
            .predicates
            .push(WherePredicate::Type(PredicateType {
                lifetimes:   None,
                bounded_ty:  syn::Type::Path(TypePath {
                    qself: None,
                    path:  Ident::new("Self", Span::call_site()).into(),
                }),
                colon_token: syn::token::Colon {
                    spans: [Span::call_site()],
                },
                bounds:      has_bound,
            }));
        where_clause
    }

    pub(crate) fn add_all_have_clause(
        mut where_clause: WhereClause,
        ty: &TraitItemType,
        tag_name: Path,
    ) -> WhereClause {
        let mut all_have_bound = Punctuated::new();
        all_have_bound.push(TypeParamBound::Trait(TraitBound {
            paren_token: None,
            modifier: syn::TraitBoundModifier::None,
            lifetimes: None,
            path: super::all_trait_name(&ty.ident).into(),
        }));
        where_clause
            .predicates
            .push(WherePredicate::Type(PredicateType {
                lifetimes:   None,
                bounded_ty:  syn::Type::Path(TypePath {
                    qself: None,
                    path:  tag_name,
                }),
                colon_token: syn::token::Colon {
                    spans: [Span::call_site()],
                },
                bounds:      all_have_bound,
            }));
        where_clause
    }
}
